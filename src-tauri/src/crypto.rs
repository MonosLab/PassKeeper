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
        // WARNING: This is a simplified key derivation for demonstration purposes.
        // PRODUCTION USE REQUIRES: Use a proper key derivation function like PBKDF2 
        // (with 100,000+ iterations) or Argon2id with salt for secure key generation.
        // The current implementation is vulnerable to dictionary attacks and should 
        // not be used for production password management.
        
        let mut key = [0u8; 32];
        let pwd_bytes = master_password.as_bytes();
        
        // Copy password bytes to key
        for (i, byte) in pwd_bytes.iter().enumerate().take(32) {
            key[i] = *byte;
        }
        
        // Fill remaining bytes with repeated pattern if password is shorter than 32 bytes
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

pub fn generate_password(length: usize, use_symbols: bool, use_numbers: bool, use_uppercase: bool) -> Result<String, String> {
    // Validate password length to prevent weak or resource-intensive passwords
    if length < 8 {
        return Err("Password length must be at least 8 characters".to_string());
    }
    if length > 128 {
        return Err("Password length cannot exceed 128 characters".to_string());
    }
    
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

    Ok(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_encryption_decryption() {
        let crypto = Crypto::new("test_master_password").unwrap();
        let plaintext = "Hello, World!";
        
        let encrypted = crypto.encrypt(plaintext).unwrap();
        let decrypted = crypto.decrypt(&encrypted).unwrap();
        
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_crypto_different_plaintexts() {
        let crypto = Crypto::new("my_secure_password").unwrap();
        
        let text1 = "password123";
        let text2 = "different_password";
        
        let encrypted1 = crypto.encrypt(text1).unwrap();
        let encrypted2 = crypto.encrypt(text2).unwrap();
        
        assert_ne!(encrypted1, encrypted2);
        assert_eq!(crypto.decrypt(&encrypted1).unwrap(), text1);
        assert_eq!(crypto.decrypt(&encrypted2).unwrap(), text2);
    }

    #[test]
    fn test_generate_password_length() {
        let password = generate_password(16, true, true, true).unwrap();
        assert_eq!(password.len(), 16);
        
        let password = generate_password(32, true, true, true).unwrap();
        assert_eq!(password.len(), 32);
    }

    #[test]
    fn test_generate_password_contains_types() {
        let password = generate_password(50, true, true, true).unwrap();
        
        let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
        let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
        let has_number = password.chars().any(|c| c.is_ascii_digit());
        let has_symbol = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));
        
        assert!(has_lowercase);
        // These might not always be true due to randomness, but with length 50 they're very likely
        // In a real test, we'd want to be more deterministic
        assert!(has_uppercase || has_number || has_symbol);
    }

    #[test]
    fn test_generate_password_lowercase_only() {
        let password = generate_password(20, false, false, false).unwrap();
        assert_eq!(password.len(), 20);
        assert!(password.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn test_generate_password_bounds() {
        // Test minimum length
        assert!(generate_password(7, true, true, true).is_err());
        assert!(generate_password(8, true, true, true).is_ok());
        
        // Test maximum length
        assert!(generate_password(128, true, true, true).is_ok());
        assert!(generate_password(129, true, true, true).is_err());
    }
}
