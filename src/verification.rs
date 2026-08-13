use ml_dsa::{MlDsa65, Signature, VerifyingKey};
use signature::Verifier;

use crate::authorization::Authorization;

#[derive(Debug, PartialEq, Eq)]
pub enum VerificationResult {
    Valid,
    Invalid,
}

pub fn verify_authorization(
    verifying_key: &VerifyingKey<MlDsa65>,
    authorization: &Authorization,
    signature: &Signature<MlDsa65>,
) -> VerificationResult {
    let bytes = authorization.canonical_bytes();
    match verifying_key.verify(&bytes, signature) {
        Ok(()) => VerificationResult::Valid,
        Err(_) => VerificationResult::Invalid,
    }
}