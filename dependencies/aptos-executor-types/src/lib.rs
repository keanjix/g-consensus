// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;
use aptos_crypto::hash::{HashValue, TransactionAccumulatorHasher};
use aptos_types::epoch_state::EpochState;
use aptos_types::transaction::{Transaction, TransactionStatus, block_epilogue::BlockEndInfo};
use aptos_types::contract_event::ContractEvent;
use aptos_types::proof::AccumulatorExtensionProof;
use thiserror::Error;


#[derive(Error, Debug, PartialEq, Eq,)]
/// Different reasons for proposal rejection
pub enum ExecutorError {
}

pub type Version = u64;

impl ExecutorError {
    pub fn internal_err<E: Display>(e: E) -> Self {
        todo!()
    }
}

pub type ExecutorResult<T> = Result<T, ExecutorError>;

pub struct ExecutableBlock {
    pub block_id: HashValue,
}

#[derive(Clone, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")] // cannot use tag = "type" as nested enums cannot work, and bcs doesn't support it
pub enum BlockGasLimitType {
    NoLimit,
    Limit(u64),
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct BlockExecutorConfigFromOnchain {
    pub block_gas_limit_type: BlockGasLimitType,
}

#[derive(Default)]
pub struct StateCheckpointOutput {
    block_end_info: Option<BlockEndInfo>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LedgerInfoWithSignatures {
}

pub trait BlockExecutorTrait: Send + Sync {
    /// Get the latest committed block id
    fn committed_block_id(&self) -> HashValue;

    /// Reset the internal state including cache with newly fetched latest committed block from storage.
    fn reset(&self) -> anyhow::Result<()>;

    /// Executes a block - TBD, this API will be removed in favor of `execute_and_state_checkpoint`, followed
    /// by `ledger_update` once we have ledger update as a separate pipeline phase.
    fn execute_block(
        &self,
        block: ExecutableBlock,
        parent_block_id: HashValue,
        onchain_config: BlockExecutorConfigFromOnchain,
    ) -> ExecutorResult<StateComputeResult> {
        let block_id = block.block_id;
        let state_checkpoint_output =
            self.execute_and_state_checkpoint(block, parent_block_id, onchain_config)?;
        self.ledger_update(block_id, parent_block_id, state_checkpoint_output)
    }

    /// Executes a block and returns the state checkpoint output.
    fn execute_and_state_checkpoint(
        &self,
        block: ExecutableBlock,
        parent_block_id: HashValue,
        onchain_config: BlockExecutorConfigFromOnchain,
    ) -> ExecutorResult<StateCheckpointOutput>;

    fn ledger_update(
        &self,
        block_id: HashValue,
        parent_block_id: HashValue,
        state_checkpoint_output: StateCheckpointOutput,
    ) -> ExecutorResult<StateComputeResult>;

    /// Saves eligible blocks to persistent storage.
    /// If we have multiple blocks and not all of them have signatures, we may send them to storage
    /// in a few batches. For example, if we have
    /// ```text
    /// A <- B <- C <- D <- E
    /// ```
    /// and only `C` and `E` have signatures, we will send `A`, `B` and `C` in the first batch,
    /// then `D` and `E` later in the another batch.
    /// Commits a block and all its ancestors in a batch manner.
    fn commit_blocks(
        &self,
        block_ids: Vec<HashValue>,
        ledger_info_with_sigs: LedgerInfoWithSignatures,
    ) -> ExecutorResult<()>;

    /// Finishes the block executor by releasing memory held by inner data structures(SMT).
    fn finish(&self);
}

#[derive(Debug, Default, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct StateComputeResult {
        /// transaction accumulator root hash is identified as `state_id` in Consensus.
        root_hash: HashValue,
        /// Represents the roots of all the full subtrees from left to right in this accumulator
        /// after the execution. For details, please see [`InMemoryAccumulator`](aptos_types::proof::accumulator::InMemoryAccumulator).
        frozen_subtree_roots: Vec<HashValue>,
    
        /// The frozen subtrees roots of the parent block,
        parent_frozen_subtree_roots: Vec<HashValue>,
    
        /// The number of leaves of the transaction accumulator after executing a proposed block.
        /// This state must be persisted to ensure that on restart that the version is calculated correctly.
        num_leaves: u64,
    
        /// The number of leaves after executing the parent block,
        parent_num_leaves: u64,
    
        /// If set, this is the new epoch info that should be changed to if this block is committed.
        epoch_state: Option<EpochState>,
        /// The compute status (success/failure) of the given payload. The specific details are opaque
        /// for StateMachineReplication, which is merely passing it between StateComputer and
        /// PayloadClient.
        ///
        /// Here, only input transactions statuses are kept, and in their order.
        /// Input includes BlockMetadata, but doesn't include StateCheckpoint/BlockEpilogue
        compute_status_for_input_txns: Vec<TransactionStatus>,
    
        /// The transaction info hashes of all success txns.
        transaction_info_hashes: Vec<HashValue>,
    
        subscribable_events: Vec<ContractEvent>,
    
        block_end_info: Option<BlockEndInfo>,
}

impl StateComputeResult {
    pub fn new(
        root_hash: HashValue,
        frozen_subtree_roots: Vec<HashValue>,
        num_leaves: u64,
        parent_frozen_subtree_roots: Vec<HashValue>,
        parent_num_leaves: u64,
        epoch_state: Option<EpochState>,
        compute_status_for_input_txns: Vec<TransactionStatus>,
        transaction_info_hashes: Vec<HashValue>,
        subscribable_events: Vec<ContractEvent>,
        block_end_info: Option<BlockEndInfo>,
    ) -> Self {
        Self {
            root_hash,
            frozen_subtree_roots,
            num_leaves,
            parent_frozen_subtree_roots,
            parent_num_leaves,
            epoch_state,
            compute_status_for_input_txns,
            transaction_info_hashes,
            subscribable_events,
            block_end_info,
        }
    }

    pub fn new_dummy() -> Self {
        todo!()
    }

    pub fn extension_proof(&self) -> AccumulatorExtensionProof<TransactionAccumulatorHasher> {
        todo!()
    }

    /// generate a new dummy state compute result with a given root hash.
    /// this function is used in RandomComputeResultStateComputer to assert that the compute
    /// function is really called.
    pub fn new_dummy_with_root_hash(root_hash: HashValue) -> Self {
        Self {
            root_hash,
            frozen_subtree_roots: vec![],
            num_leaves: 0,
            parent_frozen_subtree_roots: vec![],
            parent_num_leaves: 0,
            epoch_state: None,
            compute_status_for_input_txns: vec![],
            transaction_info_hashes: vec![],
            subscribable_events: vec![],
            block_end_info: None,
        }
    }

    pub fn new_dummy_with_num_txns(num_txns: usize) -> Self {
        Self {
            root_hash: HashValue::zero(),
            frozen_subtree_roots: vec![],
            num_leaves: 0,
            parent_frozen_subtree_roots: vec![],
            parent_num_leaves: 0,
            epoch_state: None,
            compute_status_for_input_txns: vec![
                todo!()
            ],
            transaction_info_hashes: vec![],
            subscribable_events: vec![],
            block_end_info: None,
        }
    }

    /// generate a new dummy state compute result with ACCUMULATOR_PLACEHOLDER_HASH as the root hash.
    /// this function is used in ordering_state_computer as a dummy state compute result,
    /// where the real compute result is generated after ordering_state_computer.commit pushes
    /// the blocks and the finality proof to the execution phase.
    

    pub fn version(&self) -> Version {
        todo!()
    }

    pub fn root_hash(&self) -> HashValue {
        self.root_hash
    }

    pub fn compute_status_for_input_txns(&self) -> &Vec<TransactionStatus> {
        &self.compute_status_for_input_txns
    }

    pub fn transactions_to_commit_len(&self) -> usize {
        self.compute_status_for_input_txns()
            .iter()
            .filter(|status| matches!(status, TransactionStatus::Keep(_)))
            .count()
            // StateCheckpoint/BlockEpilogue is added if there is no reconfiguration
            + (if self.has_reconfiguration() { 0 } else { 1 })
    }

    /// On top of input transactions (which contain BlockMetadata and Validator txns),
    /// filter out those that should be committed, and add StateCheckpoint/BlockEpilogue if needed.
    pub fn transactions_to_commit(
        &self,
        input_txns: Vec<Transaction>,
        block_id: HashValue,
    ) -> Vec<Transaction> {
        todo!()
    }

    pub fn epoch_state(&self) -> &Option<EpochState> {
        &self.epoch_state
    }

    // pub fn extension_proof(&self) -> AccumulatorExtensionProof<TransactionAccumulatorHasher> {
    //     todo!()
    // }

    pub fn transaction_info_hashes(&self) -> &Vec<HashValue> {
        &self.transaction_info_hashes
    }

    pub fn num_leaves(&self) -> u64 {
        self.num_leaves
    }

    pub fn frozen_subtree_roots(&self) -> &Vec<HashValue> {
        &self.frozen_subtree_roots
    }

    pub fn parent_num_leaves(&self) -> u64 {
        self.parent_num_leaves
    }

    pub fn parent_frozen_subtree_roots(&self) -> &Vec<HashValue> {
        &self.parent_frozen_subtree_roots
    }

    pub fn has_reconfiguration(&self) -> bool {
        self.epoch_state.is_some()
    }

    pub fn subscribable_events(&self) -> &[ContractEvent] {
        &self.subscribable_events
    }
}

impl From<anyhow::Error> for ExecutorError {
    fn from(error: anyhow::Error) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
}
