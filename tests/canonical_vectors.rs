use std::collections::HashSet;

use qryption_g1::authorization::Authorization;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CanonicalVector {
    case_id: String,
    authorizations: Vec<Authorization>,
    canonical_bytes: Vec<String>,
    canonical_bytes_length: Vec<usize>,
    #[serde(default)]
    bytes_equal: Option<bool>,
    notes: String,
}

fn encode_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn canonical_vectors_match_frozen_g1_g3_format() {
    let vectors: Vec<CanonicalVector> =
        serde_json::from_str(include_str!("fixtures/canonical_vectors.json"))
            .expect("canonical vector fixture must be valid JSON");

    assert_eq!(vectors.len(), 7, "the frozen fixture must contain 7 cases");

    let mut case_ids = HashSet::new();

    for vector in &vectors {
        assert!(
            case_ids.insert(vector.case_id.clone()),
            "duplicate case_id: {}",
            vector.case_id
        );
        assert!(
            !vector.notes.is_empty(),
            "notes must not be empty for {}",
            vector.case_id
        );
        assert_eq!(
            vector.authorizations.len(),
            vector.canonical_bytes.len(),
            "authorization/bytes count mismatch in {}",
            vector.case_id
        );
        assert_eq!(
            vector.authorizations.len(),
            vector.canonical_bytes_length.len(),
            "authorization/length count mismatch in {}",
            vector.case_id
        );

        for (index, authorization) in vector.authorizations.iter().enumerate() {
            let actual = authorization.canonical_bytes();

            assert_eq!(
                actual.len(),
                vector.canonical_bytes_length[index],
                "canonical length mismatch in {} authorization {}",
                vector.case_id,
                index
            );
            assert_eq!(
                encode_hex(&actual),
                vector.canonical_bytes[index],
                "canonical bytes mismatch in {} authorization {}",
                vector.case_id,
                index
            );
        }

        if let Some(expected_equal) = vector.bytes_equal {
            assert_eq!(
                vector.authorizations.len(),
                2,
                "bytes_equal requires two authorizations in {}",
                vector.case_id
            );

            let first = vector.authorizations[0].canonical_bytes();
            let second = vector.authorizations[1].canonical_bytes();

            assert_eq!(
                first == second,
                expected_equal,
                "bytes_equal mismatch in {}",
                vector.case_id
            );
        }
    }
}
