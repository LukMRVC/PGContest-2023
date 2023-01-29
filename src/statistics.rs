use rayon::prelude::*;

pub fn mean(data: &[usize]) -> f32 {
    let sum = data.iter().sum::<usize>() as f32;
    let count = data.len() as f32;

    sum / count
}

pub fn std_dev(data: &[usize], data_mean: f32) -> f32 {
    let variance: f32 = data
        .iter()
        .map(|value| {
            let diff = data_mean - (*value as f32);
            diff * diff
        })
        .sum::<f32>()
        / data.len() as f32;

    variance.sqrt()
}
