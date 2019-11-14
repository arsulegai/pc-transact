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
use std::io;

pub(crate) fn usr_input() -> Result<String, PCError> {
    let mut line = String::new();
    println!("Enter your command: ");
    match io::stdin().read_line(&mut line) {
        Ok(_) => info!("Read the line!"),
        Err(err) => return Err(PCError::from(err.to_string())),
    };
    Ok(line.clone())
}
