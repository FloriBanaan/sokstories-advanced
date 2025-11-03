use core::hash;
use std::collections::HashMap;
use std::fmt::format;
use std::os::raw::c_void;
use std::path;

use serde_json::from_str;
use serde_json::to_string_pretty;
use tauri::command;
use tauri_plugin_http::reqwest;
use reqwest::header::CONTENT_TYPE;
use serde_json::json;
use serde_json::Value;

use serde::Deserialize;
use serde::Serialize;

use std::fs;
use std::path::PathBuf;

use std::fs::OpenOptions;
use std::io::{Read, Write};

use std::fs::File;


#[tauri::command]
pub(crate) fn get_data_from_save(key: String) -> Result<String, String>{
    let path = get_sokstories_path()?.join(r"sokstories-advanced\save.json");
    let mut data: Value = if path.exists() {
        let contents = fs::read_to_string(&path).map_err(|e| format!("{}", e))?;
        from_str(&contents).unwrap_or_else(|_| serde_json::json!({}))
    } else {
        serde_json::json!({})
    };
    match data.get(key) {
        Some(value) => Ok(value.clone().to_string()),
        None => Err(format!("key not found")),
    }
}

pub(crate) fn write_data_to_save(key: String, value: String) -> Result<String, String> {
    let path = get_sokstories_path()?.join(r"sokstories-advanced\save.json");
    let mut data: Value = if path.exists() {
        let contents = fs::read_to_string(&path).map_err(|e| format!("{}", e))?;
        from_str(&contents).unwrap_or_else(|_| serde_json::json!({}))
    } else {
        serde_json::json!({})
    };
    data[key] = serde_json::from_str(&value).unwrap_or(serde_json::Value::String(value));
    let json_text = to_string_pretty(&data).unwrap();
    let mut file = fs::File::create(&path).map_err(|e| format!("{}", e))?;
    file.write_all(json_text.as_bytes()).map_err(|e| format!("{}", e))?;
    Ok("success".to_string())
}

pub(crate) fn get_sokstories_path() -> Result<PathBuf, String> {
    let roaming_dir = dirs_next::data_dir().expect("niet gevonden");
    let appdata_dir = roaming_dir.parent().expect("niet gevonden");
    println!("{:?}", appdata_dir);
    Ok(appdata_dir.join(r"Local\sokstories\"))
}