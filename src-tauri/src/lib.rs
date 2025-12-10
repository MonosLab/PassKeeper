pub mod module;
use common::define;
use base64::{engine::general_purpose, Engine as _};
use module::{
    registry::{
        self,
        HKEY_CURRENT_USER
    },
    crypto::{
        myhash::hash_data,
        mycrypt::{
            encrypt_data,
            decrypt_data
        },
    }
};
use std::{
    fs,
    env::current_dir,
    path::Path,
    thread,
    time::Duration,
};
use serde_json::{
    json,
    Value,
    from_str,
    to_string,
};
use tauri::{
    AppHandle,
    Manager,
    command,
};
use once_cell::sync::OnceCell;

static APP_HANDLE: OnceCell<AppHandle> = OnceCell::new();

// First line of username.dat file is the password.
// Subsequent lines contain user-saved data.
#[allow(dead_code)]
#[repr(u32)]
enum PassKeeperError {
    Success = 0,
    DirCreationFailed = 1,
    FileAlreadyExists = 2,
    FileNotFound = 3,
    FileWriteFailed = 4,
    DuplicateEntry = 5,
    EncryptionFailed = 6,
    DecryptionFailed = 7,
}

#[derive(Debug)]
struct SecretData {
    skey: String,
    data: String,
}

impl SecretData {
    fn new() -> Self {
        SecretData {
            skey: String::new(),
            data: String::new(),
        }
    }
    fn is_skey_empty(&self) -> bool {
        self.skey.is_empty()
    }
    fn is_data_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[command]
fn save_account(username: String, password: String) -> u32 {
    let file_path = format!("{}/data/{}.dat", current_dir().unwrap().display(), username);
    // Check if the directory exists, if not create it
    if fs::create_dir_all("./data").is_err() {
        return PassKeeperError::DirCreationFailed as u32;
    }
    // Check if the file already exists
    if Path::new(&file_path).exists() {
        return PassKeeperError::FileAlreadyExists as u32;
    }
    // SHA512 Hash the password before storing (optional)
    let hashed_password = hash_data(&password);
    let mut password = hashed_password;
    password.push_str("=="); // Append '==' to the hashed password
    // Create the file and write the password as the first line
    if fs::write(file_path, password).is_err() {
        return PassKeeperError::FileWriteFailed as u32;
    }

    PassKeeperError::Success as u32
}

fn shake_window(window: tauri::WebviewWindow) {
    // Get current position
    if let Ok(position) = window.outer_position() {
        let x = position.x;
        let y = position.y;
        // Shake 5 times, alternating left and right
        for i in 0..5 {
            let offset = if i % 2 == 0 { 10 } else { -10 };
            let _ = window.set_position(tauri::PhysicalPosition {
                x: x + offset,
                y,
            });
            thread::sleep(Duration::from_millis(50));
        }

        // Restore to original position
        let _ = window.set_position(position);
    }
}

#[command]
fn validate_login(username: String, password: String) -> bool {
    let file_path = format!("{}/data/{}.dat", current_dir().unwrap().display(), username);
    // Check if the file exists
    if !Path::new(&file_path).exists() {
        return false;
    }
    print_out!("Validating login for user: {}", username);

    // Read the stored password (first line of the file)
    let stored_content = fs::read_to_string(&file_path).unwrap_or_default();
    let mut lines = stored_content.lines();
    let stored_password = lines.next().unwrap_or_default();
    //println!("Stored password: [{}]", stored_password);
    let hashed_password = hash_data(&password);
    let mut password = hashed_password;
    password.push_str("=="); // Append '==' to the hashed password
    //println!("Provided password: [{}]", password);
    if stored_password != password {
        // Shake the window to indicate error
        if let Some(app_handle) = APP_HANDLE.get() {
            if let Some(window) = app_handle.get_webview_window("main") {
                shake_window(window);
            }
        }
        return false;
    }
    true
}

#[command]
fn save_data(username: String, service: String, account: String, password: String) -> u32 {
    let file_path = format!("{}/data/{}.dat", current_dir().unwrap().display(), username);
    // Check if the file exists
    if !Path::new(&file_path).exists() {
        return PassKeeperError::FileNotFound as u32;
    }

    // Retrieve existing secret data
    let secure_data = get_secret_data(username.clone());
    print_out!("Secure data retrieved: [{:?}]", secure_data);
    
    if secure_data.is_skey_empty() {
        return PassKeeperError::FileNotFound as u32;
    }
    if secure_data.is_data_empty() {
        print_out!("No existing data, creating new entry.");
        // No existing data, create new entry
        let new_entry = json!({
            "service": service,
            "account": account,
            "password": password,
        });
        let mut new_content = String::new();
        new_content.push_str(&secure_data.skey);
        new_content.push('\n');
        // Following lines are the json entries
        let entry_str = to_string(&vec![new_entry]).unwrap_or_default();
        print_out!("New entry string: [{}]", entry_str);
        let encrypted_entry_str = encrypt_data(&entry_str);
        print_out!("Encrypted entry result: [{:?}]", encrypted_entry_str);
        match encrypted_entry_str {
            Ok(ed) => {
                let push_data = general_purpose::STANDARD.encode(&ed);
                new_content.push_str(&push_data);
            },
            Err(_) => {
                return PassKeeperError::EncryptionFailed as u32;
            }
        }
        new_content.push('\n');

        // Write back to the file
        if fs::write(file_path, new_content).is_err() {
            return PassKeeperError::FileWriteFailed as u32;
        }
    } else {
        print_out!("Existing data found, checking for duplicates.");
        // Parse existing data
        let data: Vec<Value> = secure_data.data.lines()
            .map(|line| from_str(line).unwrap_or(Value::Null))
            .filter(|v| !v.is_null())
            .collect();
        // Flatten arrays if any    
        let mut new_data: Vec<Value> = data.into_iter().flat_map(|v| {
            if let Value::Array(arr) = v {
                arr.into_iter()
            } else {
                vec![v].into_iter()
            }
        }).collect();
        // Check for duplicates, service & account combination
        if new_data.is_empty() {
            new_data = Vec::new();
        } else {
            for entry in &new_data {
                if entry["service"] == service && entry["account"] == account {
                    // Duplicate found
                    return PassKeeperError::DuplicateEntry as u32;
                }
            }
        }

        // Append new data
        let new_entry = json!({
            "service": service,
            "account": account,
            "password": password,
        });
        new_data.push(new_entry);
        
        // Convert back to string format
        let mut new_content = String::new();
        // The password
        new_content.push_str(&secure_data.skey);
        new_content.push('\n');
        // Following lines are the json entries
        let entry_str = to_string(&new_data).unwrap_or_default();
        let encrypted_entry_str = encrypt_data(&entry_str);
        match encrypted_entry_str {
            Ok(ed) => {
                let push_data = general_purpose::STANDARD.encode(&ed);
                new_content.push_str(&push_data);
            },
            Err(_) => {
                return PassKeeperError::FileWriteFailed as u32;
            }
        }
        new_content.push('\n');

        // Write back to the file
        if fs::write(file_path, new_content).is_err() {
            return PassKeeperError::FileWriteFailed as u32;
        }
    }
    PassKeeperError::Success as u32
}

#[command]
fn modify_data(username: &str, service: &str, account: &str, password: &str) -> u32 {
    let file_path = format!("{}/data/{}.dat", current_dir().unwrap().display(), username);
    // Check if the file exists
    if !Path::new(&file_path).exists() {
        return PassKeeperError::FileNotFound as u32;
    }

    let secret_data = get_secret_data(username.to_string());
    if secret_data.is_skey_empty() || secret_data.is_data_empty() {
        return PassKeeperError::FileNotFound as u32;
    }
    // Parse existing data
    let data = secret_data.data.lines()
        .map(|line| from_str(line).unwrap_or(Value::Null))
        .filter(|v| !v.is_null())
        .collect::<Vec<Value>>();
    // Flatten arrays if any    
    let mut new_data: Vec<Value> = data.into_iter().flat_map(|v|
        if let Value::Array(arr) = v {
            arr.into_iter()
        } else {
            vec![v].into_iter()
        }
    ).collect();
    // Modify the matching entry
    for entry in &mut new_data {
        if entry["service"] == service && entry["account"] == account {
            entry["password"] = Value::String(password.to_string());
        }
    }
    // Convert back to string format
    let mut new_content = String::new();
    // The password
    new_content.push_str(&secret_data.skey);
    new_content.push('\n');
    // Following lines are the json entries
    let entry_str = to_string(&new_data).unwrap_or_default();
    let encrypted_entry_str = encrypt_data(&entry_str);
    match encrypted_entry_str {
        Ok(ed) => {
            let push_data = general_purpose::STANDARD.encode(&ed);
            new_content.push_str(&push_data);
        },
        Err(_) => {
            return PassKeeperError::EncryptionFailed as u32;
        }
    }
    new_content.push('\n');
    // Write back to the file
    if fs::write(file_path, new_content).is_err() {
        return PassKeeperError::FileWriteFailed as u32;
    }
    PassKeeperError::Success as u32
}

#[command]
fn remove_data(username: &str, service: &str) -> u32 {
    let file_path = format!("{}/data/{}.dat", current_dir().unwrap().display(), username);
    // Check if the file exists
    if !Path::new(&file_path).exists() {
        return PassKeeperError::FileNotFound as u32;
    }

    let secret_data = get_secret_data(username.to_string());
    let data = secret_data.data.lines()
        .map(|line| from_str(line).unwrap_or(Value::Null))
        .filter(|v| !v.is_null())
        .collect::<Vec<Value>>();
    // Flatten arrays if any    
    let mut new_data: Vec<Value> = data.into_iter().flat_map(|v|
        if let Value::Array(arr) = v {
            arr.into_iter()
        } else {
            vec![v].into_iter()
        }
    ).collect();
    // Remove entries matching the service
    new_data.retain(|entry| entry["service"] != service);
    // Convert back to string format
    let mut new_content = String::new();
    // The password
    new_content.push_str(&secret_data.skey);
    new_content.push('\n');
    // Following lines are the json entries
    let entry_str = to_string(&new_data).unwrap_or_default();
    let encrypted_entry_str = encrypt_data(&entry_str);
    match encrypted_entry_str {
        Ok(ed) => {
            let push_data = general_purpose::STANDARD.encode(&ed);
            new_content.push_str(&push_data);
        },
        Err(_) => {
            return PassKeeperError::EncryptionFailed as u32;
        }
    }
    new_content.push('\n');
    // Write back to the file
    if fs::write(file_path, new_content).is_err() {
        return PassKeeperError::FileWriteFailed as u32;
    }

    PassKeeperError::Success as u32
}

#[command]
fn get_data(username: &str) -> String {
    let file_path = format!("{}/data/{}.dat", current_dir().unwrap().display(), username);
    // Check if the file exists
    if !Path::new(&file_path).exists() {
        return String::new();
    }

    let secret_data = get_secret_data(username.to_string());
    if secret_data.is_skey_empty() || secret_data.is_data_empty() {
        return String::new();
    }
    let data = secret_data.data.lines()
        .map(|line| from_str(line).unwrap_or(Value::Null))
        .filter(|v| !v.is_null())
        .collect::<Vec<Value>>();
    // Flatten arrays if any    
    let new_data: Vec<Value> = data.into_iter().flat_map(|v|
        if let Value::Array(arr) = v {
            arr.into_iter()
        } else {
            vec![v].into_iter()
        }
    ).collect();
    // Convert back to string format
    let entry_str = to_string(&new_data).unwrap_or_default();
    entry_str
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Store the AppHandle in a global static variable for later use
            APP_HANDLE.set(app.handle().clone()).unwrap();
            let window = app.get_webview_window("main").unwrap();
            let (left, top, right, bottom) = get_window_pos();
            let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x: left, y: top }));
            let width = (right - left).max(0) as u32;
            let height = (bottom - top).max(0) as u32;
            let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize { width, height }));
            window.show().unwrap();
            Ok(())
        })
        .on_window_event(|_window, event| {
            match event {
                #[allow(unused_variables)]
                tauri::WindowEvent::Resized(size) => {
                    let _ = reg_position();
                }
                #[allow(unused_variables)]
                tauri::WindowEvent::Moved(position) => {
                    let _ = reg_position();
                }
                #[allow(unused_variables)]
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    // Remove the comment below to prevent window from closing.
                    // api.prevent_close();
                }                
                _ => {}
            }
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            validate_login,
            get_data,
            save_account,
            save_data,
            modify_data,
            remove_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Retrieve secret data for a given username
fn get_secret_data(username: String) -> SecretData {
    let mut secret_data =SecretData::new();
    let file_path = format!("{}/data/{}.dat", current_dir().unwrap().display(), username);
    // Check if the file exists
    if !Path::new(&file_path).exists() {
        return secret_data;
    }
    // Read the stored content
    let stored_content = fs::read_to_string(&file_path).unwrap_or_default();
    let mut lines = stored_content.lines();
    // First line is the password
    let skey = lines.next().unwrap_or_default();
    secret_data.skey = skey.to_string();
    // Following lines are the encrypted json entries
    let encrypted_line = lines.next();
    let encrypted_data = match encrypted_line {
        Some(el) => el,
        None => {
            return secret_data;
        }
    };
    print_out!("Encrypted data line: [{}]", encrypted_data);
    let base64_decoded = general_purpose::STANDARD.decode(encrypted_data);
    let decrypted_data = decrypt_data(&base64_decoded.unwrap_or_default());
    print_out!("Decryption result: [{:?}]", decrypted_data);
    let decrypted_line = match decrypted_data {
        Ok(dl) => dl,
        Err(_) => {
            return secret_data;
        }
    };

    print_out!("Decrypted data: [{}]", decrypted_line);
    secret_data.data = decrypted_line;
    secret_data
}

/// Get window position from registry
#[allow(unused_assignments)]
fn get_window_pos() -> (i32, i32, i32, i32) {
    // Declare variables without initial values
    let mut left = 0;
    let mut top = 0;
    let mut right = 0;
    let mut bottom = 0;

    let _ = registry::check_or_create_key(HKEY_CURRENT_USER, define::REG_SUBKEY_POS);

    let reg_left = registry::read_dword("l");
    match reg_left {
        Ok(value) => {
            left = value as i32;
        },
        Err(_) => {
            left = 0;
            print_out!(">> [Window Position] Left not found, using default 0");
        },
    }

    let reg_top = registry::read_dword("t");
    match reg_top {
        Ok(value) => {
            top = value as i32;
        },
        Err(_) => {
            top = 0;
            print_out!(">> [Window Position] Top not found, using default 0");
        },
    }
    let reg_right = registry::read_dword("r");
    match reg_right {
        Ok(value) => {
            right = value as i32;
        },
        Err(_) => {
            right = left + define::MIN_WIDTH;
            print_out!(">> [Window Position] Right not found, using default {}", right);
        },
    }
    let reg_bottom = registry::read_dword("b");
    match reg_bottom {
        Ok(value) => {
            bottom = value as i32;
        },
        Err(_) => {
            bottom = top + define::MIN_HEIGHT;
            print_out!(">> [Window Position] Bottom not found, using default {}", bottom);
        },
    }

    (left, top, right, bottom)
}

/// Set window position to registry
fn set_window_pos(left: i32, top: i32, right: i32, bottom: i32) {
    let _ = registry::check_or_create_key(HKEY_CURRENT_USER, define::REG_SUBKEY_POS);

    let _ = registry::write_dword("l", left as u32);
    let _ = registry::write_dword("t", top as u32);
    let _ = registry::write_dword("r", right as u32);
    let _ = registry::write_dword("b", bottom as u32);

    print_out!(">> [Window Position] Saved: l={}, t={}, r={}, b={}", left, top, right, bottom);
}

/// Register window position on move/resize
fn reg_position() {
    if let Some(app_handle) = APP_HANDLE.get() {
        if let Some(window) = app_handle.get_webview_window("main") {
            let win = window.clone();
            std::thread::spawn(move || {
                // Because the resize/move event may be triggered multiple times,
                // I add a small delay to ensure we get the final position and size.
                std::thread::sleep(std::time::Duration::from_millis(100)); // Delay 100ms
                if let Ok(true) = win.is_maximized() {
                    print_out!(">> Window is maximized (after delay)");
                    return;
                }
                if let Ok(true) = win.is_minimized() {
                    print_out!(">> Window is minimized (after delay)");
                    return;
                }

                let outer_position = window.outer_position().unwrap();
                let outer_size = window.outer_size().unwrap();
                let inner_size = window.inner_size().unwrap();
                let frame_width = outer_size.width - inner_size.width;
                let frame_height = outer_size.height - inner_size.height;
                let left = outer_position.x;
                let top = outer_position.y;
                let right = left + outer_size.width as i32 - frame_width as i32;
                let bottom = top + outer_size.height as i32 - frame_height as i32;
                set_window_pos(left, top, right, bottom);
            });
        }
    }
}
