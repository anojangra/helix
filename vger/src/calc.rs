//! Calculation functions

/// Calculates average from vector of f32s
///
/// ## Usage
/// ```
///     let test_vec: Vec<f32> = vec![6.0, 2.0, 3.0, 1.0];
///     let result = average(test_vec);
///     assert_eq!(result, 3.0 as f32);
/// ```
pub fn average(values: Vec<f32>) -> f32 {
    let sum: f32 = values.iter().sum();
    return sum / values.len() as f32;
}

/// Calculates standard deviation from vector fo f32s
///
/// ## Usage
/// ```
///     let test_vec: Vec<f32> = vec![6.0, 2.0, 3.0, 1.0];
///     let result = std_dev(test_vec);
///     assert_eq!(result, 1.8708287 as f32);
/// ```
pub fn std_dev(values: Vec<f32>) -> f32 {
    let mean = average(values.clone());
    let diffs: Vec<f32> = values.iter().map(|x| x - mean).collect();
    let abs_diffs: Vec<f32> = diffs.iter().map(|x| x.abs()).collect();
    let square_diffs: Vec<f32> = abs_diffs.iter().map(|x| x.powf(2 as f32)).collect();
    let avg_diff = average(square_diffs);
    avg_diff.sqrt()
}

/// Calculates kelly ratio
pub fn kelly(mean: f32, variance: f32) -> f32 {
    if variance > 0.0 {
        return mean / variance;
    }
    return 0.0;
}