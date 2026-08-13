use ml_dsa::{Generate, Keypair, MlDsa65, SigningKey, VerifyingKey};

pub struct KeyPair {
    pub signing_key: SigningKey<MlDsa65>,
    pub verifying_key: VerifyingKey<MlDsa65>,
}

pub fn generate() -> KeyPair {
    let signing_key = SigningKey::<MlDsa65>::generate();
    let verifying_key = signing_key.verifying_key();
    KeyPair {
        signing_key,
        verifying_key,
    }
}