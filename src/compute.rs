pub fn perform_operation(signal1: &[f64], signal2: &[f64], operation: &str) -> Vec<f64> {
    match operation {
        "add" => add_signals(signal1, signal2),
        "subtract" => subtract_signals(signal1, signal2),
        "multiply" => multiply_signals(signal1, signal2),
        _ => {
            println!("Warning: Unknown operation '{}', defaulting to addition", operation);
            add_signals(signal1, signal2)
        }
    }
}

pub fn add_signals(signal1: &[f64], signal2: &[f64]) -> Vec<f64> {
    signal1.iter()
        .zip(signal2)
        .map(|(a, b)| a + b)
        .collect()
}

pub fn subtract_signals(signal1: &[f64], signal2: &[f64]) -> Vec<f64> {
    signal1.iter()
        .zip(signal2)
        .map(|(a, b)| a - b)
        .collect()
}


pub fn multiply_signals(signal1: &[f64], signal2: &[f64]) -> Vec<f64> {
    signal1.iter()
        .zip(signal2)
        .map(|(a, b)| a * b)
        .collect()
}

pub fn calculate_rms(signal: &[f64]) -> f64 {
    if signal.is_empty() {
        return 0.0;
    }
    
    let sum_squares: f64 = signal.iter()
        .map(|&x| x * x)
        .sum();
    
    (sum_squares / signal.len() as f64).sqrt()
}

pub fn calculate_max_amplitude(signal: &[f64]) -> f64 {
    signal.iter()
        .map(|&x| x.abs())
        .fold(f64::NEG_INFINITY, f64::max)
}

#[allow(dead_code)]
pub fn calculate_mean(signal: &[f64]) -> f64 {
    if signal.is_empty() {
        return 0.0;
    }
    
    let sum: f64 = signal.iter().sum();
    sum / signal.len() as f64
}

#[allow(dead_code)]
pub fn calculate_peak_to_peak(signal: &[f64]) -> f64 {
    if signal.is_empty() {
        return 0.0;
    }
    
    let max = signal.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min = signal.iter().cloned().fold(f64::INFINITY, f64::min);
    
    max - min
}
