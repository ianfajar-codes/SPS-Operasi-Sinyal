use std::f64::consts::PI;

pub fn generate_time_vector(samples: usize, fs: f64) -> Vec<f64> {
    (0..samples)
        .map(|n| n as f64 / fs)
        .collect()
}

pub fn generate_signal(amplitude: f64, frequency: f64, phase: f64, time: &[f64]) -> Vec<f64> {
    time.iter()
        .map(|&t| amplitude * (2.0 * PI * frequency * t + phase).sin())
        .collect()
}

#[allow(dead_code)]
pub fn generate_cosine_signal(amplitude: f64, frequency: f64, phase: f64, time: &[f64]) -> Vec<f64> {
    time.iter()
        .map(|&t| amplitude * (2.0 * PI * frequency * t + phase).cos())
        .collect()
}
