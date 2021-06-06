// Copyright 2019 Arun S M
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

use crate::handler::ProduceConsumeHandler;
use crate::pc_error::PCError;
use transact::context::manager::sync::ContextManager;
use transact::database::btree::BTreeDatabase;
use transact::database::Database;
use transact::execution::adapter::static_adapter::StaticExecutionAdapter;
use transact::execution::executor::Executor;
use transact::sawtooth::SawtoothToTransactHandlerAdapter;
use transact::state::merkle::{MerkleState, INDEXES};

pub(crate) struct PCState {
    db: Box<dyn Database>,
    context_manager: ContextManager,
    executor: Executor,
}

impl PCState {
    pub(crate) fn new() -> Result<PCState, PCError> {
        // Prepare the database to store the commits
        let db = Box::new(BTreeDatabase::new(&INDEXES));
        let context_manager = ContextManager::new(Box::new(MerkleState::new(db.clone())));
        let mut executor = Executor::new(vec![Box::new(
            match StaticExecutionAdapter::new_adapter(
                vec![Box::new(SawtoothToTransactHandlerAdapter::new(
                    ProduceConsumeHandler::new(),
                ))],
                context_manager.clone(),
            ) {
                Ok(execution_adapter) => execution_adapter,
                Err(err) => return Err(PCError::from(err.to_string())),
            },
        )]);
        match executor.start() {
            Ok(_) => info!("Execution unit started"),
            Err(err) => return Err(PCError::from(err.to_string())),
        }

        Ok(PCState {
            db,
            context_manager,
            executor,
        })
    }

    pub(crate) fn get_executor(&self) -> &Executor {
        &self.executor
    }

    pub(crate) fn get_db(&self) -> Box<dyn Database> {
        self.db.clone()
    }

    pub(crate) fn get_context_manager(&self) -> ContextManager {
        self.context_manager.clone()
    }
}
