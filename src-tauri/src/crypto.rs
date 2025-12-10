use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;

pub struct Crypto {
    cipher: Aes256Gcm,
}

impl Crypto {
    pub fn new(master_password: &str) -> Result<Self, String> {
        // Derive a key from master password (simple version - in production use PBKDF2 or Argon2)
        let mut key = [0u8; 32];
        let pwd_bytes = master_password.as_bytes();
        for (i, byte) in pwd_bytes.iter().enumerate().take(32) {
            key[i] = *byte;
        }
        // Fill remaining bytes with repeated pattern if password is shorter
        if pwd_bytes.len() < 32 {
            for i in pwd_bytes.len()..32 {
                key[i] = pwd_bytes[i % pwd_bytes.len()];
            }
        }

        let cipher = Aes256Gcm::new(&key.into());
        Ok(Crypto { cipher })
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String, String> {
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| format!("Encryption error: {}", e))?;

        // Combine nonce and ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);

        Ok(general_purpose::STANDARD.encode(&result))
    }

    pub fn decrypt(&self, encrypted: &str) -> Result<String, String> {
        let data = general_purpose::STANDARD
            .decode(encrypted)
            .map_err(|e| format!("Base64 decode error: {}", e))?;

        if data.len() < 12 {
            return Err("Invalid encrypted data".to_string());
        }

        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption error: {}", e))?;

        String::from_utf8(plaintext).map_err(|e| format!("UTF-8 decode error: {}", e))
    }
}

pub fn generate_password(length: usize, use_symbols: bool, use_numbers: bool, use_uppercase: bool) -> String {
    let mut charset = String::from("abcdefghijklmnopqrstuvwxyz");
    
    if use_uppercase {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if use_numbers {
        charset.push_str("0123456789");
    }
    if use_symbols {
        charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
    }

    let charset_bytes: Vec<u8> = charset.bytes().collect();
    let mut password = String::new();
    let mut rng = rand::thread_rng();

    for _ in 0..length {
        let idx = (rng.next_u32() as usize) % charset_bytes.len();
        password.push(charset_bytes[idx] as char);
    }

    password
}
