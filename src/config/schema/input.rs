use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InputConfig {
    pub input_type: String,
    pub input_path: String,
}