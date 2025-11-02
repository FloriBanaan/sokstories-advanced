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


pub(crate) async fn post_and_get_info<T: serde::Serialize>(
    url: &str,
    data: &T,
) -> Result<String, String> {
    let client = tauri_plugin_http::reqwest::Client::new();
    let body = serde_json::to_string(data).map_err(|e| e.to_string())?;
    let res = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(res.text().await.map_err(|e| e.to_string())?)
}

pub(crate) async fn fetch_data(url: &str) -> Result<String, String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?;
    let body = response.text().await.map_err(|e| e.to_string())?;
    Ok(body)
}