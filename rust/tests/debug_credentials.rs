use flpc::unit::{Add, Login, Register};

#[test]
fn register_debug_hides_token_and_password() {
    let unit = Register::new(
        "example-token".into(),
        "alice".into(),
        "example-password".into(),
    );
    let output = format!("{unit:?}");

    assert!(!output.contains("example-token"));
    assert!(!output.contains("example-password"));
}

#[test]
fn login_debug_hides_password() {
    let unit = Login::new("alice".into(), "example-password".into());

    assert!(!format!("{unit:?}").contains("example-password"));
}

#[test]
fn add_debug_hides_password() {
    let unit = Add::new("alice".into(), "example-password".into());

    assert!(!format!("{unit:?}").contains("example-password"));
}
