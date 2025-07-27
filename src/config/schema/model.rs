use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ModelConfig {
    pub input_time_steps: usize,
}