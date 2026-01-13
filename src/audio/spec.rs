use std::error::Error;
use std::path::PathBuf;
use hound::{SampleFormat, WavReader};
use mel_spec::prelude::{MelSpectrogram, Spectrogram};

pub fn audio_to_mel(file: &PathBuf, n_mels: usize) -> Result<Vec<ndarray::Array2<f64>>, Box<dyn Error>> {
    let mut reader = WavReader::open(file)?;
    let spec = reader.spec();
    let fft_size = 400;
    let hop_size = 100;
    let sample_rate = spec.sample_rate as f64;

    let mut stft = Spectrogram::new(fft_size, hop_size);
    let mut mel = MelSpectrogram::new(fft_size, sample_rate, n_mels);

    let mut mel_frames: Vec<ndarray::Array2<f64>> = Vec::new();

    let samples: Box<dyn Iterator<Item=Result<f32, _>>> = match spec.sample_format {
        SampleFormat::Float => Box::new(reader.samples::<f32>()),
        SampleFormat::Int => Box::new(reader.samples::<i32>().map(|i| i.map(|i| i as f32)))
    };

    for x in samples {
        let sample = x?;
        let buf = [sample];
        if let Some(fft_frame) = stft.add(&buf) {
            let mel_frame = mel.add(&fft_frame);
            mel_frames.push(mel_frame);
        }
    }

    Ok(mel_frames)
}