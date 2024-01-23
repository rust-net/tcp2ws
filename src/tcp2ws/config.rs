use std::{path::PathBuf, str::FromStr, env};

use serde::{Deserialize, Serialize};

pub const CONFIG: &str = "tcp2ws/config.json";

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct Item {
    pub name: String,
    pub desc: String,
    pub ws: String,
    pub listen: String,
    pub udp: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct Config {
    pub port: u16,
    pub item: Vec<Item>,
}

pub fn get_config() -> std::path::PathBuf {
    #[cfg(windows)]
    let temp = PathBuf::from_str(&env::var("userprofile").unwrap()).unwrap();
    #[cfg(not(windows))]
    let temp = PathBuf::from_str(&env::var("HOME").unwrap()).unwrap();
    let config = temp.join(CONFIG);
    let Some(parent) = config.parent() else {
        return "./tcp2ws.json".into();
    };
    if !parent.exists() {
        macro_log::d!("Creating dir {:?}", parent);
        std::fs::create_dir(parent).unwrap();
    }
    config
}