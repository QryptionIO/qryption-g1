use std::collections::HashSet;

use ml_dsa::{MlDsa65, Signature, VerifyingKey};

use crate::authorization::Authorization;
use crate::verification::{verify_authorization, VerificationResult};

#[derive(Debug, PartialEq, Eq)]
pub enum AuthorizationDecision {
    Authorized,
    RejectedInvalidSignature,
    RejectedReplay,
}

/// Lleva el registro de qué nonces ya se han consumido.
/// Esto NO es criptografía: es responsabilidad del protocolo, no de la firma.
#[derive(Debug, Default)]
pub struct AuthorizationLedger {
    used_nonces: HashSet<u64>,
}

impl AuthorizationLedger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn authorize(
        &mut self,
        verifying_key: &VerifyingKey<MlDsa65>,
        authorization: &Authorization,
        signature: &Signature<MlDsa65>,
    ) -> AuthorizationDecision {
        let verification = verify_authorization(verifying_key, authorization, signature);
        if verification != VerificationResult::Valid {
            return AuthorizationDecision::RejectedInvalidSignature;
        }

        if self.used_nonces.contains(&authorization.nonce) {
            return AuthorizationDecision::RejectedReplay;
        }

        self.used_nonces.insert(authorization.nonce);
        AuthorizationDecision::Authorized
    }
}