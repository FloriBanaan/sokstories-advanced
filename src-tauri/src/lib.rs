mod server;
mod savefile;
mod story;

use core::hash;
use std::collections::HashMap;
use std::fmt::format;
use std::os::raw::c_void;
use std::path;

use serde_json::from_str;
use serde_json::to_string_pretty;
use serde_json::Map;
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

use server::post_and_get_info;
use server::fetch_data;

use savefile::get_data_from_save;
use savefile::write_data_to_save;
use savefile::get_sokstories_path;

use story::index_story;

const URL: &str = "http://127.0.0.1:5000";
const SOKSTORIES_URL: &str = "https://sok-stories.com/api/";

#[derive(serde::Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StoryValues {
    Text(String),
    Usize(usize),
    I64(i64),
    F64(f64),
    Map(HashMap<String, StoryValues>),
    Vec(Vec<StoryValues>),
    VecWithMap(Vec<HashMap<String, StoryValues>>),
}

impl StoryValues {
    fn as_str(&self) -> Option<&str> {
        if let StoryValues::Text(s) = self {
            Some(s)
        } else {
            None
        }
    }
}

// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

// #[tauri::command]
// async fn test_story(story: &str) -> Result<String, String> {
//     let data_to_server = serde_json::json!({"story_name":story});
//     if let Ok(maker_name) = post_and_get_info(format!("{URL}/sokstories/api/getstory").as_str(), &data_to_server).await {
//         let parsed = json_to_vec(maker_name).await?;
//         Ok(format!("{:?}", parsed))
//     } else {
//         Err(String::from("story niet gevonden"))
//     }
// }

// #[tauri::command]
// async fn json_to_vec(json_data: String) -> Result<Vec<String>, String> {
//     let parsed: serde_json::Value = serde_json::from_str(&json_data)
//         .map_err(|e| format!("JSON parse error: {}", e))?;

//     let items = parsed.as_array()
//         .ok_or("Expected array")?
//         .iter()
//         .filter_map(|inner| inner.as_array())
//         .filter_map(|arr| arr.get(0))
//         .filter_map(|val| val.as_str())
//         .map(|s| s.to_string())
//         .collect::<Vec<_>>();

//     Ok(items)
// }

#[tauri::command]
async fn fetch_story(id: &str) -> Result<String, String> {
    if let Ok(story) = fetch_data(format!("{SOKSTORIES_URL}/getStory.php?unique_id={id}").as_str()).await {
        let story = index_story(story).await?;
        let story_map: HashMap<String, StoryValues> = serde_json::from_str(&story).unwrap();
        // Ok(story)
        match get_data_from_save("played".to_string()) {
            Ok(mut played) => {
                let mut played_vec: Vec<String> = serde_json::from_str(&played).unwrap();
                let unique_id = story_map["unique_id"].as_str().unwrap_or("").to_string();
                if played_vec.contains(&unique_id) {
                    Ok(story)
                }
                else {
                    played_vec.push(unique_id);
                    match write_data_to_save("played".to_string(), serde_json::to_string(&played_vec).unwrap()) {
                        Ok(success) => {
                            Ok(story)
                        }
                        Err(error) => {
                            Ok(story)
                        }
                    }
                }
            }
            Err(error) => {
                Ok(story)
            }
        }
    } else {
        Err(String::from("story niet gevonden"))
    }
}

#[tauri::command]
async fn request_token(request_new: &str) -> Result<String, String> {
    let name_and_id =  get_sokstories_name_and_id();
    let maker_id = &name_and_id[1];
    let data_to_server = serde_json::json!({"maker_id":maker_id, "request_new":request_new});
    if let Ok(token_and_code) = post_and_get_info(format!("{URL}/sokstories/api/requesttoken").as_str(), &data_to_server).await {
        let token_and_code_vec: Vec<&str> = token_and_code.split(",").collect();
        let token = &token_and_code_vec[0].trim().replace('"', "");
        if token == "token already exists" {
            return Ok("token already exists".to_string());
        }
        println!("{:?}", token_and_code_vec);
        let code = &token_and_code_vec[1].trim().replace('"', "");
        match write_data_to_save("token".to_string(), token.to_string()){
            Ok(token) => {
                Ok(code.to_string())
            }
            Err(error) => {
                Ok("unable to save token".to_string())
            }
        }
        // Ok(token)
    } else {
        Ok("unable to generate token".to_string())
    }
}

#[tauri::command]
async fn request_verification() -> Result<String, String> {
    match get_data_from_save("token".to_string()) {
        Ok(token) => {
            let name_and_id = get_sokstories_name_and_id();
            let maker_id = &name_and_id[1];
            let data_to_server = serde_json::json!({"maker_id":maker_id, "token":token});
            match post_and_get_info(format!("{URL}/sokstories/api/requestverification").as_str(), &data_to_server).await {
                Ok(res) => {
                    Ok(res)
                }
                Err(error) => {
                    Ok("unable to connect to server".to_string())
                }
            }
        }
        Err(error) => {
            Ok("save not found".to_string())
        }
    }
}

#[tauri::command]
async fn fetch_stories_by_recency(min: i64, max:i64) -> Result<String, String> {
    let data_to_server = serde_json::json!({"min":min, "max":max});
    match post_and_get_info(format!("{URL}/sokstories/api/loadstories").as_str(), &data_to_server).await {
        Ok(res) => {
            // println!("{:?}", res);
            // let res_to_vec: Vec<Vec<StoryValues>> = serde_json::from_str(&res).unwrap();
            Ok(res)
        }
        Err(error) => {
            // Ok([[StoryValues::Text("unable to connect to server".to_string())].to_vec()].to_vec())
            Ok("unable to connect to server".to_string())
        }
    }
}

#[tauri::command]
async fn fetch_stories_by_name(name: String) -> Result<String, String> {
    let data_to_server = serde_json::json!({"name":name});
    match post_and_get_info(format!("{URL}/sokstories/api/getstoriesbyname").as_str(), &data_to_server).await {
        Ok(res) => {
            Ok(res)
        }
        Err(error) => {
            Ok("unable to connect to server".to_string())
        }
    }
}

#[tauri::command]
async fn fetch_makers_by_name(name: String) -> Result<String, String> {
    let data_to_server = serde_json::json!({"name":name});
    match post_and_get_info(format!("{URL}/sokstories/api/getmakersbyname").as_str(), &data_to_server).await {
        Ok(res) => {
            Ok(res)
        }
        Err(error) => {
            Ok("unable to connect to server".to_string())
        }
    }
}

#[tauri::command]
fn create_category(name: String) -> Result<String, String> {
    let mut empty_category: HashMap<String, StoryValues> = HashMap::new();
    empty_category.insert("name".to_string(), StoryValues::Text(name));
    empty_category.insert("stories".to_string(), StoryValues::Vec(vec![]));
    match get_data_from_save("categories".to_string()) {
        Ok(mut categories) => {
            let mut categories_vec: Vec<StoryValues> = serde_json::from_str(&categories).unwrap();
            categories_vec.push(StoryValues::Map(empty_category));
            match write_data_to_save("categories".to_string(), serde_json::to_string(&categories_vec).unwrap()) {
                Ok(success) => {
                    Ok("success".to_string())
                }
                Err(error) => {
                    Ok("could not write to save".to_string())
                }
            }
        }
        Err(error) => {
            Ok("save not found".to_string())
        }
    }
}

#[tauri::command]
fn remove_category(name: String, ) -> Result<String, String> {
    match get_data_from_save("categories".to_string()) {
        Ok(categories) => {
            let mut categories_vec: Vec<Value> = serde_json::from_str(&categories).unwrap();
            categories_vec.retain(|map| map.get("name") != Some(&Value::String(name.to_string())));
            match write_data_to_save("categories".to_string(), serde_json::to_string(&categories_vec).unwrap()) {
                Ok(succes) => {
                    Ok("success".to_string())
                }
                Err(error) => {
                    Ok("could not write to save".to_string())
                }
            }
        }
        Err(error) => {
            Ok("save not found".to_string())
        }
    }
}

#[tauri::command]
fn add_story_to_category(category_name: String, story: String) -> Result<String, String> {
    match get_data_from_save("categories".to_string()) {
        Ok(mut categories) => {
            let mut categories_vec: Vec<Value> = serde_json::from_str(&categories).unwrap();
            for category in &mut categories_vec {
                println!("{}", category["name"]);
                if category["name"] == category_name {
                    if category["stories"].as_array().unwrap().iter().any(|s| s.as_str().unwrap() == story) {
                        return Ok("story is already in the category".to_string());
                    }
                    else {
                        if let Some(stories) = category["stories"].as_array_mut() {
                            stories.push(Value::String(story));
                        }
                        match write_data_to_save("categories".to_string(), serde_json::to_string(&categories_vec).unwrap()) {
                            Ok(success) => {
                                return Ok("success".to_string());
                            }
                            Err(error) => {
                                return Ok("could not write to save".to_string());
                            }
                        }
                    }
                }
            }
            Ok("category not found".to_string())
        }
        Err(error) => {
            Ok("save not found".to_string())
        }
    }
}

#[tauri::command]
fn remove_story_from_category(category_name: String, story: String) -> Result<String, String> {
    match get_data_from_save("categories".to_string()) {
        Ok(mut categories) => {
            let mut categories_vec: Vec<Value> = serde_json::from_str(&categories).unwrap();
            for category in &mut categories_vec {
                if category["name"] == category_name {
                    if let Some(stories_val) = category.get_mut("stories") {
                        if let Some(stories) = stories_val.as_array_mut() {
                            stories.retain(|x| x.as_str() != Some(&story));
                            println!("{:?}", categories_vec);
                            match write_data_to_save("categories".to_string(), serde_json::to_string(&categories_vec).unwrap()) {
                                Ok(success) => {
                                    return Ok("success".to_string());
                                }
                                Err(error) => {
                                    return Ok("could not write to save".to_string());
                                }
                            }
                        }
                    }
                }
            }
            Ok("category not found".to_string())
        }
        Err(error) => {
            Ok("save not found".to_string())
        }
    }
}

#[tauri::command]
async fn post_comment(comment: String, story_id: String) -> Result<String, String> {
    let name_and_id = get_sokstories_name_and_id();
    let maker_id = &name_and_id[1];
    let data_to_server = serde_json::json!({"comment":comment, "story_id":story_id, "maker_id":maker_id});
    match post_and_get_info(format!("{URL}/sokstories/api/postcomment").as_str(), &data_to_server).await {
        Ok(res) => {
            Ok(res)
        }
        Err(error) => {
            Ok("could not connect to server".to_string())
        }
    }
}

#[tauri::command]
async fn fetch_comments(story_id: String) -> Result<String, String> {
    let name_and_id = get_sokstories_name_and_id();
    let maker_id = &name_and_id[1];
    let data_to_server = serde_json::json!({"story_id":story_id, "maker_id":maker_id});
    match post_and_get_info(format!("{URL}/sokstories/api/getcomments").as_str(), &data_to_server).await {
        Ok(res) => {
            Ok(res)
        }
        Err(error) => {
            Ok("could not connect to server".to_string())
        }
    }
}

#[tauri::command]
fn init_save_folder() -> String {
    match get_sokstories_path() {
        Ok(path) => {
            match create_sokstories_advanced_files(path) {
                Ok(succes) => {
                    "success".to_string()
                }
                Err(error) => {
                    "error".to_string()
                }
            }
        }
        Err(error) => {
            "sokstories not found".to_string()
        }
    }
}

fn create_sokstories_advanced_files(path: PathBuf) -> Result<String, String> {
    let mut save_folder = path.join(r"sokstories-advanced\");
    let folder = fs::create_dir(&save_folder);
    if !&save_folder.join("save.json").exists() {
        let mut file = File::create(&save_folder.join("save.json")).expect("could not create save");
        let empty_data = r#"{"token": "", "played": [], "categories": []}"#;
        file.write_all(empty_data.as_bytes()).expect("could not write to save");
    }
    Ok("success".to_string())
}

#[tauri::command]
fn get_sokstories_name_and_id() -> Vec<String> {
    match get_sokstories_path() {
        Ok(path) => {
            println!("{:?}", path);
            let savefile_path = path.join("savefile.ini");
            let savefile = fs::read_to_string(savefile_path).expect("niet gevonden");
            let mut id: String = "".to_string();
            let mut name: String = "".to_string();

            for element in savefile.lines() {
                if element.contains("maker id=") {
                    id = element.replace("maker id=", "").replace('"', "");
                    println!("{}", id);
                }
                if element.contains("maker name=") {
                    name = element.replace("maker name=", "").replace('"', "");
                    println!("{}", name);
                }
            }
            [name.to_string(), id.to_string()].to_vec()
        }
        Err(error) => {
            ["sokstories not found".to_string()].to_vec()
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![fetch_story, get_sokstories_name_and_id, request_token, init_save_folder, request_verification, fetch_stories_by_recency, create_category, add_story_to_category, remove_story_from_category, remove_category, fetch_stories_by_name, fetch_makers_by_name, post_comment, fetch_comments])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
