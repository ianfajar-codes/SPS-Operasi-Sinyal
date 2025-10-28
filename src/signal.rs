// src/signal.rs
use std::f64::consts::PI;

/// Generate time vector from 0 to (samples-1)/fs
/// 
/// # Arguments
/// * `samples` - Number of samples to generate
/// * `fs` - Sampling frequency in Hz
/// 
/// # Returns
/// Vector of time values
pub fn generate_time_vector(samples: usize, fs: f64) -> Vec<f64> {
    (0..samples)
        .map(|n| n as f64 / fs)
        .collect()
}

/// Generate sinusoidal signal using formula: x(t) = A * sin(2πft + φ)
/// 
/// # Arguments
/// * `amplitude` - Signal amplitude (A)
/// * `frequency` - Signal frequency in Hz (f)
/// * `phase` - Phase offset in radians (φ)
/// * `time` - Time vector
/// 
/// # Returns
/// Vector of signal values
pub fn generate_signal(amplitude: f64, frequency: f64, phase: f64, time: &[f64]) -> Vec<f64> {
    time.iter()
        .map(|&t| amplitude * (2.0 * PI * frequency * t + phase).sin())
        .collect()
}

/// Generate cosine signal using formula: x(t) = A * cos(2πft + φ)
/// 
/// # Arguments
/// * `amplitude` - Signal amplitude (A)
/// * `frequency` - Signal frequency in Hz (f)
/// * `phase` - Phase offset in radians (φ)
/// * `time` - Time vector
/// 
/// # Returns
/// Vector of signal values
#[allow(dead_code)]
pub fn generate_cosine_signal(amplitude: f64, frequency: f64, phase: f64, time: &[f64]) -> Vec<f64> {
    time.iter()
        .map(|&t| amplitude * (2.0 * PI * frequency * t + phase).cos())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_time_vector() {
        let t = generate_time_vector(10, 1000.0);
        assert_eq!(t.len(), 10);
        assert_eq!(t[0], 0.0);
        assert!((t[9] - 0.009).abs() < 1e-10);
    }

    #[test]
    fn test_generate_signal() {
        let t = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let signal = generate_signal(1.0, 1.0, 0.0, &t);
        assert_eq!(signal.len(), 5);
        assert!((signal[0] - 0.0).abs() < 1e-10);
    }
}