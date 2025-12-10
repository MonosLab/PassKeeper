# PassKeeper Architecture

## Overview

PassKeeper is a desktop password manager built with Tauri, combining a Rust backend with a vanilla JavaScript frontend.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                         Frontend (UI)                        │
│                    HTML + CSS + JavaScript                   │
├─────────────────────────────────────────────────────────────┤
│  - index.html (UI Structure)                                 │
│  - styles.css (Styling with dark mode support)               │
│  - main.js (Application logic, event handling)               │
│                                                               │
│  Components:                                                  │
│  • Unlock Screen (Master password entry)                     │
│  • Password List View (Grid layout)                          │
│  • Password Form Modal (Add/Edit)                            │
│  • Password Generator Modal (With options)                   │
│  • Search Bar                                                 │
└─────────────────────────────────────────────────────────────┘
                              ▲
                              │ Tauri IPC
                              │ (invoke commands)
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                       Backend (Rust)                         │
│                        Tauri Framework                        │
├─────────────────────────────────────────────────────────────┤
│  lib.rs (Main application, Tauri commands)                   │
│  ├── unlock_storage()                                        │
│  ├── lock_storage()                                          │
│  ├── add_password()                                          │
│  ├── get_all_passwords()                                     │
│  ├── update_password()                                       │
│  ├── delete_password()                                       │
│  ├── search_passwords()                                      │
│  └── generate_password()                                     │
│                                                               │
│  password.rs (Data structures)                               │
│  ├── Password struct                                         │
│  │   ├── id (UUID)                                          │
│  │   ├── title, username, password                          │
│  │   ├── url, notes (optional)                              │
│  │   └── timestamps                                         │
│  └── CRUD methods                                            │
│                                                               │
│  storage.rs (Data persistence)                               │
│  ├── Storage struct                                          │
│  ├── Encrypted file I/O                                      │
│  ├── JSON serialization                                      │
│  └── Local filesystem access                                 │
│                                                               │
│  crypto.rs (Encryption/Security)                             │
│  ├── Crypto struct                                           │
│  ├── AES-256-GCM encryption                                  │
│  ├── encrypt() / decrypt()                                   │
│  └── generate_password()                                     │
└─────────────────────────────────────────────────────────────┘
                              ▲
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Local File System                         │
│                                                               │
│  ~/.local/share/passkeeper/passwords.enc (Linux)             │
│  ~/Library/Application Support/passkeeper/ (macOS)           │
│  %APPDATA%\passkeeper\ (Windows)                             │
│                                                               │
│  Format: Base64-encoded, AES-GCM encrypted JSON             │
└─────────────────────────────────────────────────────────────┘
```

## Component Details

### Frontend Layer

**Technology**: Vanilla JavaScript (ES6+), HTML5, CSS3

**Responsibilities**:
- User interface rendering
- User input handling
- Form validation
- Search and filtering (client-side)
- Modal management
- Calling backend commands via Tauri IPC

**Key Features**:
- Responsive design
- Dark mode support
- Real-time search filtering
- Password visibility toggle
- Copy to clipboard functionality

### Backend Layer

**Technology**: Rust with Tauri framework

**Responsibilities**:
- Business logic
- Data encryption/decryption
- File I/O operations
- Password generation
- State management

**Key Modules**:

1. **lib.rs**: Application entry point and command handlers
2. **password.rs**: Password data model and operations
3. **storage.rs**: Persistence and file management
4. **crypto.rs**: Encryption and password generation

### Data Flow

#### Adding a Password:
```
User Input → JavaScript (main.js)
          → invoke("add_password", {...})
          → Rust command handler
          → Create Password struct
          → Storage.add_password()
          → Crypto.encrypt()
          → Write to file
          → Return success
          → Update UI
```

#### Retrieving Passwords:
```
User opens app → JavaScript loads
              → invoke("get_all_passwords")
              → Rust command handler
              → Storage.load_passwords()
              → Crypto.decrypt()
              → Return password list
              → Render in UI
```

#### Unlocking:
```
User enters master password → JavaScript
                           → invoke("unlock_storage", {password})
                           → Create Crypto instance
                           → Store in app state
                           → Return success/failure
                           → Show main screen / error
```

## Security Architecture

### Encryption Flow:
```
Master Password
     ↓
Key Derivation (⚠️ Simplified - needs upgrade)
     ↓
AES-256-GCM Key
     ↓
Encrypt with random nonce
     ↓
Base64 encode
     ↓
Write to disk
```

### Data at Rest:
- All password data encrypted before storage
- Master password never stored
- Each encryption uses unique nonce
- File format: `nonce(12 bytes) + ciphertext`

### Data in Memory:
- Passwords decrypted only when needed
- Master password cleared after key derivation
- Application lockable to clear memory

## Technology Stack

### Core Dependencies:

**Rust (Backend)**:
- `tauri` v2 - Application framework
- `aes-gcm` v0.10 - Encryption
- `serde` v1 - Serialization
- `serde_json` v1 - JSON handling
- `uuid` v1 - Unique ID generation
- `rand` v0.8 - Random number generation
- `base64` v0.22 - Encoding
- `dirs` v5 - Directory paths

**JavaScript (Frontend)**:
- Vanilla JavaScript (no frameworks)
- ES6+ features
- Tauri API for IPC

### Build Tools:
- Cargo (Rust)
- npm (JavaScript)
- Tauri CLI

## File Structure

```
PassKeeper/
├── src/                      # Frontend source
│   ├── index.html           # Main UI
│   ├── main.js              # Application logic
│   ├── styles.css           # Styling
│   └── assets/              # Static assets
│
├── src-tauri/               # Backend source
│   ├── src/
│   │   ├── lib.rs          # Main app & commands
│   │   ├── main.rs         # Entry point
│   │   ├── password.rs     # Password model
│   │   ├── storage.rs      # Persistence
│   │   └── crypto.rs       # Encryption
│   │
│   ├── Cargo.toml          # Rust dependencies
│   ├── tauri.conf.json     # Tauri config
│   └── icons/              # App icons
│
├── README.md               # Project overview
├── SECURITY.md             # Security documentation
├── TUTORIAL.md             # User guide
├── ARCHITECTURE.md         # This file
└── package.json            # Node dependencies
```

## State Management

### Application State:
```rust
struct AppState {
    storage: Mutex<Storage>
}
```

### Storage State:
```rust
struct Storage {
    crypto: Mutex<Option<Crypto>>,  // None when locked
    data_path: PathBuf
}
```

### Frontend State:
```javascript
let passwords = [];        // Current password list
let currentEditId = null;  // ID of password being edited
```

## Event Flow

### User Events → Actions:
- Search input → Filter passwords (client-side)
- Add button → Show modal
- Save form → Add/update password
- Delete button → Confirm and delete
- Lock button → Lock storage and return to unlock screen
- Generate button → Open generator modal
- View button → Copy password to clipboard

## Performance Considerations

1. **Encryption**: 
   - One-time unlock per session
   - Passwords cached in memory after unlock
   - Encryption only on save operations

2. **UI**:
   - Client-side filtering for instant results
   - No pagination needed for typical use (hundreds of passwords)
   - Grid layout for optimal space usage

3. **Storage**:
   - Single file for all passwords
   - Read once on unlock
   - Write on each change (small overhead)

## Extension Points

Future enhancements could include:

1. **Import/Export**:
   - Add commands for CSV import/export
   - Support for other password manager formats

2. **Sync**:
   - Cloud storage integration
   - Encrypted sync between devices

3. **Security**:
   - Proper key derivation (PBKDF2/Argon2)
   - Two-factor authentication
   - Biometric unlock

4. **Features**:
   - Password strength meter
   - Breach checking
   - Auto-lock timer
   - Password history
   - Secure notes
   - File attachments

5. **UI/UX**:
   - Themes
   - Customizable layouts
   - Browser extension integration
   - Mobile apps (iOS/Android via Tauri)

## Deployment

### Development:
```bash
npm run dev
```

### Production Build:
```bash
npm run build
```

### Outputs:
- Windows: `.exe` installer
- macOS: `.dmg` disk image
- Linux: `.AppImage` or `.deb`

---

This architecture provides a solid foundation for a password manager while maintaining simplicity and security.
