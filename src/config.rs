use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(toml::from_str(&fs::read_to_string("Config.toml")?)?)
    }
    pub fn write(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let s = toml::to_string(&config)?;

        let mut f = File::create("Config.toml")?;
        write!(f, "{s}")?;

        Ok(())
    }
}
