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

use crate::handler::{PRODUCE_CONSUME, VERSION};
use crate::pc_error::PCError;
use rand::Rng;
use transact::protocol::batch::{BatchBuilder, BatchPair};
use transact::protocol::transaction::{HashMethod, TransactionBuilder, TransactionPair};
use transact::signing::Signer;

pub(crate) struct Batcher {
    signer: Box<dyn Signer>,
}

impl Batcher {
    pub(crate) fn new(signer: Box<dyn Signer>) -> Batcher {
        Batcher { signer }
    }

    pub(crate) fn single_txn(
        &self,
        raw_bytes: &[u8],
        inputs: Vec<Vec<u8>>,
        outputs: Vec<Vec<u8>>,
    ) -> Result<BatchPair, PCError> {
        let txn = self.get_txn(raw_bytes, inputs, outputs)?;
        self.get_batch(txn)
    }

    pub(crate) fn get_txn(
        &self,
        raw_bytes: &[u8],
        inputs: Vec<Vec<u8>>,
        outputs: Vec<Vec<u8>>,
    ) -> Result<TransactionPair, PCError> {
//        let nonce = rand::thread_rng()
//            .gen_iter::<u8>()
//            .take(64)
//            .collect::<Vec<u8>>();
        let nonce = b"arun_nonce".to_vec();
        match TransactionBuilder::new()
            .with_batcher_public_key(self.signer.public_key().to_vec())
            .with_family_name(PRODUCE_CONSUME.to_string())
            .with_family_version(VERSION.to_string())
            .with_inputs(inputs)
            .with_outputs(outputs)
            .with_nonce(nonce)
            .with_payload_hash_method(HashMethod::SHA512)
            .with_payload(raw_bytes.to_vec())
            .build_pair(&*self.signer)
        {
            Ok(txn) => Ok(txn),
            Err(err) => Err(PCError::from(format!("Txn Builder: {}", err.to_string()))),
        }
    }

    pub(crate) fn get_batch(&self, transaction: TransactionPair) -> Result<BatchPair, PCError> {
        // Transactions from transaction pair
        // TODO: Currently works with only one transaction
        let (txn, _header) = transaction.take();
        let txns = vec![txn];
        match BatchBuilder::new()
            .with_transactions(txns)
            .build_pair(&*self.signer)
        {
            Ok(batch) => Ok(batch),
            Err(err) => Err(PCError::from(format!("BatchBuilder: {}", err.to_string()))),
        }
    }
}
