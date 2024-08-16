use std::collections::BTreeMap;
use anyhow::Result;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct TransactionSummary;
struct TransactionInProgress;
struct RejectedTransactionSummary;

#[derive(Debug)]
struct SignedTransaction;

pub enum QuorumStoreRequest {
    GetBatchRequest(
        // max batch size
        u64,
        // max byte size
        u64,
        // return non full
        bool,
        // transactions to exclude from the requested batch
        BTreeMap<TransactionSummary, TransactionInProgress>,
        // callback to respond to
        oneshot::Sender<Result<QuorumStoreResponse>>,
    ),
    // TODO: Do we use it in the real QS as well?
    /// Notifications about *rejected* committed txns.
    RejectNotification(
        // rejected transactions from consensus
        Vec<RejectedTransactionSummary>,
        // callback to respond to
        oneshot::Sender<Result<QuorumStoreResponse>>,
    ),
}

#[derive(Debug)]
pub enum QuorumStoreResponse {
    /// Block to submit to consensus
    GetBatchResponse(Vec<SignedTransaction>),
    CommitResponse(),
}

