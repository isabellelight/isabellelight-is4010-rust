#[derive(Debug, PartialEq, Clone)]
pub enum PasswordStrength {
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

pub fn validate_strength(password: &str) -> PasswordStrength {
    let len = password.len();
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_symbol = password.chars().any(|c| !c.is_alphanumeric());

    match (len, has_lower, has_upper, has_digit, has_symbol) {
        (l, _, _, _, true) if l >= 12 => PasswordStrength::VeryStrong,
        (l, true, true, true, _) if l >= 8 => PasswordStrength::Strong,
        (l, true, true, _, _) if l >= 8 => PasswordStrength::Medium,
        _ => PasswordStrength::Weak,
    }
}

pub fn check_common_patterns(password: &str) -> bool {
    let common = ["123", "456", "789", "qwerty", "asdf", "aaa", "111", "000"];
    let lower = password.to_lowercase();
    common.iter().any(|pattern| lower.contains(pattern))
}

pub fn calculate_entropy(password: &str) -> f64 {
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_symbol = password.chars().any(|c| !c.is_alphanumeric());

    let mut charset_size = 0usize;
    if has_lower {
        charset_size += 26;
    }
    if has_upper {
        charset_size += 26;
    }
    if has_digit {
        charset_size += 10;
    }
    if has_symbol {
        charset_size += 8;
    }

    if charset_size == 0 {
        return 0.0;
    }

    password.len() as f64 * (charset_size as f64).log2()
}
