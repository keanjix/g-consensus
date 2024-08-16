use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct OnChainConfigPayload<P> {
    epoch: u64,
    provider: Arc<P>,
}

impl<P> OnChainConfigPayload<P> {
    pub fn new(epoch: u64, provider: P) -> Self {
        Self {
            epoch,
            provider: Arc::new(provider),
        }
    }

    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    pub fn get<T>(&self) -> Result<T, anyhow::Error> {
        todo!()
    }
}

pub struct DbBackedOnChainConfig {
    pub on_chain_configs: OnChainConfigPayload<aptos_types::on_chain_config::ValidatorSet>,
}

pub struct ReconfigNotificationListener<T> {
    pub network_client: T,
}

impl <T> ReconfigNotificationListener<T> {
    pub async fn next(&mut self) -> Result<DbBackedOnChainConfig, ()> {
        unimplemented!()
    }
}