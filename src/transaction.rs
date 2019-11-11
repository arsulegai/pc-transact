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

use crate::input::usr_input;
use crate::payload::ProduceConsumePayload;
use crate::pc_error::PCError;
use crate::proto::action::Action;
use protobuf::Message;
use sha2::{Digest, Sha512};

/// This method returns the payload if any from the user.
/// If user interrupts the execution, it's returned here.
pub(crate) fn transaction_payload() -> Result<(Vec<u8>, Vec<Vec<u8>>, Vec<Vec<u8>>), PCError> {
    // Read input from the user, compose into the payload bytes
    // Send it back to the caller.
    let usr_input = match usr_input() {
        Ok(read_line) => read_line,
        Err(err) => return Err(err),
    };
    let pc_payload = match ProduceConsumePayload::new(&usr_input) {
        Ok(read_payload) => read_payload,
        Err(err) => return Err(err),
    };

    // Get the raw bytes of the payload that can be sent to the handler
    let mut payload = Action::new();
    payload.set_command(pc_payload.get_command());
    payload.set_identifier(pc_payload.get_identifier());
    payload.set_quantity(pc_payload.get_quantity());
    let payload_bytes = match payload.write_to_bytes() {
        Ok(bytes) => bytes,
        Err(err) => return Err(PCError::from(err.to_string())),
    };
    let input = compute_address(&pc_payload.get_identifier());
    let output = compute_address(&pc_payload.get_identifier());
    Ok((payload_bytes, vec![input], vec![output]))
}

fn compute_address(identifier: &str) -> Vec<u8> {
    let mut sha = Sha512::default();
    sha.input(identifier);
    hex::decode("ce2292".to_owned() + &hex::encode(&sha.result())[..64]).unwrap()
}
