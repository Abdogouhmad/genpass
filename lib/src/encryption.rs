use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce}; // Or `Aes128Gcm` if you want 128-bit
use anyhow::{Context, Error, Result};
use argon2::Argon2;
use rand::TryRngCore;
use std::fs;
use std::io::Read;
use zeroize::Zeroize;
/// GenEnc for encryption and encryption logic
pub struct GenEnc;

impl GenEnc {
    /// prompt_password is function that prompt password
    pub fn prompt_password() -> Result<String, Error> {
        let passcode =
            rpassword::prompt_password("Enter password: ").context("Failed to read password")?;

        Ok(passcode)
    }

    /// Encrypt a file (stub for now — implement later)
    pub fn encrypt_file(path: &str) -> Result<(), Error> {
        let password = Self::prompt_password()?;

        // read file
        let mut file_data = Vec::new();
        fs::File::open(path)
            .context(format!("Failed to open file: {path}"))?
            .read_to_end(&mut file_data)?;
        // 3. Generate random salt (16 bytes)
        let mut salt = [0u8; 16];
        let _ = rand::rngs::OsRng.try_fill_bytes(&mut salt);

        // 4. Derive 32-byte key with Argon2id
        let mut key = [0u8; 32];
        Argon2::default()
            .hash_password_into(password.as_bytes(), &salt, &mut key)
            .context("Key derivation failed")?;

        // Wipe password
        {
            let mut pw = password;
            pw.zeroize();
        }

        // 5. Generate random nonce (12 bytes for AES-GCM)
        let mut nonce_bytes = [0u8; 12];
        let _ = rand::rngs::OsRng.try_fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // 6. Encrypt
        let cipher = Aes256Gcm::new(&key.into());
        let ciphertext = cipher.encrypt(nonce, file_data.as_ref()).unwrap();

        // 7. Write output file
        let out_path = format!("{}.enc", path);
        let mut out = fs::File::create(&out_path)
            .context(format!("Failed to create output file: {out_path}"))?;

        // Format: MAGIC | VERSION | SALT | NONCE | CIPHERTEXT
        const MAGIC: &[u8; 8] = b"GENENC01";
        let version: u8 = 1;

        use std::io::Write;
        out.write_all(MAGIC)?;
        out.write_all(&[version])?;
        out.write_all(&salt)?;
        out.write_all(&nonce_bytes)?;
        out.write_all(&ciphertext)?;

        // Wipe key
        key.zeroize();

        println!("🔐 Successfully encrypted -> {out_path}");
        Ok(())
    }

    /// Decrypt a file (stub for now)
    pub fn decrypt_file(path: &str) -> Result<()> {
        let password = Self::prompt_password()?;
        println!(
            "🔓 Decrypting {path} with password length {}...",
            password.len()
        );

        // TODO: implement real decryption logic here
        Ok(())
    }
}
