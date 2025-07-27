use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AudioConfig {
    pub sample_rate: u32,
    pub fft_size: u32,
    pub hop_length: u32,
    pub n_mels: u32,
    pub window_size: u32,
}
