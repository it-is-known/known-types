// This is free and unencumbered software released into the public domain.

#![cfg(all(feature = "alloc", feature = "serde"))]

use known_types_x::XHandle;

#[test]
fn test_serde_validates_and_preserves_case() {
    let handle: XHandle = serde_json::from_str(r#""@PlayItAgainSam""#).expect("valid handle");
    assert_eq!(handle.as_str(), "PlayItAgainSam");
    let encoded = serde_json::to_string(&handle).expect("serialize handle");
    assert_eq!(encoded, r#""PlayItAgainSam""#);
    assert_eq!(
        serde_json::from_str::<XHandle>(&encoded)
            .expect("valid handle")
            .as_str(),
        handle.as_str()
    );

    for input in [
        "",
        "@",
        "@@alice",
        "alice smith",
        "álîce",
        "a".repeat(16).as_str(),
    ] {
        let json = serde_json::to_string(input).expect("serialize string");
        assert!(
            serde_json::from_str::<XHandle>(&json).is_err(),
            "accepted {input:?}"
        );
    }
    for json in ["null", "42", "true", "[]", "{}"] {
        assert!(serde_json::from_str::<XHandle>(json).is_err());
    }
}
