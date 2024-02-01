use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;

pub fn calculate_power_curve(power_data: &[i32]) -> Vec<(usize, f32)> {
    if power_data.is_empty() {
        return vec![];
    }
    (1..=power_data.len())
        .into_par_iter()
        .map(|duration| {
            let max_avg_power = (0..power_data.len() - duration + 1)
                .map(|i| average(&power_data[i..i + duration]))
                .fold(0.0, |a: f32, b: f32| a.max(b));
            (duration, max_avg_power)
        })
        .collect()
}

fn average(slice: &[i32]) -> f32 {
    let sum: i32 = slice.iter().sum();
    sum as f32 / slice.len() as f32
}
