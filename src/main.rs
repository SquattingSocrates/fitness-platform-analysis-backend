use fitparser::{from_reader, Value};
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::{cmp, fs::File};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let max_duration = 300;
    // let file = "/Users/y/Downloads/2024-01-25-workout.fit";
    let file = "/Users/y/Downloads/2024-01-09-125540-Indoor Cycling.fit";

    let mut fp = File::open(&file)?;
    let mut data = from_reader(&mut fp)?;
    println!("Length of fit file {}", data.len());

    let power_data: Vec<i32> = data
        .iter()
        .map(|entry| {
            let found = entry
                .fields()
                .iter()
                .find(|f| f.name() == "power")
                .and_then(|e| Some(e.value()));
            match found {
                Some(Value::UInt16(power)) => *power as i32,
                Some(Value::UInt8(power)) => *power as i32,
                Some(Value::UInt32(power)) => *power as i32,
                Some(Value::UInt64(power)) => *power as i32,
                Some(Value::SInt8(power)) => *power as i32,
                Some(Value::SInt16(power)) => *power as i32,
                Some(Value::SInt32(power)) => *power as i32,
                Some(Value::SInt64(power)) => *power as i32,
                Some(Value::Float32(power)) => *power as i32,
                Some(Value::Float64(power)) => *power as i32,
                Some(Value::UInt8z(power)) => *power as i32,
                Some(Value::UInt16z(power)) => *power as i32,
                Some(Value::UInt32z(power)) => *power as i32,
                Some(Value::UInt64z(power)) => *power as i32,
                Some(value) => {
                    eprintln!("Unexpected value: {:?}", value);
                    0
                }
                None => 0,
            }
        })
        .collect();
    // let power_data: Vec<i32> = vec![];
    let power_curve = calculate_power_curve(&power_data, max_duration);

    // Output the power curve data
    // for (duration, power) in power_curve.iter() {
    //     println!("{} seconds: {} watts", duration, power);
    // }
    plot_power_curve(&power_curve.as_slice())?;

    Ok(())
    // Optionally, export data for plotting
}

fn calculate_power_curve(power_data: &[i32], max_duration: usize) -> Vec<(usize, f32)> {
    if power_data.is_empty() {
        return vec![];
    }
    (1..=max_duration)
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

use plotters::prelude::*;

fn plot_power_curve(data: &[(usize, f32)]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("power_curve.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_duration = data.last().map(|x| x.0).unwrap_or(0);
    let max_power = data.iter().map(|x| x.1).fold(0.0_f32, |a, b| a.max(b));

    let mut chart = ChartBuilder::on(&root)
        .caption("Power Curve", ("sans-serif", 40).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..max_duration, 0.0_f32..max_power)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(data.iter().map(|&(x, y)| (x, y)), &RED))?;

    root.present()?;
    Ok(())
}

// Usage: Call plot_power_curve with your power curve data
