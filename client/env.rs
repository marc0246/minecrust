//! Abstracts the environment of the client.

use std::fs::{self, OpenOptions};
use std::io::BufReader;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

/// The version of the client.
pub const VERSION: &str = "1.18.2";

/// Collection of all environment-specific variables.
#[derive(Debug)]
pub struct Env {
    /// Path to the original `.minecraft` folder.
    pub original_config_dir: PathBuf,
    /// Directory where logs are stored.
    pub log_dir: PathBuf,
    /// Maximum log level. Lower level log records will be ignored.
    pub log_level_filter: log::LevelFilter,
    /// Cache directory.
    pub cache_dir: PathBuf,
    /// File where settings are stored.
    pub settings_path: PathBuf,
    /// The user's settings.
    pub settings: Settings,
}

impl Env {
    /// Loads the environment-specific variables.
    pub fn load() -> Self {
        let home_dir = home::home_dir().expect("couldn't find your home directory");

        #[cfg(windows)]
        let original_config_dir = home_dir.join("AppData/Roaming/.minecraft");
        #[cfg(unix)]
        let original_config_dir = home_dir.join(".minecraft");
        #[cfg(target_os = "macos")]
        let original_config_dir = home_dir.join("Library/Application Support/minecraft");

        let project_dirs = ProjectDirs::from_path(PathBuf::from("minecrust"))
            .expect("failed to retrieve home directory path");
        let config_dir = project_dirs.config_dir();

        let log_dir = config_dir.join("logs");

        fs::create_dir_all(&log_dir).expect("failed to create log directory");

        for entry in fs::read_dir(&log_dir).unwrap().flatten() {
            if let Ok(file_type) = entry.file_type() {
                let path = entry.path();

                if let Some(extension) = path.extension() {
                    if file_type.is_file() && extension == "log" {
                        fs::remove_file(path).unwrap();
                    }
                }
            }
        }

        let cache_dir = config_dir.join("cache");
        let settings_path = config_dir.join("settings.json");

        Env {
            original_config_dir,
            log_dir,
            log_level_filter: log::LevelFilter::Trace,
            cache_dir,
            settings: Settings::from_path(&settings_path),
            settings_path,
        }
    }
}

/// User settings.
#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Settings {
    /// Vertical field of view, in degrees.
    pub fov: f32,
    /// Mouse sensitivity.
    pub mouse_sensitivity: f32,
}

impl Settings {
    /// Loads the settings file from the given path.
    fn from_path(path: &Path) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("failed to open settings.json");

        serde_json::from_reader(BufReader::new(file)).unwrap_or_default()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            fov: 90.0,
            mouse_sensitivity: 1.0,
        }
    }
}
