use testing_and_documentation::is_passing;

#[test]
fn integration_test_uses_public_api() {
    assert!(is_passing(90));
    assert!(!is_passing(59));
}
