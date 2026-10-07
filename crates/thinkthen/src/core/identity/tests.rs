use super::{AnswerId, CallId, FailureId, IdentityError, ObservationId, SdkRequestId};

#[test]
fn identities_accept_only_the_exact_lowercase_hexadecimal_spelling() {
    for valid in ["0".repeat(64), "abcdef0123456789".repeat(4)] {
        let id = CallId::new(valid.clone()).unwrap();
        assert_eq!(id.as_str(), valid);
        assert_eq!(serde_json::to_string(&id).unwrap(), format!("\"{valid}\""));
        assert_eq!(
            serde_json::from_str::<CallId>(&format!("\"{valid}\"")).unwrap(),
            id
        );
    }
    for invalid in [
        "".into(),
        "0".repeat(63),
        "0".repeat(65),
        "A".repeat(64),
        "g".repeat(64),
        format!(" {}", "0".repeat(64)),
        "é".repeat(32),
    ] {
        assert_eq!(CallId::new(invalid.clone()), Err(IdentityError));
        assert_eq!(SdkRequestId::new(invalid.clone()), Err(IdentityError));
        assert_eq!(ObservationId::new(invalid.clone()), Err(IdentityError));
        assert_eq!(FailureId::new(invalid.clone()), Err(IdentityError));
        assert_eq!(AnswerId::new(invalid.clone()), Err(IdentityError));
        let encoded = serde_json::to_string(&invalid).unwrap();
        assert!(serde_json::from_str::<ObservationId>(&encoded).is_err());
    }
}

#[test]
fn invalid_identity_diagnostics_withhold_the_supplied_bytes() {
    let secret = "private-evidence-marker";
    let error = AnswerId::new(secret).unwrap_err();
    assert!(!format!("{error:?} {error}").contains(secret));
}
