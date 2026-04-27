use rand::Rng;

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*";

const WORD_LIST: &[&str] = &[
    "correct", "horse", "battery", "staple", "apple", "river", "cloud", "tiger", "piano", "forest",
    "silver", "rocket", "ocean", "purple", "castle", "butter", "garden", "monkey", "pencil",
    "window", "frozen", "bridge", "candle", "desert", "falcon", "gentle", "harbor", "island",
];

pub fn generate_random(length: usize, use_symbols: bool) -> String {
    let mut rng = rand::thread_rng();
    let mut charset = String::new();
    charset.push_str(LOWERCASE);
    charset.push_str(UPPERCASE);
    charset.push_str(DIGITS);
    if use_symbols {
        charset.push_str(SYMBOLS);
    }
    let chars: Vec<char> = charset.chars().collect();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars[idx]
        })
        .collect()
}

pub fn generate_passphrase(word_count: usize, separator: char) -> String {
    let mut rng = rand::thread_rng();
    (0..word_count)
        .map(|_| {
            let idx = rng.gen_range(0..WORD_LIST.len());
            WORD_LIST[idx]
        })
        .collect::<Vec<&str>>()
        .join(&separator.to_string())
}

pub fn generate_pin(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let digits: Vec<char> = DIGITS.chars().collect();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..digits.len());
            digits[idx]
        })
        .collect()
}
