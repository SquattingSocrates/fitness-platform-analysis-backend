use lazy_static::lazy_static;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;

const MAX_DURATION: usize = 86_400; // 24 hours in seconds

lazy_static! {
    static ref POWER_CURVE_BUCKETS: Vec<usize> = {
        let mut buckets = Vec::new();

        // Add 1 to 300
        buckets.extend(1..=300);

        // Add 310 to 3600, stepping by 10
        buckets.extend((310..=3600).step_by(10));

        // Add 3700 to MAX_DURATION, stepping by 30
        buckets.extend((3700..=MAX_DURATION).step_by(30));

        buckets
    };
}

fn get_power_curve_buckets(duration: usize) -> &'static [usize] {
    let duration = duration.min(MAX_DURATION);
    let end_index = POWER_CURVE_BUCKETS
        .iter()
        .position(|&x| x > duration)
        .unwrap_or_else(|| POWER_CURVE_BUCKETS.len());
    &POWER_CURVE_BUCKETS[..end_index]
}

pub fn calculate_power_curve(power_data: &[u64]) -> Vec<(usize, f32)> {
    if power_data.is_empty() {
        return vec![];
    }
    get_power_curve_buckets(power_data.len())
        .into_par_iter()
        .map(|duration| {
            let max_avg_power = (0..power_data.len() - *duration + 1)
                .map(|i| average(&power_data[i..i + *duration]))
                .fold(0.0, |a: f32, b: f32| a.max(b));
            (*duration, max_avg_power)
        })
        .collect()
}

fn average(slice: &[u64]) -> f32 {
    let sum: u64 = slice.iter().sum();
    sum as f32 / slice.len() as f32
}
