// src/compute.rs

/// Perform signal operations: addition, subtraction, or multiplication
/// 
/// # Arguments
/// * `signal1` - First input signal
/// * `signal2` - Second input signal
/// * `operation` - Operation type: "add", "subtract", or "multiply"
/// 
/// # Returns
/// Result signal after performing the operation
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

/// Add two signals element-wise: y(t) = x1(t) + x2(t)
/// 
/// # Arguments
/// * `signal1` - First signal
/// * `signal2` - Second signal
/// 
/// # Returns
/// Sum of the two signals
pub fn add_signals(signal1: &[f64], signal2: &[f64]) -> Vec<f64> {
    signal1.iter()
        .zip(signal2)
        .map(|(a, b)| a + b)
        .collect()
}

/// Subtract two signals element-wise: y(t) = x1(t) - x2(t)
/// 
/// # Arguments
/// * `signal1` - First signal
/// * `signal2` - Second signal
/// 
/// # Returns
/// Difference of the two signals
pub fn subtract_signals(signal1: &[f64], signal2: &[f64]) -> Vec<f64> {
    signal1.iter()
        .zip(signal2)
        .map(|(a, b)| a - b)
        .collect()
}

/// Multiply two signals element-wise: y(t) = x1(t) × x2(t)
/// 
/// # Arguments
/// * `signal1` - First signal
/// * `signal2` - Second signal
/// 
/// # Returns
/// Product of the two signals
pub fn multiply_signals(signal1: &[f64], signal2: &[f64]) -> Vec<f64> {
    signal1.iter()
        .zip(signal2)
        .map(|(a, b)| a * b)
        .collect()
}

/// Calculate Root Mean Square (RMS) value of a signal
/// Formula: RMS = sqrt(mean(x²))
/// 
/// # Arguments
/// * `signal` - Input signal
/// 
/// # Returns
/// RMS value of the signal
pub fn calculate_rms(signal: &[f64]) -> f64 {
    if signal.is_empty() {
        return 0.0;
    }
    
    let sum_squares: f64 = signal.iter()
        .map(|&x| x * x)
        .sum();
    
    (sum_squares / signal.len() as f64).sqrt()
}

/// Calculate maximum absolute amplitude of a signal
/// 
/// # Arguments
/// * `signal` - Input signal
/// 
/// # Returns
/// Maximum absolute value in the signal
pub fn calculate_max_amplitude(signal: &[f64]) -> f64 {
    signal.iter()
        .map(|&x| x.abs())
        .fold(f64::NEG_INFINITY, f64::max)
}

/// Calculate mean (average) value of a signal
/// 
/// # Arguments
/// * `signal` - Input signal
/// 
/// # Returns
/// Mean value of the signal
#[allow(dead_code)]
pub fn calculate_mean(signal: &[f64]) -> f64 {
    if signal.is_empty() {
        return 0.0;
    }
    
    let sum: f64 = signal.iter().sum();
    sum / signal.len() as f64
}

/// Calculate peak-to-peak amplitude of a signal
/// 
/// # Arguments
/// * `signal` - Input signal
/// 
/// # Returns
/// Peak-to-peak amplitude (max - min)
#[allow(dead_code)]
pub fn calculate_peak_to_peak(signal: &[f64]) -> f64 {
    if signal.is_empty() {
        return 0.0;
    }
    
    let max = signal.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min = signal.iter().cloned().fold(f64::INFINITY, f64::min);
    
    max - min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_signals() {
        let s1 = vec![1.0, 2.0, 3.0];
        let s2 = vec![4.0, 5.0, 6.0];
        let result = add_signals(&s1, &s2);
        assert_eq!(result, vec![5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_subtract_signals() {
        let s1 = vec![5.0, 7.0, 9.0];
        let s2 = vec![1.0, 2.0, 3.0];
        let result = subtract_signals(&s1, &s2);
        assert_eq!(result, vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_multiply_signals() {
        let s1 = vec![2.0, 3.0, 4.0];
        let s2 = vec![5.0, 6.0, 7.0];
        let result = multiply_signals(&s1, &s2);
        assert_eq!(result, vec![10.0, 18.0, 28.0]);
    }

    #[test]
    fn test_calculate_rms() {
        let signal = vec![1.0, 2.0, 3.0, 4.0];
        let rms = calculate_rms(&signal);
        assert!((rms - 2.7386).abs() < 0.001);
    }

    #[test]
    fn test_calculate_max_amplitude() {
        let signal = vec![-3.0, 1.0, 5.0, -2.0];
        let max = calculate_max_amplitude(&signal);
        assert_eq!(max, 5.0);
    }
}