use log::error;
use std::fs;
use std::path::Path;

pub fn load_token() -> String {
    match fs::read_to_string("settings.json") {
        Ok(contents) => {
            let settings: serde_json::Value =
                serde_json::from_str(&contents).expect("Invalid JSON in settings.json");
            settings["token"]
                .as_str()
                .expect("Token not found in settings.json")
                .to_string()
        }
        Err(_) => {
            error!("Could not locate settings file. Make sure it is in the same directory");
            std::process::exit(1);
        }
    }
}

pub fn sanitize_filename(name: &str) -> String {
    name.replace(&['\\', '/', ':', '*', '?', '"', '<', '>', '|'][..], "")
        .replace(' ', "_")
}

pub fn ensure_emotes_directory() -> std::path::PathBuf {
    let path = Path::new("./emotes");
    fs::create_dir_all(path).unwrap();
    path.to_path_buf()
}
