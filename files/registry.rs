//! Windows registry read helpers for Game Maker Studio 2.

pub const APP_KEY: &str = "Software\\Game\\Game Maker Studio 2";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Game Maker Studio 2";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Game Maker Studio 2") },
        RegValue { name: "Version",     value: String::from("v3.2.6") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Game Maker Studio 2") },
        RegValue { name: "Publisher",       value: String::from("Game") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Game Maker Studio 2\\uninstall.exe\"") },
    ]
}
