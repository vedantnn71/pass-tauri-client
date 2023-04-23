// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(drain_filter)]

use pass_client::{
    item::{get_item, Item},
    path::store_path,
};
use serde::{Deserialize, Serialize};
use std::{ffi::OsString, fs, io, path::PathBuf, process::Command};

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

    let store: Vec<Item> = store
        .into_iter()
        .map(|item| get_item(folder, &item))
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
        .invoke_handler(tauri::generate_handler![
            get_folders,
            get_passwords,
            add_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
