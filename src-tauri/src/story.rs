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

use crate::StoryValues;

pub(crate) async fn index_story(story: String) -> Result<String, String> {
    let cleaned = story.replace(" \\r\\n", ", ").trim().to_string();
    let mut story_map: HashMap<String, StoryValues> = string_to_map(&cleaned)?;
    for key in ["0", "1", "2", "3", "4"] {
        story_map.remove(key);
    }
    let mut story_file = story_map["file"].clone();
    let mut story_file_vec: Vec<&str> = Vec::new();
    if let Some(s) = story_file.as_str() {
        story_file_vec = s.split(", ").collect();
    }
    story_map.insert("icon".to_string(), StoryValues::Text(story_file_vec[0].to_string()));
    story_file_vec.remove(0);
    story_file_vec.remove(0);
    let amounts: Vec<&str> = story_file_vec[0].split(" ").collect();
    let object_amount: usize = amounts[0].parse().unwrap();
    let rule_amount: usize = amounts[1].parse().unwrap();
    let transition_amount: usize = amounts[2].parse().unwrap();
    let room_amount: usize = amounts[3].parse().unwrap();
    story_file_vec.remove(0);
    let mut objects: Vec<HashMap<String, StoryValues>> = Vec::new();
    for i in 0..object_amount {
        objects.push(HashMap::new());
        if story_file_vec[0].parse::<i32>().unwrap() == 0 {
            objects[i].insert("mobility".to_string(), StoryValues::Text("movable".to_string()));
        } else {
            objects[i].insert("mobility".to_string(), StoryValues::Text("static".to_string()));
        }
        story_file_vec.remove(0);
    }
    let mut rules:Vec<HashMap<String, StoryValues>> = Vec::new();
    for i in 0..rule_amount {
        rules.push(HashMap::new());
        let rule_data: Vec<i64> = story_file_vec[0].split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
        rules[i].insert("pos1".to_string(), StoryValues::I64(rule_data[0]));
        rules[i].insert("pos2".to_string(), StoryValues::I64(rule_data[1]));
        rules[i].insert("pos3".to_string(), StoryValues::I64(rule_data[2]));
        rules[i].insert("pos4".to_string(), StoryValues::I64(rule_data[3]));
        rules[i].insert("pos5".to_string(), StoryValues::I64(rule_data[4]));
        if rule_data[5] == 0 {
            rules[i].insert("condition".to_string(), StoryValues::Text("tick".to_string()));
        } else {
            rules[i].insert("condition".to_string(), StoryValues::Text("click".to_string()));
        }
        rules[i].insert("sound".to_string(), StoryValues::I64(rule_data[6]));
        story_file_vec.remove(0);
    }
    story_map.insert("rules".to_string(), StoryValues::VecWithMap(rules));
    let mut transitions:Vec<HashMap<String, StoryValues>> = Vec::new();
    for i in 0..transition_amount {
        transitions.push(HashMap::new());
        let transition_data: Vec<i64> = story_file_vec[0].split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
        transitions[i].insert("pos1".to_string(), StoryValues::I64(transition_data[0]));
        transitions[i].insert("pos2".to_string(), StoryValues::I64(transition_data[1]));
        if transition_data[2] == -2 {
            transitions[i].insert("goto".to_string(), StoryValues::Text("next".to_string()));
        } else if transition_data[2] == -1 {
            transitions[i].insert("goto".to_string(), StoryValues::Text("prev".to_string()));
        } else {
            transitions[i].insert("goto".to_string(), StoryValues::I64(transition_data[2]));
        }
        if transition_data[3] == 0 {
            transitions[i].insert("condition".to_string(), StoryValues::Text("tick".to_string()));
        } else {
            transitions[i].insert("condition".to_string(), StoryValues::Text("click".to_string()));
        }
        if transition_data[4] == 0 {
            transitions[i].insert("take".to_string(), StoryValues::Text("left".to_string()));
        } else if transition_data[4] == 1 {
            transitions[i].insert("take".to_string(), StoryValues::Text("right".to_string()));
        } else {
            transitions[i].insert("take".to_string(), StoryValues::Text("none".to_string()));
        }
        story_file_vec.remove(0);
    }
    story_map.insert("transitions".to_string(), StoryValues::VecWithMap(transitions));
    let mut rooms:Vec<HashMap<String, StoryValues>> = Vec::new();
    for i  in 0..room_amount {
        rooms.push(HashMap::new());
        let room_data: Vec<i64> = story_file_vec[0].split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
        if room_data[1] == 0 {
            rooms[i].insert("state".to_string(), StoryValues::Text("persistent".to_string()));
        } else {
            rooms[i].insert("state".to_string(), StoryValues::Text("restarts".to_string()));
        }
        let room_object_amount: usize = room_data[0] as usize;
        rooms[i].insert("objects".to_string(), StoryValues::Vec(Vec::new()));
        story_file_vec.remove(0);
        for j in 0..room_object_amount {
            if let Some(objects) = rooms[i].get_mut("objects") {
                push_room_object_map(objects, HashMap::new());
                let room_object_data: Vec<f64> = story_file_vec[0].split(' ').map(|s| s.parse::<f64>().unwrap()).collect();
                insert_into_room_object(&mut rooms, i, j, "posx", StoryValues::F64(room_object_data[0]));
                insert_into_room_object(&mut rooms, i, j, "posy", StoryValues::F64(room_object_data[1]));
                insert_into_room_object(&mut rooms,i, j, "id", StoryValues::I64(room_object_data[2] as i64));
                story_file_vec.remove(0);
            }
        }
    }
    story_map.insert("rooms".to_string(), StoryValues::VecWithMap(rooms));
    for i in 0..object_amount {
        objects[i].insert("img".to_string(), StoryValues::Text(story_file_vec[0].to_string()));
        story_file_vec.remove(0);
    }
    story_map.insert("objects".to_string(), StoryValues::VecWithMap(objects));
    story_map.remove("file");
    let story_indexed = map_to_string(&story_map);
    Ok(story_indexed)
}

fn push_room_object_map(objects: &mut StoryValues, new_map: HashMap<String, StoryValues>) {
    if let StoryValues::Vec(vec) = objects {
        vec.push(StoryValues::Map(new_map));
    }
}

fn insert_into_room_object(rooms: &mut Vec<HashMap<String, StoryValues>>, room_index: usize, object_index: usize, key: &str, value: StoryValues) {
    if let Some(StoryValues::Vec(objects_vec)) = rooms[room_index].get_mut("objects") {
        if let Some(StoryValues::Map(object_map)) = objects_vec.get_mut(object_index) {
            object_map.insert(key.to_string(), value);
        }
    }
}

fn string_to_map(json_str: &str) -> Result<HashMap<String, StoryValues>, String> {
    let map: HashMap<String, StoryValues> =
        serde_json::from_str(json_str).map_err(|e| e.to_string())?;
    Ok(map)
}

fn map_to_string(map: &HashMap<String, StoryValues>) -> String {
    serde_json::to_string(map).unwrap()
}