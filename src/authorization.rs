use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Authorization {
    pub operation: String,
    pub amount: String,
    pub currency: String,
    pub destination: String,
    pub nonce: u64,
    pub expires_at: u64,
    pub context: String,
}

impl Authorization {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("serialization should not fail")
    }
}