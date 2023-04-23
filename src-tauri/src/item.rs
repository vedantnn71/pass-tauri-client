use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    password: String,
    filename: String,
    username: Option<String>,
    email: Option<String>,
    url: Option<String>,
    notes: Option<String>,
    phone: Option<String>,
}

pub fn get_item(folder: &str, file: &str) -> Item {
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
