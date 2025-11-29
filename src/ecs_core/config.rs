use bevy_ecs::prelude::Resource;
use serde::Deserialize;

#[derive(Debug, Deserialize, Resource, Clone)]
pub struct AppConfig {
    pub texture_pack: String,
}

pub fn load_config() -> AppConfig {
    let config_path = "config.ron";
    let config_file = std::fs::read_to_string(config_path)
        .unwrap_or_else(|_| panic!("Failed to read config file at {}", config_path));

    let config: AppConfig = ron::from_str(&config_file)
        .unwrap_or_else(|e| panic!("Failed to parse config file: {}", e));

    config
}
