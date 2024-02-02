mod power_curve;
mod structures;

use fitparser::{from_reader, profile::MesgNum, Value};
use power_curve::calculate_power_curve;
use std::fs::File;
use structures::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = "/Users/y/Downloads/2024-01-25-workout.fit";
    // let file = "/Users/y/Downloads/2024-01-09-125540-Indoor Cycling.fit";

    let mut fp = File::open(&file)?;
    let data = from_reader(&mut fp)?;
    println!("Length of fit file {}", data.len());
    // let mut workout_session = WorkoutSession::default();
    let data: Vec<FitEntry> = data.into_iter().map(FitEntry::new).collect();
    let power_data: Vec<u64> = data
        .iter()
        .filter_map(|x| match x {
            // MesgNum::Record => x.fields.iter().find(|(name, _)| *name == "power").and_then(
            //     |(_, value)| match value.value() {
            //         Value::UInt16(pwr) => Some(*pwr as u64),
            //         _ => None,
            //     },
            // ),
            // FitDataMap::Record(record) => return Some(record.power.value as u64),
            FitEntry::Record(record) => Some(record.power.value as u64),
            _ => None,
        })
        .collect();
    // .collect();
    println!("COLLECTED POWER DATA");
    let power_curve = calculate_power_curve(&power_data);
    println!("Parsed power curve");
    plot_power_curve(&power_curve.as_slice())?;

    Ok(())
    // Optionally, export data for plotting
}

use plotters::prelude::*;

fn plot_power_curve(data: &[(usize, f32)]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("power_curve.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_duration = data.last().map(|x| x.0).unwrap_or(0);
    let max_power = data.iter().map(|x| x.1).fold(0.0_f32, |a, b| a.max(b));

    let mut chart = ChartBuilder::on(&root)
        .caption("Power Curve", ("sans-serif", 40).into_font())
        .margin(15)
        .x_label_area_size(45)
        .y_label_area_size(45)
        .build_cartesian_2d(0..max_duration, 0.0_f32..max_power)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(data.iter().map(|&(x, y)| (x, y)), &RED))?;

    root.present()?;
    Ok(())
}

// fn turn_into_time(seconds: usize) -> String {
//     let hours = seconds / 3600;
//     let minutes = (seconds % 3600) / 60;
//     let seconds = seconds % 60;
//     match (hours, minutes, seconds) {
//         (0, 0, _) => format!("{:02}", seconds),
//         (0, _, _) => format!("{:02}:{:02}", minutes, seconds),
//         (_, _, _) => format!("{:02}:{:02}:{:02}", hours, minutes, seconds),
//     }
// }

// Usage: Call plot_power_curve with your power curve data
