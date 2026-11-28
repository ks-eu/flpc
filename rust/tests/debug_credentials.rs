use std::fmt::Debug;

use flpc::{unit::FlpcUnit, wire::decode};

fn assert_debug_redacts(unit: &impl Debug, fields: &[(&str, &str)]) {
    for output in [format!("{unit:?}"), format!("{unit:#?}")] {
        assert!(output.contains(r#"username: "alice""#));
        for (field, value) in fields {
            assert!(output.contains(&format!(r#"{field}: "[redacted]""#)));
            assert!(!output.contains(value));
        }
    }
}

#[test]
fn decoded_register_redacts_token_and_password() {
    let (unit, _) = decode(b"\x01\x0dexample-token\x05alice\x10example-password")
        .unwrap()
        .unwrap();
    let FlpcUnit::Register(register) = &unit else {
        panic!("expected a Register unit");
    };

    let fields = [("token", "example-token"), ("password", "example-password")];
    assert_debug_redacts(register, &fields);
    assert_debug_redacts(&unit, &fields);
}

#[test]
fn decoded_login_redacts_password() {
    let (unit, _) = decode(b"\x03\x05alice\x10example-password")
        .unwrap()
        .unwrap();
    let FlpcUnit::Login(login) = &unit else {
        panic!("expected a Login unit");
    };

    let fields = [("password", "example-password")];
    assert_debug_redacts(login, &fields);
    assert_debug_redacts(&unit, &fields);
}

#[test]
fn decoded_add_redacts_password() {
    let (unit, _) = decode(b"\x60\x05alice\x10example-password")
        .unwrap()
        .unwrap();
    let FlpcUnit::Add(add) = &unit else {
        panic!("expected an Add unit");
    };

    let fields = [("password", "example-password")];
    assert_debug_redacts(add, &fields);
    assert_debug_redacts(&unit, &fields);
}
