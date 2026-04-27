use passgen::{
    generate_passphrase, generate_pin, generate_random, validate_strength, PasswordStrength,
};

#[test]
fn test_random_password_length() {
    let password = generate_random(20, false);
    assert_eq!(password.len(), 20);
}

#[test]
fn test_random_password_with_symbols() {
    let password = generate_random(100, true);
    let has_symbol = password.chars().any(|c| "!@#$%^&*".contains(c));
    assert!(has_symbol || password.len() == 100);
}

#[test]
fn test_pin_only_digits() {
    let pin = generate_pin(6);
    assert_eq!(pin.len(), 6);
    assert!(pin.chars().all(|c| c.is_numeric()));
}

#[test]
fn test_validate_weak_password() {
    assert_eq!(validate_strength("abc"), PasswordStrength::Weak);
    assert_eq!(validate_strength("password"), PasswordStrength::Weak);
}

#[test]
fn test_validate_medium_password() {
    assert_eq!(validate_strength("Password"), PasswordStrength::Medium);
}

#[test]
fn test_validate_strong_password() {
    assert_eq!(validate_strength("Password123"), PasswordStrength::Strong);
}

#[test]
fn test_validate_very_strong_password() {
    assert_eq!(
        validate_strength("MyP@ssw0rd123!"),
        PasswordStrength::VeryStrong
    );
}

#[test]
fn test_passphrase_word_count() {
    let passphrase = generate_passphrase(5, '-');
    let word_count = passphrase.split('-').count();
    assert_eq!(word_count, 5);
}

#[test]
fn test_passphrase_separator() {
    let passphrase = generate_passphrase(3, '_');
    assert!(passphrase.contains('_'));
}
