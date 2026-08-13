use ml_dsa::{MlDsa65, Signature, SigningKey};
use signature::Signer;

use crate::authorization::Authorization;

pub fn sign_authorization(
    signing_key: &SigningKey<MlDsa65>,
    authorization: &Authorization,
) -> Signature<MlDsa65> {
    let bytes = authorization.canonical_bytes();
    signing_key.sign(&bytes)
}