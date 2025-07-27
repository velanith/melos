use std::sync::mpsc::{Receiver, Sender};
use crate::config::config::Config;
use crate::dsp;
use ndarray::{Array, Array1, Array2};
use rustfft::FftPlanner;
use num_complex::Complex;

pub fn run(rx: Receiver<Vec<f32>>, tx: Sender<Array2<f32>>) {
    // 1. installation and setup
    let config = Config::get();
    let audio_config = config.audio;
    let model_config = config.model;

    // 1.1. get parameters
    let window_size = audio_config.window_size;
    let hop_length = audio_config.hop_length;
    let sample_rate = audio_config.sample_rate;
    let fft_size = audio_config.fft_size;
    let n_mels = audio_config.n_mels;
    let fmin = audio_config.fmin;
    let fmax = audio_config.fmax;
    
    // 1.2. once only setup
    let window = Array::from_vec(dsp::utils::hann_window(window_size));
    let mel_basis = dsp::utils::create_mel_basis(sample_rate, fft_size, n_mels, fmin, fmax);

    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(fft_size);

    // 1.3. buffer for audio data
    let mut buffer = Vec::<f32>::new();

    // 1.4. buffer for mel spectrogram expected model
    let mut mel_chunks = Vec::<Array1<f32>>::new();

    // 2. processing
    while let Ok(received_chunk) = rx.recv() {
        // 2.1. append received chunk to buffer
        buffer.extend(received_chunk);

        // 2.2. procces buffer if it's enough for processing
        while buffer.len() >= window_size {
            // 2.2.1. framing and windowing
            let frame_slice = &buffer[..window_size];
            let windowed: Array1<f32> = Array::from_iter(frame_slice.iter().copied()) * &window;

            // 2.2.2. apply fft
            let mut fft_buffer: Vec<Complex<f32>> = windowed
                .iter()
                .map(|&x| Complex::new(x, 0.0))
                .collect();
            fft_buffer.resize(fft_size, Complex::new(0.0, 0.0));
            fft.process(&mut fft_buffer);

            // 2.2.3. Power spectrum
            let power_spec : Array1<f32> = fft_buffer[..fft_size / 2 + 1]  
                .iter()
                .map(|&x| x.norm_sqr())
                .collect();

            // 2.2.4. Mel Filter
            let mel_filter = mel_basis.dot(&power_spec);

            // 2.2.5. Log mel spectrogram
            let log_mel_spec = mel_filter.mapv(|v| (v + 1e-10).ln());

            // 2.2.6. batching
            mel_chunks.push(log_mel_spec); 
            if mel_chunks.len() >= model_config.input_time_steps {
                // Vec<Array1<f32>> -> Array2<f32>
                let mut chunk_matrix = Array2::zeros((model_config.input_time_steps, n_mels));
                for (i, chunk) in mel_chunks.iter().enumerate() {
                    chunk_matrix.row_mut(i).assign(chunk);
                }

                // 2.2.7. send to model
                if let Err(err) = tx.send(chunk_matrix) {
                    eprintln!("Error sending chunk to model: {}", err);
                    break;
                }
                mel_chunks.clear();
            }

            // 2.2.7. update buffer
            buffer.drain(..hop_length);
        }
        
    }

    println!("Processor finished");
}