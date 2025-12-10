# PassKeeper 🔐

A secure password management application built with Tauri and Rust.

## Features

- **Secure Password Storage**: All passwords are encrypted using AES-256-GCM encryption
- **Master Password Protection**: Access your passwords with a single master password
- **Password Generator**: Create strong, random passwords with customizable options
- **Search Functionality**: Quickly find passwords by title, username, or URL
- **Cross-Platform**: Built with Tauri, works on Windows, macOS, and Linux
- **Local Storage**: All data is stored locally on your machine, not in the cloud
- **Modern UI**: Clean and intuitive user interface

## Technology Stack

- **Backend**: Rust with Tauri framework
- **Frontend**: HTML, CSS, JavaScript (Vanilla)
- **Encryption**: AES-256-GCM
- **Storage**: Local filesystem with encrypted data

## Installation

### Prerequisites

- Node.js (v16 or higher)
- Rust (latest stable version)
- System dependencies for Tauri (see [Tauri Prerequisites](https://tauri.app/start/prerequisites/))

### Build from Source

1. Clone the repository:
```bash
git clone https://github.com/MonosLab/PassKeeper.git
cd PassKeeper
```

2. Install dependencies:
```bash
npm install
```

3. Run in development mode:
```bash
npm run dev
```

4. Build for production:
```bash
npm run build
```

## Usage

1. **First Time Setup**: When you first launch PassKeeper, enter a master password to unlock the application. This password will be used to encrypt and decrypt your stored passwords.

2. **Adding Passwords**: Click the "Add Password" button to create a new password entry. Fill in the required fields (title, username, password) and optional fields (URL, notes).

3. **Generating Passwords**: Click the "Generate" button when adding or editing a password to create a strong random password. Customize the length and character types to suit your needs.

4. **Viewing Passwords**: Click "View" on any password card to see the actual password and copy it to your clipboard.

5. **Editing Passwords**: Click "Edit" to modify an existing password entry.

6. **Searching**: Use the search bar to quickly find passwords by title, username, or URL.

7. **Locking**: Click the "Lock" button to lock the application when you're done. You'll need to enter your master password again to unlock it.

## Security

- All passwords are encrypted using AES-256-GCM before being stored
- The master password is never stored; it's used only to derive the encryption key
- Data is stored locally in your system's data directory
- No network requests are made; all data stays on your device

**Important Security Note**: The current implementation uses a simplified key derivation method for demonstration purposes. For production use, the key derivation should be upgraded to use PBKDF2 (with 100,000+ iterations) or Argon2id with proper salt generation to protect against dictionary attacks.

## Data Location

PassKeeper stores encrypted data in your system's data directory:
- **Linux**: `~/.local/share/passkeeper/`
- **macOS**: `~/Library/Application Support/passkeeper/`
- **Windows**: `%APPDATA%\passkeeper\`

## Development

### Project Structure

```
PassKeeper/
├── src/                    # Frontend source files
│   ├── index.html         # Main HTML file
│   ├── main.js            # JavaScript logic
│   └── styles.css         # Styling
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── lib.rs         # Main Tauri commands
│   │   ├── crypto.rs      # Encryption/decryption logic
│   │   ├── password.rs    # Password data structure
│   │   └── storage.rs     # File storage logic
│   └── Cargo.toml         # Rust dependencies
└── package.json           # Node.js dependencies
```

### Available Commands

- `npm run dev` - Start development server with hot reload
- `npm run build` - Build production-ready application
- `npm run tauri` - Run Tauri CLI commands

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

See [LICENSE](LICENSE) file for details.

## Disclaimer

This is a personal password manager. While it uses industry-standard encryption, please use it at your own risk. Always keep backups of important passwords.
