# PassKeeper

## Features

* A program for managing passwords.

### Installation

You need to install the required modules once using 'pnpm install' in the project's root directory.

### Distribution Folder Structure   

📂root   
├─📂data   
│   └─📄user.dat   
└─📄passkeeper.exe   

## Development Tool Versions

* RUST Version : v.1.90.0 (1159e78c4 2025-09-14)
* TAURI Version : v.2.9.3
* TAURI-CLI Version : v.2.9.5
* VITE Version : v.7.2.4

## Version Information

### v.1.0.0

* Initial version

# Key Development Notes

## crypto   
* It is necessary to redefine the encrypt_data, decrypt_data, and hash_data functions.   
> The encryption and hash parts must be added and developed individually. (By default, data is stored without encryption)   
 (* Location: src-tauri > src > module > crypto.rs )   

## tauri.conf.json   
* tauri.conf.json : References a locally saved schema file. (src-tauri/schema/config.schema.json)   
* If issues occur after updating Tauri version with local reference, download and use the file from    https://github.com/tauri-apps/tauri/blob/dev/crates/tauri-schema-generator/schemas/config.schema.json   

## Uncaught TypeError: Cannot read properties of undefined (reading 'core')   
* Occurrence location : const { invoke } = window.**TAURI**.core;   
* If this error occurs at the above location in devtools, you need to rebuild once more according to the comments in build.rs.   
