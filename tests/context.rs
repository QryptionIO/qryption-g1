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
        nonce: 7,
        expires_at: 1_234_567_890,
        context: "qryption-poc".to_string(),
    }
}

#[test]
fn signature_bound_to_original_context_only() {
    let kp = keys::generate();
    let original = sample_authorization();
    let signature = sign_authorization(&kp.signing_key, &original);

    let result = verify_authorization(&kp.verifying_key, &original, &signature);
    assert_eq!(result, VerificationResult::Valid);

    let mut wrong_context = original.clone();
    wrong_context.context = "otro-sistema".to_string();

    let wrong_context_result =
        verify_authorization(&kp.verifying_key, &wrong_context, &signature);
    assert_eq!(wrong_context_result, VerificationResult::Invalid);
}