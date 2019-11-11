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
use crate::pc_state::PCState;
use transact::protocol::batch::BatchPair;
use transact::scheduler::serial::SerialScheduler;
use transact::scheduler::{BatchExecutionResult, Scheduler};

pub(crate) fn schedule(
    state: &PCState,
    batch: BatchPair,
    state_root: &str,
) -> Result<BatchExecutionResult, PCError> {
    let context_manager = state.get_context_manager();
    let mut scheduler =
        match SerialScheduler::new(Box::new(context_manager), state_root.to_string()) {
            Ok(created_scheduler) => created_scheduler,
            Err(err) => return Err(PCError::from(err.to_string())),
        };

    let (sender, receiver) = std::sync::mpsc::channel();

    match scheduler.set_result_callback(Box::new(move |batch_result| {
        sender.send(batch_result).expect("Failed!!!")
    })) {
        Ok(_) => info!("Successfully registered the callback"),
        Err(err) => return Err(PCError::from(err.to_string())),
    };

    match scheduler.add_batch(batch) {
        Ok(_) => info!("Successfully added the batch"),
        Err(err) => return Err(PCError::from(err.to_string())),
    };

    match scheduler.finalize() {
        Ok(_) => info!("Successfully finalized the scheduler"),
        Err(err) => return Err(PCError::from(err.to_string())),
    };

    // Run the scheduler
    let task_iterator = match scheduler.take_task_iterator() {
        Ok(iterator) => iterator,
        Err(err) => return Err(PCError::from(err.to_string())),
    };

    let executor = state.get_executor();
    let notifier = match scheduler.new_notifier() {
        Ok(success) => success,
        Err(err) => return Err(PCError::from(err.to_string())),
    };
    match executor.execute(task_iterator, notifier) {
        Ok(_) => info!("Successfully executed the scheduled tasks"),
        Err(err) => return Err(PCError::from(err.to_string())),
    };

    // Receive the result from the execution
    let result = match receiver.recv() {
        Ok(success) => success,
        Err(err) => return Err(PCError::from(err.to_string())),
    };
    match result {
        Some(found_result) => Ok(found_result),
        None => Err(PCError::from("Unexpected error, no result found")),
    }
}
