use std::f32::consts::PI;
use ndarray::{Array, Array2};

pub fn hann_window(size: usize) -> Vec<f32> {
    (0..size)
    .map(|i| 0.5 * (1.0 - f32::cos(2.0 * PI * i as f32 / (size as f32 - 1.0))))
    .collect()
}

// create mel filter bank for using in calculating mel spectrogram
pub fn create_mel_basis(
    sample_rate: f32, 
    n_fft: usize, 
    n_mels: usize, 
    fmin: f32, 
    fmax: f32) -> Array2<f32> {
    
        fn hz_to_mel(hz: f32) -> f32 {
            2595.0 * (1.0 + hz / 700.0).log10()
        }

        fn mel_to_hz(mel: f32) -> f32 {
            700.0 * (10.0f32.powf(mel / 2595.0) - 1.0)
        }

        let min_mel = hz_to_mel(fmin);
        let max_mel = hz_to_mel(fmax);
        let mel_scale = Array::linspace(min_mel, max_mel, n_mels + 2);
        let hz_scale = mel_scale.mapv(mel_to_hz);
        let fft_bins = (hz_scale * (n_fft as f32 / sample_rate as f32)).mapv(|hz| hz.round() as usize);

        let mut mel_basis = Array2::zeros((n_mels, n_fft / 2 + 1));

        for i in 0..n_mels {
            let start_bin = fft_bins[i];
            let center_bin = fft_bins[i + 1];
            let end_bin = fft_bins[i + 2];

            for j in start_bin..center_bin {
                mel_basis[[i, j]] = (j - start_bin) as f32 / (center_bin - start_bin) as f32;
            }
            for j in center_bin..end_bin {
                mel_basis[[i, j]] = (end_bin - j) as f32 / (end_bin - center_bin) as f32;
            }
        }
        mel_basis
}