use crate::crypto::Crypto;
use crate::password::Password;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
struct PasswordStore {
    passwords: Vec<Password>,
}

pub struct Storage {
    crypto: Mutex<Option<Crypto>>,
    data_path: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self, String> {
        let data_dir = dirs::data_dir()
            .ok_or("Could not find data directory")?
            .join("passkeeper");

        fs::create_dir_all(&data_dir)
            .map_err(|e| format!("Failed to create data directory: {}", e))?;

        let data_path = data_dir.join("passwords.enc");

        Ok(Storage {
            crypto: Mutex::new(None),
            data_path,
        })
    }

    pub fn unlock(&self, master_password: &str) -> Result<(), String> {
        let crypto = Crypto::new(master_password)?;
        let mut crypto_lock = self.crypto.lock().unwrap();
        *crypto_lock = Some(crypto);
        Ok(())
    }

    pub fn is_unlocked(&self) -> bool {
        self.crypto.lock().unwrap().is_some()
    }

    pub fn lock(&self) {
        let mut crypto_lock = self.crypto.lock().unwrap();
        *crypto_lock = None;
    }

    fn load_passwords(&self) -> Result<Vec<Password>, String> {
        if !self.data_path.exists() {
            return Ok(Vec::new());
        }

        let crypto_lock = self.crypto.lock().unwrap();
        let crypto = crypto_lock.as_ref().ok_or("Storage is locked")?;

        let encrypted_data = fs::read_to_string(&self.data_path)
            .map_err(|e| format!("Failed to read data file: {}", e))?;

        if encrypted_data.is_empty() {
            return Ok(Vec::new());
        }

        let decrypted = crypto.decrypt(&encrypted_data)?;
        let store: PasswordStore = serde_json::from_str(&decrypted)
            .map_err(|e| format!("Failed to parse password data: {}", e))?;

        Ok(store.passwords)
    }

    fn save_passwords(&self, passwords: &[Password]) -> Result<(), String> {
        let crypto_lock = self.crypto.lock().unwrap();
        let crypto = crypto_lock.as_ref().ok_or("Storage is locked")?;

        let store = PasswordStore {
            passwords: passwords.to_vec(),
        };

        let json = serde_json::to_string(&store)
            .map_err(|e| format!("Failed to serialize passwords: {}", e))?;

        let encrypted = crypto.encrypt(&json)?;

        fs::write(&self.data_path, encrypted)
            .map_err(|e| format!("Failed to write data file: {}", e))?;

        Ok(())
    }

    pub fn add_password(&self, password: Password) -> Result<Password, String> {
        let mut passwords = self.load_passwords()?;
        passwords.push(password.clone());
        self.save_passwords(&passwords)?;
        Ok(password)
    }

    pub fn get_all_passwords(&self) -> Result<Vec<Password>, String> {
        self.load_passwords()
    }

    pub fn get_password(&self, id: &str) -> Result<Option<Password>, String> {
        let passwords = self.load_passwords()?;
        Ok(passwords.into_iter().find(|p| p.id == id))
    }

    pub fn update_password(&self, id: &str, title: String, username: String, password: String, url: Option<String>, notes: Option<String>) -> Result<Password, String> {
        let mut passwords = self.load_passwords()?;
        
        let password_entry = passwords.iter_mut()
            .find(|p| p.id == id)
            .ok_or("Password not found")?;

        password_entry.update(title, username, password, url, notes);
        let updated = password_entry.clone();

        self.save_passwords(&passwords)?;
        Ok(updated)
    }

    pub fn delete_password(&self, id: &str) -> Result<(), String> {
        let mut passwords = self.load_passwords()?;
        passwords.retain(|p| p.id != id);
        self.save_passwords(&passwords)?;
        Ok(())
    }

    pub fn search_passwords(&self, query: &str) -> Result<Vec<Password>, String> {
        let passwords = self.load_passwords()?;
        let query_lower = query.to_lowercase();
        
        Ok(passwords.into_iter()
            .filter(|p| {
                p.title.to_lowercase().contains(&query_lower)
                    || p.username.to_lowercase().contains(&query_lower)
                    || p.url.as_ref().map_or(false, |u| u.to_lowercase().contains(&query_lower))
            })
            .collect())
    }
}
