use std::{path::PathBuf, env};
use tauri::api::path::home_dir;

pub fn store_path() -> Result<PathBuf, String> {
    let mut fallback_path = home_dir().expect("[panic]: unable to find the home directory");
    fallback_path.push(".password-store");

    let path = match env::var_os("PASSWORD_STORE_DIR") {
        Some(dir) => dir,
        None => fallback_path.into(),
    };

    return Ok(PathBuf::from(path));
}
