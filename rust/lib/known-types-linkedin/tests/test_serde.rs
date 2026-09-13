// This is free and unencumbered software released into the public domain.

#![cfg(all(feature = "alloc", feature = "serde"))]

use known_types_linkedin::LinkedinHandle;

#[test]
fn test_serde_validates_percent_encoded_handles() {
    let handle: LinkedinHandle = serde_json::from_str(r#"" Bj%C3%96rn ""#).expect("valid handle");
    assert_eq!(handle.as_str(), "BjÖrn");
    let encoded = serde_json::to_string(&handle).expect("serialize handle");
    assert_eq!(encoded, r#""BjÖrn""#);
    assert_eq!(
        serde_json::from_str::<LinkedinHandle>(&encoded)
            .expect("valid handle")
            .as_str(),
        handle.as_str()
    );

    for input in [
        "",
        "ab",
        "@alice",
        "abc%GG",
        "abc%FF",
        "%20alice%20",
        "alice%2Fsmith",
        "literal%2520handle",
        "a".repeat(101).as_str(),
    ] {
        let json = serde_json::to_string(input).expect("serialize string");
        assert!(
            serde_json::from_str::<LinkedinHandle>(&json).is_err(),
            "accepted {input:?}"
        );
    }
    for json in ["null", "42", "true", "[]", "{}"] {
        assert!(serde_json::from_str::<LinkedinHandle>(json).is_err());
    }
}
