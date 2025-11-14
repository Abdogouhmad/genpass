use anyhow::{Error, Result};
use rand::{self, Rng}; // Import the Rng trait

// --- Constants for password ---
static MAX_PASSWORD_LENGTH: usize = 20;
static MIN_PASSWORD_LENGTH: usize = 12;

// --- Constants for user ---
static MAX_USER_LEN: usize = 8;
static MIN_USER_LEN: usize = 4;

/// A struct to hold character sets and generate credentials.
pub struct CredentialGenerator {
    pub numbers: String,
    pub lower_alphabet: String,
    pub upper_alphabet: String,
    pub special_chars: String,
}

impl CredentialGenerator {
    /// Creates a new generator with default character sets.
    pub fn new() -> Self {
        Self {
            numbers: "0123456789".to_string(),
            lower_alphabet: "abcdefghijklmnopqrstuvwxyz".to_string(),
            upper_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
            special_chars: "!@#$%^&*()_+-=[]{}|;:',.<>/?".to_string(),
        }
    }

    /// gen_user - cool way to generate a username
    /// few words meaningless yet cool asf
    pub fn gen_user(&self) -> Result<String, Error> {
        let mut rng = rand::rng(); // Use thread_rng
        let user_len = rng.random_range(MIN_USER_LEN..=MAX_USER_LEN); // Use random_range
        let mut user = String::with_capacity(user_len);

        for _ in 0..user_len {
            let user_type = rng.random_range(0..2); // 0 for number, 1 for lower_alphabet
            let chosen_usr = match user_type {
                0 => {
                    let idx = rng.random_range(0..self.numbers.len());
                    self.numbers.chars().nth(idx).unwrap()
                }
                1 => {
                    let idx = rng.random_range(0..self.lower_alphabet.len());
                    self.lower_alphabet.chars().nth(idx).unwrap()
                }
                _ => unreachable!(),
            };
            user.push(chosen_usr);
        }
        Ok(user)
    }

    /// gen_password - generate password from 12 char to 20 char max
    /// hard to crack hard to memorize
    pub fn gen_password(&self) -> Result<String, Error> {
        let mut rng = rand::rng(); // Use thread_rng
        let passcode_length = rng.random_range(MIN_PASSWORD_LENGTH..=MAX_PASSWORD_LENGTH); // Use gen_range
        let mut password = String::with_capacity(passcode_length);

        for _ in 0..passcode_length {
            let char_type = rng.random_range(0..4); // 0-3 for different char types
            let chosen_char = match char_type {
                0 => {
                    let idx = rng.random_range(0..self.numbers.len());
                    self.numbers.chars().nth(idx).unwrap()
                }
                1 => {
                    let idx = rng.random_range(0..self.lower_alphabet.len());
                    self.lower_alphabet.chars().nth(idx).unwrap()
                }
                2 => {
                    let idx = rng.random_range(0..self.upper_alphabet.len());
                    self.upper_alphabet.chars().nth(idx).unwrap()
                }
                3 => {
                    let idx = rng.random_range(0..self.special_chars.len());
                    self.special_chars.chars().nth(idx).unwrap()
                }
                _ => unreachable!(),
            };
            password.push(chosen_char);
        }
        Ok(password)
    }
}

impl Default for CredentialGenerator {
    fn default() -> Self {
        CredentialGenerator {
            numbers: "0123456789".to_string(),
            lower_alphabet: "abcdefghijklmnopqrstuvwxyz".to_string(),
            upper_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
            special_chars: "!@#$%^&*()_+-=[]{}|;:',.<>/?".to_string(),
        }
    }
}
