use serde::{Deserialize, Serialize};

pub const CONFIG: &str = "assets/config.json";

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct Item {
    pub name: String,
    pub desc: String,
    pub ws: String,
    pub listen: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct Config {
    pub port: u16,
    pub item: Vec<Item>,
}