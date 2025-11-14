use anyhow::{Error, Result};
use rpassword;
/// GenEnc for encryption and encryption logic
pub struct GenEnc {
    pub password: String,
}

impl GenEnc {
    pub fn new() -> Self {
        Self {
            password: "".to_string(),
        }
    }

    /// prompt_password is function that prompt password
    pub fn prompt_password() -> Result<String, Error> {
        let passcode = rpassword::prompt_password("Safe code please: ")?;

        Ok(passcode)
    }
}

impl Default for GenEnc {
    fn default() -> Self {
        GenEnc {
            password: "".to_string(),
        }
    }
}
