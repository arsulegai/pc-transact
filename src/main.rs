// Copyright 2019 Walmart Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate regex;
#[macro_use]
extern crate log;

use crate::batcher::Batcher;
use crate::pc_state::PCState;
use crate::scheduler::schedule;
use crate::state_handler::commit_state;
use crate::transaction::transaction_payload;
use transact::signing::ursa::UrsaSigner;
use transact::state::merkle::{MerkleRadixTree, MerkleState};

mod batcher;
mod handler;
mod input;
mod payload;
mod pc_error;
mod pc_state;
mod proto;
mod scheduler;
mod state_handler;
mod transaction;

/// The following application is an example of standalone application
/// running, that makes use of Hyperledger Transact. It utilizes
/// the crate generated from
/// ```https://github.com/arsulegai/produce-consume```.
///
/// This example accepts user input in the form of
/// Command <item> <quantity>
/// Where Command is either PRODUCE or CONSUME
/// <item> is the identifier for the item
/// <quantity> is a positive integer, the number of items
/// produced/consumed.
fn main() {
    // Initialize current state of the `produce-consume`
    // Start the executor
    let cur_state = match PCState::new() {
        Ok(initialized) => initialized,
        Err(err) => panic!("Error: {:?}", err),
    };

    // Create the state store from the KV database
    let statestore = MerkleState::new(cur_state.get_db().clone());

    let db = match MerkleRadixTree::new(cur_state.get_db().clone(), None) {
        Ok(database) => database,
        Err(err) => panic!("Error: {:?}", err),
    };
    let mut state_root = db.get_merkle_root();

    // Generate a signer
    let signer = UrsaSigner::new();

    // Get the payload signed by the signer
    let batcher_obj = Batcher::new(Box::from(signer));

    loop {
        // Get the payload from the user
        let (usr_payload, inputs, outputs) = match transaction_payload() {
            Ok(valid) => valid,
            Err(err) => panic!("Error: {:?}", err),
        };

        let batch = match batcher_obj.single_txn(&usr_payload, inputs, outputs) {
            Ok(batch) => batch,
            Err(err) => panic!("Error: {:?}", err),
        };

        let result = match schedule(&cur_state, batch, &state_root) {
            Ok(success) => success,
            Err(err) => panic!("Failed {:?}", err),
        };

        state_root = match commit_state(&statestore, &state_root, result) {
            Ok(new_state_root) => new_state_root,
            Err(err) => panic!("Failed {:?}", err),
        };
    }
}
