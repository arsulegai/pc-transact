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
use crate::proto::action::Action;
use crate::proto::action::Action_Command;
use regex::Regex;
use std::str::FromStr;
use transact::handler::ApplyError;

#[derive(Debug)]
pub(crate) struct ProduceConsumePayload {
    command: Action_Command,
    identifier: String,
    quantity: i32,
}

impl ProduceConsumePayload {
    pub(crate) fn new(text: &str) -> Result<ProduceConsumePayload, PCError> {
        // match the command line arguments against the valid pattern
        // and prepare a payload that can be serialized.
        let re = match Regex::new(r#"^(PRODUCE|CONSUME)[ ][[:word:]]+[ ][[:word:]]+"#) {
            Ok(match_expression) => match_expression,
            Err(err) => return Err(PCError::from(err.to_string())),
        };
        if !re.is_match(text) {
            return Err(PCError::from(
                "Please input \"[PRODUCE|CONSUME] <identifier> <quantity>\"",
            ));
        }

        // Get the parameters
        let words: Vec<&str> = text.split(' ').collect();
        // A pattern is matched, it's expected to have the word 0
        let action_command = match words.get(0).unwrap().clone() {
            "PRODUCE" => Action_Command::PRODUCE,
            "CONSUME" => Action_Command::CONSUME,
            _ => panic!("Unexpected command action found"),
        };

        // A pattern is matched, there should be a identifier
        let identifier = words.get(1).unwrap().clone();

        // A pattern is matches, there should be a quantity
        let mut quantity_string = words.get(2).unwrap().to_string();

        info!("Quantity is {}", quantity_string);
        quantity_string.truncate(quantity_string.len() - 1);

        let quantity = match i32::from_str(&quantity_string) {
            Ok(value_read) => value_read,
            Err(err) => return Err(PCError::from(err.to_string())),
        };

        // Create a payload structure with the information parsed
        Ok(ProduceConsumePayload {
            command: action_command,
            identifier: identifier.to_string(),
            quantity,
        })
    }

    /// Convert from bytes to
    pub(crate) fn from(raw_bytes: &[u8]) -> Result<ProduceConsumePayload, ApplyError> {
        info!("Payload in raw is {:?}", &raw_bytes);
        let parsed_payload: Action = match parse_from(&raw_bytes) {
            Ok(result) => result,
            Err(e) => return Err(e),
        };
        Ok(ProduceConsumePayload {
            command: parsed_payload.get_command(),
            identifier: parsed_payload.get_identifier().to_string(),
            quantity: parsed_payload.get_quantity(),
        })
    }

    pub(crate) fn get_command(&self) -> Action_Command {
        return self.command;
    }

    pub(crate) fn get_identifier(&self) -> String {
        return self.identifier.clone();
    }

    pub(crate) fn get_quantity(&self) -> i32 {
        return self.quantity;
    }
}

fn parse_from<T>(data: &[u8]) -> Result<T, ApplyError>
where
    T: protobuf::Message,
{
    protobuf::parse_from_bytes(&data).map_err(|err| {
        warn!("Invalid error: Failed to parse the payload: {:?}", err);
        ApplyError::InvalidTransaction(format!("Failed to unmarshal payload: {:?}", err))
    })
}
