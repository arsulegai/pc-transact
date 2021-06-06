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

use crate::pc_error::PCError;
use transact::protocol::receipt::{StateChange, TransactionReceipt, TransactionResult};
use transact::scheduler::BatchExecutionResult;
use transact::state::merkle::MerkleState;
use transact::state::StateChange as ChangeSet;
use transact::state::Write;

pub(crate) fn commit_state(
    state: &MerkleState,
    cur_root: &str,
    mut result: BatchExecutionResult,
) -> Result<String, PCError> {
    // Extract the transaction result
    let txn_result = match result.receipts.pop() {
        Some(result) => result.transaction_result,
        None => return Err(PCError::from("Unable to find the result")),
    };

    // Extract the transaction execution result
    let mut state_change_vector = match txn_result {
        TransactionResult::Valid{state_changes, .. } => state_changes,
        TransactionResult::Invalid{error_message, ..} => {
            return Err(PCError::from(error_message))
        }
    };

    let state_changes = match state_change_vector.pop() {
        Some(state_change) => state_change,
        None => {
            return Err(PCError::from(
                "Unable to find the receipt for the transaction",
            ))
        }
    };

    let (key, value) = match state_changes {
        StateChange::Set { key, value } => (key, value),
        _ => return Err(PCError::from("Found a delete operation")),
    };

    let changeset = ChangeSet::Set { key, value };

    let commit_result = match state.commit(&cur_root.to_string(), &[changeset]) {
        Ok(state_id) => state_id,
        Err(err) => return Err(PCError::from(err.to_string())),
    };

    Ok(commit_result)
}
