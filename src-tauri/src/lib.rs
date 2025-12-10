mod crypto;
mod password;
mod storage;

use password::Password;
use storage::Storage;
use std::sync::Mutex;
use tauri::State;

// Global storage state
struct AppState {
    storage: Mutex<Storage>,
}

#[tauri::command]
fn unlock_storage(master_password: String, state: State<AppState>) -> Result<bool, String> {
    let storage = state.storage.lock().unwrap();
    storage.unlock(&master_password)?;
    Ok(true)
}

#[tauri::command]
fn is_unlocked(state: State<AppState>) -> Result<bool, String> {
    let storage = state.storage.lock().unwrap();
    Ok(storage.is_unlocked())
}

#[tauri::command]
fn lock_storage(state: State<AppState>) -> Result<(), String> {
    let storage = state.storage.lock().unwrap();
    storage.lock();
    Ok(())
}

#[tauri::command]
fn add_password(
    title: String,
    username: String,
    password: String,
    url: Option<String>,
    notes: Option<String>,
    state: State<AppState>,
) -> Result<Password, String> {
    let storage = state.storage.lock().unwrap();
    let pwd = Password::new(title, username, password, url, notes);
    storage.add_password(pwd)
}

#[tauri::command]
fn get_all_passwords(state: State<AppState>) -> Result<Vec<Password>, String> {
    let storage = state.storage.lock().unwrap();
    storage.get_all_passwords()
}

#[tauri::command]
fn get_password(id: String, state: State<AppState>) -> Result<Option<Password>, String> {
    let storage = state.storage.lock().unwrap();
    storage.get_password(&id)
}

#[tauri::command]
fn update_password(
    id: String,
    title: String,
    username: String,
    password: String,
    url: Option<String>,
    notes: Option<String>,
    state: State<AppState>,
) -> Result<Password, String> {
    let storage = state.storage.lock().unwrap();
    storage.update_password(&id, title, username, password, url, notes)
}

#[tauri::command]
fn delete_password(id: String, state: State<AppState>) -> Result<(), String> {
    let storage = state.storage.lock().unwrap();
    storage.delete_password(&id)
}

#[tauri::command]
fn search_passwords(query: String, state: State<AppState>) -> Result<Vec<Password>, String> {
    let storage = state.storage.lock().unwrap();
    storage.search_passwords(&query)
}

#[tauri::command]
fn generate_password(length: usize, use_symbols: bool, use_numbers: bool, use_uppercase: bool) -> Result<String, String> {
    crypto::generate_password(length, use_symbols, use_numbers, use_uppercase)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let storage = Storage::new().expect("Failed to initialize storage");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            storage: Mutex::new(storage),
        })
        .invoke_handler(tauri::generate_handler![
            unlock_storage,
            is_unlocked,
            lock_storage,
            add_password,
            get_all_passwords,
            get_password,
            update_password,
            delete_password,
            search_passwords,
            generate_password,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
