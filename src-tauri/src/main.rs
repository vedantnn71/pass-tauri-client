// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(drain_filter)]

use serde::{Deserialize, Serialize};
use std::process::Command;
use std::{
    env::{self, home_dir},
    ffi::OsString,
    fs, io,
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    password: String,
    filename: String,
    username: Option<String>,
    email: Option<String>,
    url: Option<String>,
    notes: Option<String>,
    phone: Option<String>,
}

fn store_path() -> Result<PathBuf, String> {
    let mut fallback_path = home_dir().expect("[panic]: unable to find the home directory");
    fallback_path.push(".password-store");

    let path = match env::var_os("PASSWORD_STORE_DIR") {
        Some(dir) => dir,
        None => fallback_path.into(),
    };

    Ok(PathBuf::from(path))
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_folders() -> Result<Vec<String>, String> {
    let path = store_path()?;

    let mut store: Vec<PathBuf> = fs::read_dir(path)
        .expect("[panic]: unable to find the password store")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    store.drain_filter(|p| p.is_file() || p.file_name().unwrap() == &OsString::from(".git"));

    let store = store
        .into_iter()
        .map(|item| {
            String::from(
                item.file_name()
                    .expect("[error]: to get the folder names")
                    .to_str()
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();

    return Ok(store);
}

fn get_password(folder: &str, file: &str) -> Item {
    let output = Command::new("pass")
        .arg("show")
        .arg(format!("{}/{}", folder, file))
        .output()
        .expect("[panic]: unable to run pass");

    let output =
        String::from_utf8(output.stdout).expect("[panic]: unable to convert output to string");

    // first line is the password
    let password = output.lines().next().unwrap();

    // rest are the metadata
    let metadata = output.lines().skip(1).collect::<Vec<_>>();

    let mut username: Option<String> = None;
    let mut email: Option<String> = None;
    let mut url: Option<String> = None;
    let mut notes: Option<String> = None;
    let mut phone: Option<String> = None;

    // incasesensitevly compare and get the metadata
    for line in metadata {
        if line.to_lowercase().starts_with("username:") {
            username = Some(
                line.split(":")
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join(":")
                    .trim()
                    .to_string(),
            );
        } else if line.to_lowercase().starts_with("email:") {
            email = Some(
                line.split(":")
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join(":")
                    .trim()
                    .to_string(),
            );
        } else if line.to_lowercase().starts_with("url:") {
            url = Some(
                line.split(":")
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join(":")
                    .trim()
                    .to_string(),
            );
        } else if line.to_lowercase().starts_with("notes:") {
            notes = Some(
                line.split(":")
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join(":")
                    .trim()
                    .to_string(),
            );
        } else if line.to_lowercase().starts_with("phone:") {
            phone = Some(
                line.split(":")
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join(":")
                    .trim()
                    .to_string(),
            );
        }
    }

    return Item {
        password: password.to_string(),
        username,
        email,
        url,
        notes,
        phone,
        filename: file.to_string(),
    };
}

#[tauri::command]
fn get_passwords(folder: &str) -> Result<Vec<Item>, String> {
    let mut path = store_path()?;

    path.push(folder);

    let mut store: Vec<PathBuf> = fs::read_dir(path)
        .map_err(|err| err.to_string())?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .map_err(|err| err.to_string())?;

    store.drain_filter(|p| p.is_dir() || p.file_name().unwrap().to_str().unwrap().starts_with("."));

    let store = store
        .into_iter()
        .map(|item| {
            String::from(
                item.file_name()
                    .expect("[error]: to get the folder names")
                    .to_str()
                    .unwrap()
                    .trim_end_matches(".gpg"),
            )
        })
        .collect::<Vec<_>>();

    let store:Vec<Item> = store
        .into_iter()
        .map(|item| get_password(folder, &item))
        .collect::<Vec<_>>();

    println!("store: {:?}", store);

    return Ok(store);
}

#[tauri::command]
fn add_folder(name: &str) -> Result<(), String> {
    let mut path = store_path()?;

    path.push(name);
    fs::create_dir(path).map_err(|err| err.to_string())?;

    return Ok(());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_folders, get_passwords, add_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
