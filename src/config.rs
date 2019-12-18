use serde::Deserialize;
use std::fs::read_to_string;

static CONFIG_PATH: &str = "config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub security: SecurityConfig,
    pub database: DatabaseConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct SecurityConfig {}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: Option<u32>,
}

// FIXME: Actually handle `path`.
pub fn load(_path: Option<&str>) -> Config {
    let content = read_to_string(CONFIG_PATH).unwrap();
    toml::from_str(&content).unwrap()
}
