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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_creation() {
        let pwd = Password::new(
            "Gmail".to_string(),
            "user@example.com".to_string(),
            "secret123".to_string(),
            Some("https://gmail.com".to_string()),
            Some("Personal email".to_string()),
        );

        assert_eq!(pwd.title, "Gmail");
        assert_eq!(pwd.username, "user@example.com");
        assert_eq!(pwd.password, "secret123");
        assert_eq!(pwd.url, Some("https://gmail.com".to_string()));
        assert_eq!(pwd.notes, Some("Personal email".to_string()));
        assert!(!pwd.id.is_empty());
        assert!(pwd.created_at > 0);
        assert_eq!(pwd.created_at, pwd.updated_at);
    }

    #[test]
    fn test_password_update() {
        let mut pwd = Password::new(
            "Old Title".to_string(),
            "old@example.com".to_string(),
            "oldpass".to_string(),
            None,
            None,
        );

        let created_at = pwd.created_at;
        std::thread::sleep(std::time::Duration::from_millis(10));

        pwd.update(
            "New Title".to_string(),
            "new@example.com".to_string(),
            "newpass".to_string(),
            Some("https://example.com".to_string()),
            Some("Updated notes".to_string()),
        );

        assert_eq!(pwd.title, "New Title");
        assert_eq!(pwd.username, "new@example.com");
        assert_eq!(pwd.password, "newpass");
        assert_eq!(pwd.url, Some("https://example.com".to_string()));
        assert_eq!(pwd.notes, Some("Updated notes".to_string()));
        assert_eq!(pwd.created_at, created_at);
        assert!(pwd.updated_at > created_at);
    }

    #[test]
    fn test_password_unique_ids() {
        let pwd1 = Password::new(
            "Test1".to_string(),
            "user1".to_string(),
            "pass1".to_string(),
            None,
            None,
        );

        let pwd2 = Password::new(
            "Test2".to_string(),
            "user2".to_string(),
            "pass2".to_string(),
            None,
            None,
        );

        assert_ne!(pwd1.id, pwd2.id);
    }
}
