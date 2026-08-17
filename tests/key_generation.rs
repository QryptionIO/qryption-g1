use qryption_g1::keys;
use signature::SignatureEncoding;

#[test]
fn generates_nonempty_keys() {
    let kp = keys::generate();
    assert!(!format!("{:?}", kp.signing_key).is_empty());
    assert!(!format!("{:?}", kp.verifying_key).is_empty());
}

#[test]
fn prints_key_and_signature_sizes() {
    use signature::Signer;

    let kp = keys::generate();
    let sig = kp.signing_key.sign(b"size probe");

    let vk_bytes = kp.verifying_key.encode();
    let sig_bytes = sig.to_bytes();

    println!("Verifying (public) key size: {} bytes", vk_bytes.len());
    println!("Signature size: {} bytes", sig_bytes.len());
}