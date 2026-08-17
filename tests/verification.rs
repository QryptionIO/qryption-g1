use qryption_g1::authorization::Authorization;
use qryption_g1::keys;
use qryption_g1::signing::sign_authorization;
use qryption_g1::verification::{verify_authorization, VerificationResult};

fn sample_authorization() -> Authorization {
    Authorization {
        operation: "TRANSFER".to_string(),
        amount: "1000".to_string(),
        currency: "EUR".to_string(),
        destination: "ACCOUNT_123".to_string(),
        nonce: 1,
        expires_at: 1_234_567_890,
        context: "qryption-poc".to_string(),
    }
}

#[test]
fn happy_path_generate_sign_verify() {
    let kp = keys::generate();
    let auth = sample_authorization();
    let signature = sign_authorization(&kp.signing_key, &auth);

    let result = verify_authorization(&kp.verifying_key, &auth, &signature);

    assert_eq!(result, VerificationResult::Valid);
}