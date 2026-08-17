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
fn tampered_payload_is_rejected() {
    let kp = keys::generate();
    let original = sample_authorization();
    let signature = sign_authorization(&kp.signing_key, &original);

    let result = verify_authorization(&kp.verifying_key, &original, &signature);
    assert_eq!(result, VerificationResult::Valid);

    let mut tampered = original.clone();
    tampered.amount = "9000".to_string();

    let tampered_result = verify_authorization(&kp.verifying_key, &tampered, &signature);
    assert_eq!(tampered_result, VerificationResult::Invalid);
}