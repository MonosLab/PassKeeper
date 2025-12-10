// 7.6.x.x   : NaonAI 최초 버전
//             ├ ????
//             └ ????

#![allow(dead_code)]

// application info
pub const APP_NAME: &str = "PassKeeper";
pub const APP_VERSION: &str = "Ver 1.0.0";
pub const APP_VERSION_NUMBER: &str = "1.0.0";
pub const APP_INSTANCE: &str = "PassKeeper_Instance";
pub const PASSKEEPER_CLASS_NAME: &str = "PassKeeper@1";

// registry
pub const REG_KEY_RUN: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
pub const REG_WIN_PATHS: &str = r"Software\Microsoft\Windows\CurrentVersion\App Paths";
pub const REG_PRODUCT_NAME: &str = r"Software\Microsoft\Windows NT\CurrentVersion";
pub const REG_SUBKEY_CRATES: &str = r"Software\Monoslab\PassKeeper";
pub const REG_SUBKEY_POS: &str = r"Software\Monoslab\PassKeeper\Pos";
pub const REG_SUBKEY_SETTINGS: &str = r"Software\Monoslab\PassKeeper\Settings";

// minimum window size
pub const MIN_WIDTH: i32 = 600;
pub const MIN_HEIGHT: i32 = 400;