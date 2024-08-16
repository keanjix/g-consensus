use std::{collections::HashSet, sync::{Arc, Mutex}};

use aptos_crypto::HashValue;

#[derive(Clone)]
pub struct VTxnPoolState {
    inner: Arc<Mutex<PoolStateInner>>,
}

#[derive(Default)]
pub struct PoolStateInner {
}

pub enum TransactionFilter {
    PendingTxnHashSet(HashSet<HashValue>),
}