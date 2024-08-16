use aptos_config::network_id::{NetworkId, PeerNetworkId};

pub mod application;

pub mod protocols {
    pub mod network {
        use std::fmt::Display;



        #[derive(Debug, thiserror::Error)]
        pub struct RpcError;

        impl Display for RpcError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                todo!()
            }
        }

        pub struct Event<T> {
            t: T,
        }
    }

    pub mod wire {
        pub mod handshake {
            pub mod v1 {
                pub struct ProtocolId;
            }
        }
    }
}

pub struct ProtocolId;

impl ProtocolId {
    pub fn to_bytes<T: serde::Serialize>(&self, t: T) -> anyhow::Result<Vec<u8>> {
        todo!()
    }
}

pub mod transport {
    pub struct ConnectionMetadata;
}