use std::sync::OnceLock;
use anyhow::Result;
use std::fs;
use serde::Deserialize;
use crate::config::schema::audio::AudioConfig;
use crate::config::schema::input::InputConfig;
use crate::config::schema::model::ModelConfig;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub audio: AudioConfig,
    pub input: InputConfig,
    pub model: ModelConfig,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

fn load_all_configs() -> Result<Config> {
    // load audio config
    let audio_config_str = fs::read_to_string("src/config/yml/audio.yml")?;
    let audio_config: AudioConfig = serde_yaml::from_str(&audio_config_str)?;

    // load input config
    let input_config_str = fs::read_to_string("src/config/yml/input.yml")?;
    let input_config: InputConfig = serde_yaml::from_str(&input_config_str)?;

    // load model config
    let model_config_str = fs::read_to_string("src/config/yml/model.yml")?;
    let model_config: ModelConfig = serde_yaml::from_str(&model_config_str)?;

    Ok(Config { audio: audio_config, input: input_config, model: model_config })
}

impl Config {
    pub fn get() -> &'static Config {
        CONFIG.get_or_init(|| {
            load_all_configs().expect("Failed to load all configs")
        })
    }
}