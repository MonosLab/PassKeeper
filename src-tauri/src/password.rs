use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password {
    pub id: String,
    pub title: String,
    pub username: String,
    pub password: String,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Password {
    pub fn new(title: String, username: String, password: String, url: Option<String>, notes: Option<String>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Password {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            username,
            password,
            url,
            notes,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, title: String, username: String, password: String, url: Option<String>, notes: Option<String>) {
        self.title = title;
        self.username = username;
        self.password = password;
        self.url = url;
        self.notes = notes;
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}
