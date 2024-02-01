use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use fitparser::{profile::MesgNum, FitDataField, FitDataRecord, Value};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PowerZoneDescription {
    pub low_bound: u32,
    pub high_bound: u32,
    pub time_spent_in_zone: u32,
    pub percentage_of_time_in_zone: f32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TimeSeriesData {
    pub speed: Vec<f32>,
    pub power: Vec<f32>,
    pub pace: Vec<f32>,
    pub vertical_oscillation: Vec<f32>,
    pub ground_contact_time: Vec<f32>,
    pub stride_length: Vec<f32>,
    pub cadence: Vec<f32>,
    pub heart_rate: Vec<f32>,
    pub elevation: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkoutType {
    Cycling,
    Run,
    WeightTraining,
}

#[derive(Serialize, Debug)]
pub struct ValueWithUnit<T> {
    pub value: T,
    pub units: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Split {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub name: Option<String>,
    // ... other fields specific to the type of split ...
}

#[derive(Serialize, Debug)]
pub enum FitEntry {
    FileId {
        manufacturer: String,
        product_name: String,
        serial_number: u32,          // UInt32z
        time_created: DateTime<Utc>, // Timestamp
        file_type: String,
    },
    FileCreator {
        software_version: u16,
    },
    DeviceInfo {
        descriptor: String,
        device_index: String,
        manufacturer: String,
        product_name: String,
        serial_number: u32, // UInt32z
        source_type: String,
        timestamp: DateTime<Utc>, // Timestamp
    },
    DeveloperDataId {
        application_id: Vec<u8>,
        application_version: u32,
        developer_data_index: u8,
    },
    FieldDescription {
        array: u8,
        developer_data_index: u8,
        field_definition_number: u8,
        field_name: String,
        fit_base_type_id: String,
    },
    Workout {
        capabilities: String,
        num_valid_steps: u16,
        sport: String,
        wkt_name: String,
    },
    WorkoutStep {
        duration_time: ValueWithUnit<f64>, // Float64
        duration_type: String,
        intensity: String,
        message_index: i64, // SInt64
        target_type: String,
        target_value: u32,
    },
    Event {
        event: String,
        event_group: u8,
        event_type: String,
        timer_trigger: String,
        timestamp: DateTime<Utc>, // Timestamp
    },
    Sport {
        name: String,
        sport: String,
        sub_sport: String,
    },
    ZonesTarget {
        functional_threshold_power: ValueWithUnit<f64>,
        pwr_calc_type: String,
    },
    Record {
        accumulated_power: ValueWithUnit<u32>, // UInt32
        power: ValueWithUnit<u16>,             // UInt16
        timestamp: DateTime<Utc>,              // Timestamp
    },
    Lap {
        avg_cadence: ValueWithUnit<f64>,
        avg_fractional_cadence: ValueWithUnit<f64>, // Float64
        avg_heart_rate: ValueWithUnit<u8>,
        avg_power: ValueWithUnit<u16>,             // UInt16
        enhanced_avg_speed: ValueWithUnit<f64>,    // Float64
        enhanced_max_altitude: ValueWithUnit<f64>, // Float64
        enhanced_max_speed: ValueWithUnit<f64>,    // Float64
        enhanced_min_altitude: ValueWithUnit<f64>, // Float64
        event: String,
        event_type: String,
        intensity: String,
        max_cadence: ValueWithUnit<f64>,
        max_fractional_cadence: ValueWithUnit<f64>, // Float64
        max_heart_rate: ValueWithUnit<u8>,
        max_power: ValueWithUnit<u16>, // UInt16
        message_index: i64,            // SInt64
        min_heart_rate: ValueWithUnit<u8>,
        sport: String,
        start_time: DateTime<Utc>, // Timestamp
        sub_sport: String,
        timestamp: DateTime<Utc>,               // Timestamp
        total_calories: ValueWithUnit<u16>,     // UInt16
        total_distance: ValueWithUnit<f64>,     // Float64
        total_elapsed_time: ValueWithUnit<f64>, // Float64
        total_timer_time: ValueWithUnit<f64>,   // Float64
        wkt_step_index: i64,                    // SInt64
    },
    TimeInZone {
        functional_threshold_power: ValueWithUnit<f64>,
        hr_calc_type: String,
        hr_zone_high_boundary: ValueWithUnit<f64>,
        max_heart_rate: ValueWithUnit<f64>,
        power_zone_high_boundary: ValueWithUnit<f64>,
        pwr_calc_type: String,
        reference_index: i64, // SInt64
        reference_mesg: String,
        resting_heart_rate: ValueWithUnit<f64>,
        time_in_hr_zone: Vec<f64>,    // Array of Float64
        time_in_power_zone: Vec<f64>, // Array of Float64
        timestamp: DateTime<Utc>,     // Timestamp
    },
    Session {
        avg_cadence: ValueWithUnit<f64>,
        avg_fractional_cadence: ValueWithUnit<f64>, // Float64
        avg_heart_rate: ValueWithUnit<f64>,
        avg_power: ValueWithUnit<f64>,             // UInt16
        avg_temperature: ValueWithUnit<f64>,       // SInt8
        enhanced_avg_altitude: ValueWithUnit<f64>, // Float64
        enhanced_avg_speed: ValueWithUnit<f64>,    // Float64
        enhanced_max_altitude: ValueWithUnit<f64>, // Float64
        enhanced_max_speed: ValueWithUnit<f64>,    // Float64
        enhanced_min_altitude: ValueWithUnit<f64>, // Float64
        event_type: String,
        first_lap_index: ValueWithUnit<f64>, // UInt16
        max_cadence: ValueWithUnit<f64>,
        max_fractional_cadence: ValueWithUnit<f64>, // Float64
        max_heart_rate: ValueWithUnit<f64>,
        max_power: ValueWithUnit<f64>, // UInt16
        message_index: i64,            // SInt64
        min_heart_rate: ValueWithUnit<f64>,
        nec_lat: ValueWithUnit<f64>,  // SInt32
        nec_long: ValueWithUnit<f64>, // SInt32
        num_laps: ValueWithUnit<f64>, // UInt16
        sport: String,
        start_time: DateTime<Utc>, // Timestamp
        sub_sport: String,
        swc_lat: ValueWithUnit<f64>,            // SInt32
        swc_long: ValueWithUnit<f64>,           // SInt32
        threshold_power: ValueWithUnit<f64>,    // UInt16
        timestamp: DateTime<Utc>,               // Timestamp
        total_ascent: ValueWithUnit<f64>,       // UInt16
        total_calories: ValueWithUnit<f64>,     // UInt16
        total_distance: ValueWithUnit<f64>,     // Float64
        total_elapsed_time: ValueWithUnit<f64>, // Float64
        total_timer_time: ValueWithUnit<f64>,   // Float64
        trigger: String,
    },
    Activity {
        event: String,
        event_type: String,
        local_timestamp: DateTime<Utc>,       // Timestamp
        num_sessions: ValueWithUnit<f64>,     // UInt16
        timestamp: DateTime<Utc>,             // Timestamp
        total_timer_time: ValueWithUnit<f64>, // Float64
        type_: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkoutSession {
    pub name: String,
    // pub workout_type: WorkoutType,
    pub distance: Option<f32>,
    pub moving_time: Option<u32>,
    pub total_time: u32,
    pub start_date_time: chrono::DateTime<chrono::Utc>,
    pub average_speed: Option<f32>,
    pub max_speed: Option<f32>,
    pub average_heart_rate: Option<u8>,
    pub max_heart_rate: Option<u8>,
    pub splits: Option<Vec<Split>>,
    pub time_series_data: TimeSeriesData,
    pub power_zone_distribution: Vec<PowerZoneDescription>,
    pub power_curve: Vec<u32>,
}

fn value_to_string(field: &FitDataField) -> Option<String> {
    match field.into_value() {
        Value::String(s) => Some(s.to_owned()),
        _ => None,
    }
}

fn value_to_i64(field: &FitDataField) -> Option<i64> {
    field.into_value().try_into().ok()
}

fn value_to_f64(field: &FitDataField) -> Option<f64> {
    field.into_value().try_into().ok()
}

fn value_to_units(field: &FitDataField) -> Option<ValueWithUnit<f64>> {
    Some(ValueWithUnit {
        value: field.value().to_owned().try_into().unwrap(),
        units: field.units().to_owned(),
    })
}

fn to_timestamp(field: &FitDataField) -> Option<DateTime<Utc>> {
    match field.into_value() {
        Value::Timestamp(t) => Some(t.into()),
        _ => None,
    }
}

macro_rules! extract_field {
    ($record:expr, $field_name:expr, $default_type:ty, $transform:expr) => {
        FitEntry::get_field($record, $field_name)
            .and_then(|field| $transform(field) as Option<$default_type>)
            .unwrap_or_else(|| <$default_type>::default())
    };
}

macro_rules! extract_value_with_unit {
    ($record:expr, $field_name:expr, $try_into_type:ty, $output_type:ty, $default_unit:expr) => {{
        FitEntry::get_field($record, $field_name)
            .and_then(|f| {
                let value: $try_into_type = f.value().to_owned().try_into().ok()?;
                let units = f.units().to_owned();
                Some(ValueWithUnit {
                    value: value as $output_type,
                    units,
                })
            })
            .unwrap_or_else(|| ValueWithUnit {
                value: <$output_type>::default(),
                units: String::from($default_unit),
            })
    }};
}

impl FitEntry {
    pub fn get_field<'a>(record: &'a FitDataRecord, field_name: &str) -> Option<&'a FitDataField> {
        record.fields().into_iter().find(|f| f.name() == field_name)
    }

    pub fn new(record: fitparser::FitDataRecord) -> Self {
        match record.kind() {
            MesgNum::FileId => FitEntry::FileId {
                manufacturer: FitEntry::get_field(&record, "manufacturer")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
                product_name: FitEntry::get_field(&record, "product_name")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
                serial_number: FitEntry::get_field(&record, "product_name")
                    .and_then(value_to_i64)
                    .unwrap_or_else(|| 0) as u32,
                time_created: FitEntry::get_field(&record, "time_created")
                    .and_then(to_timestamp)
                    .unwrap_or_else(|| Utc::now()),
                file_type: FitEntry::get_field(&record, "file_type")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
            },
            MesgNum::FileCreator => FitEntry::FileCreator {
                software_version: FitEntry::get_field(&record, "software_version")
                    .and_then(value_to_i64)
                    .unwrap_or_else(|| 0) as u16,
            },
            MesgNum::DeviceInfo => FitEntry::DeviceInfo {
                descriptor: FitEntry::get_field(&record, "descriptor")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
                device_index: FitEntry::get_field(&record, "device_index")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
                manufacturer: FitEntry::get_field(&record, "manufacturer")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
                product_name: FitEntry::get_field(&record, "product_name")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
                serial_number: FitEntry::get_field(&record, "serial_number")
                    .and_then(value_to_i64)
                    .unwrap_or_else(|| 0) as u32,
                source_type: FitEntry::get_field(&record, "source_type")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
                timestamp: FitEntry::get_field(&record, "timestamp")
                    .and_then(to_timestamp)
                    .unwrap_or_else(|| Utc::now()),
            },
            MesgNum::DeveloperDataId => FitEntry::DeveloperDataId {
                application_id: FitEntry::get_field(&record, "application_id")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from(""))
                    .into_bytes(),
                application_version: FitEntry::get_field(&record, "application_version")
                    .and_then(value_to_i64)
                    .unwrap_or_else(|| 0) as u32,
                developer_data_index: FitEntry::get_field(&record, "developer_data_index")
                    .and_then(value_to_i64)
                    .unwrap_or_else(|| 0) as u8,
            },
            MesgNum::FieldDescription => FitEntry::FieldDescription {
                array: FitEntry::get_field(&record, "array")
                    .and_then(value_to_i64)
                    .unwrap_or_else(|| 0) as u8,
                developer_data_index: FitEntry::get_field(&record, "developer_data_index")
                    .and_then(value_to_i64)
                    .unwrap_or_else(|| 0) as u8,
                field_definition_number: FitEntry::get_field(&record, "field_definition_number")
                    .and_then(value_to_i64)
                    .unwrap_or_else(|| 0) as u8,
                field_name: FitEntry::get_field(&record, "field_name")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
                fit_base_type_id: FitEntry::get_field(&record, "fit_base_type_id")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
            },
            MesgNum::Workout => FitEntry::Workout {
                capabilities: extract_field!(&record, "capabilities", String, value_to_string),
                num_valid_steps: extract_field!(&record, "num_valid_steps", u16, value_to_i64),
                sport: extract_field!(&record, "sport", String, value_to_string),
                wkt_name: extract_field!(&record, "wkt_name", String, value_to_string),
            },
            MesgNum::WorkoutStep => FitEntry::WorkoutStep {
                duration_time: extract_value_with_unit!(&record, "duration_time", f64, f64, ""),
                duration_type: extract_field!(&record, "duration_type", String, value_to_string),
                intensity: extract_field!(&record, "intensity", String, value_to_string),
                message_index: extract_field!(&record, "message_index", i64, value_to_i64),
                target_type: extract_field!(&record, "target_type", String, value_to_string),
                target_value: extract_field!(&record, "target_value", u32, value_to_i64),
            },
            MesgNum::Event => FitEntry::Event {
                event: FitEntry::get_field(&record, "event")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
                event_group: FitEntry::get_field(&record, "event_group")
                    .and_then(value_to_i64)
                    .unwrap_or_else(|| 0) as u8,
                event_type: FitEntry::get_field(&record, "event_type")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
                timer_trigger: FitEntry::get_field(&record, "timer_trigger")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
                timestamp: FitEntry::get_field(&record, "timestamp")
                    .and_then(to_timestamp)
                    .unwrap_or_else(|| Utc::now()),
            },
            MesgNum::Sport => FitEntry::Sport {
                name: FitEntry::get_field(&record, "name")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
                sport: FitEntry::get_field(&record, "sport")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
                sub_sport: FitEntry::get_field(&record, "sub_sport")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
            },
            MesgNum::ZonesTarget => FitEntry::ZonesTarget {
                functional_threshold_power: FitEntry::get_field(
                    &record,
                    "functional_threshold_power",
                )
                .and_then(|f| {
                    Some(ValueWithUnit {
                        value: f.value().to_owned().try_into().unwrap(),
                        units: f.units().to_owned(),
                    })
                })
                .unwrap_or_else(|| ValueWithUnit {
                    value: 0.0,
                    units: String::from("W"),
                }),
                pwr_calc_type: FitEntry::get_field(&record, "pwr_calc_type")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
            },
            MesgNum::Record => FitEntry::Record {
                accumulated_power: FitEntry::get_field(&record, "accumulated_power")
                    .and_then(|field| {
                        let value: i64 = field.value().to_owned().try_into().unwrap();
                        Some(ValueWithUnit {
                            value: value as u32,
                            units: field.units().to_owned(),
                        })
                    })
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0,
                        units: String::from(""),
                    }),
                power: FitEntry::get_field(&record, "power")
                    .and_then(|f| {
                        Some(ValueWithUnit {
                            value: f.value().to_owned().try_into().unwrap() as u16,
                            units: f.units().to_owned(),
                        })
                    })
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0,
                        units: String::from(""),
                    }),
                timestamp: FitEntry::get_field(&record, "timestamp")
                    .and_then(to_timestamp)
                    .unwrap_or_else(|| Utc::now()),
            },
            MesgNum::Lap => FitEntry::Lap {
                avg_cadence: FitEntry::get_field(&record, "avg_cadence")
                    .and_then(|f| {
                        Some(ValueWithUnit {
                            value: f.value().to_owned().try_into().unwrap(),
                            units: f.units().to_owned(),
                        })
                    })
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0.0,
                        units: String::from(""),
                    }),
                avg_fractional_cadence: FitEntry::get_field(&record, "avg_fractional_cadence")
                    .and_then(|f| {
                        Some(ValueWithUnit {
                            value: f.value().to_owned().try_into().unwrap(),
                            units: f.units().to_owned(),
                        })
                    })
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0.0,
                        units: String::from(""),
                    }),
                avg_heart_rate: FitEntry::get_field(&record, "avg_heart_rate")
                    .and_then(|f| {
                        Some(ValueWithUnit {
                            value: (f.value().to_owned().try_into().unwrap() as i64) as u8,
                            units: f.units().to_owned(),
                        })
                    })
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0,
                        units: String::from(""),
                    }),
                avg_power: FitEntry::get_field(&record, "avg_power")
                    .and_then(|f| {
                        Some(ValueWithUnit {
                            value: f.value().to_owned().try_into().unwrap() as u16,
                            units: f.units().to_owned(),
                        })
                    })
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0,
                        units: String::from(""),
                    }),
                enhanced_avg_speed: FitEntry::get_field(&record, "enhanced_avg_speed")
                    .and_then(|f| {
                        Some(ValueWithUnit {
                            value: f.value().to_owned().try_into().unwrap(),
                            units: String::from("m/s"),
                        })
                    })
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0.0,
                        units: String::from("m/s"),
                    }),
                enhanced_max_altitude: FitEntry::get_field(&record, "enhanced_max_altitude")
                    .and_then(|f| {
                        Some(ValueWithUnit {
                            value: f.value().to_owned().try_into().unwrap(),
                            units: f.units().to_owned(),
                        })
                    })
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0.0,
                        units: String::from("m"),
                    }),
                enhanced_max_speed: FitEntry::get_field(&record, "enhanced_max_speed")
                    .and_then(|f| {
                        Some(ValueWithUnit {
                            value: f.value().to_owned().try_into().unwrap(),
                            units: String::from("m/s"),
                        })
                    })
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0.0,
                        units: String::from("m/s"),
                    }),
                enhanced_min_altitude: FitEntry::get_field(&record, "enhanced_min_altitude")
                    .and_then(|f| {
                        Some(ValueWithUnit {
                            value: f.value().to_owned().try_into().unwrap(),
                            units: String::from("m"),
                        })
                    })
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0.0,
                        units: String::from("m"),
                    }),
                event: FitEntry::get_field(&record, "event")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
                event_type: FitEntry::get_field(&record, "event_type")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
                intensity: FitEntry::get_field(&record, "intensity")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
                max_cadence: FitEntry::get_field(&record, "max_cadence")
                    .and_then(|f| {
                        Some(ValueWithUnit {
                            value: f.value().to_owned().try_into().unwrap(),
                            units: String::from("rpm"),
                        })
                    })
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0.0,
                        units: String::from("rpm"),
                    }),
                max_fractional_cadence: FitEntry::get_field(&record, "max_fractional_cadence")
                    .and_then(value_to_units)
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0.0,
                        units: String::from("rpm"),
                    }),
                max_heart_rate: FitEntry::get_field(&record, "max_heart_rate")
                    .and_then(value_to_units)
                    .and_then(|f| {
                        Some(ValueWithUnit {
                            value: f.value as u8,
                            units: f.units,
                        })
                    })
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0,
                        units: String::from("bpm"),
                    }),
                max_power: FitEntry::get_field(&record, "max_power")
                    .and_then(value_to_units)
                    .and_then(|f| {
                        Some(ValueWithUnit {
                            value: f.value as u16,
                            units: f.units,
                        })
                    })
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0,
                        units: String::from("W"),
                    }),
                message_index: FitEntry::get_field(&record, "message_index")
                    .and_then(value_to_i64)
                    .unwrap_or_else(|| 0),
                min_heart_rate: FitEntry::get_field(&record, "min_heart_rate")
                    .and_then(value_to_units)
                    .and_then(|f| {
                        Some(ValueWithUnit {
                            value: f.value as u8,
                            units: f.units,
                        })
                    })
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0,
                        units: String::from("bpm"),
                    }),
                sport: FitEntry::get_field(&record, "sport")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
                start_time: FitEntry::get_field(&record, "start_time")
                    .and_then(to_timestamp)
                    .unwrap_or_else(|| Utc::now()),
                sub_sport: FitEntry::get_field(&record, "sub_sport")
                    .and_then(value_to_string)
                    .unwrap_or_else(|| String::from("")),
                timestamp: FitEntry::get_field(&record, "timestamp")
                    .and_then(to_timestamp)
                    .unwrap_or_else(|| Utc::now()),
                total_calories: FitEntry::get_field(&record, "total_calories")
                    .and_then(|f| {
                        Some(ValueWithUnit {
                            value: f.value().to_owned().try_into().unwrap() as u16,
                            units: f.units().to_owned(),
                        })
                    })
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0,
                        units: String::from("kcal"),
                    }),
                total_distance: FitEntry::get_field(&record, "total_distance")
                    .and_then(|f| {
                        Some(ValueWithUnit {
                            value: f.value().to_owned().try_into().unwrap(),
                            units: String::from("m"),
                        })
                    })
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0.0,
                        units: String::from("m"),
                    }),
                total_elapsed_time: FitEntry::get_field(&record, "total_elapsed_time")
                    .and_then(value_to_units)
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0.0,
                        units: String::from("s"),
                    }),
                total_timer_time: FitEntry::get_field(&record, "total_timer_time")
                    .and_then(value_to_units)
                    .unwrap_or_else(|| ValueWithUnit {
                        value: 0.0,
                        units: String::from("s"),
                    }),
                wkt_step_index: FitEntry::get_field(&record, "wkt_step_index")
                    .and_then(value_to_i64)
                    .unwrap_or_else(|| 0),
            },
            // TODO: this is useful
            MesgNum::Set => todo!(),
            MesgNum::StressLevel => todo!(),
            MesgNum::MaxMetData => todo!(),
            MesgNum::DiveSettings => todo!(),
            MesgNum::DiveGas => todo!(),
            MesgNum::DiveAlarm => todo!(),
            MesgNum::ExerciseTitle => todo!(),
            MesgNum::DiveSummary => todo!(),
            MesgNum::Spo2Data => todo!(),
            MesgNum::SleepLevel => todo!(),
            MesgNum::Jump => todo!(),
            MesgNum::BeatIntervals => todo!(),
            MesgNum::RespirationRate => todo!(),
            MesgNum::Split => todo!(),
            // MesgNum::Split => FitEntry::Split {
            //     start_time: FitEntry::get_field(&record, "start_time")
            //         .and_then(to_timestamp)
            //         .unwrap_or_else(|| Utc::now()),
            //     end_time: FitEntry::get_field(&record, "end_time")
            //         .and_then(to_timestamp)
            //         .unwrap_or_else(|| Utc::now()),
            //     name: FitEntry::get_field(&record, "name").and_then(value_to_string),
            // },
            MesgNum::ClimbPro => todo!(),
            MesgNum::TankUpdate => todo!(),
            MesgNum::TankSummary => todo!(),
            MesgNum::SleepAssessment => todo!(),
            MesgNum::HrvStatusSummary => todo!(),
            MesgNum::HrvValue => todo!(),
            MesgNum::DeviceAuxBatteryInfo => todo!(),
            MesgNum::DiveApneaAlarm => todo!(),
            MesgNum::MfgRangeMin => todo!(),
            MesgNum::MfgRangeMax => todo!(),
            MesgNum::Value(_) => todo!(),
            MesgNum::Capabilities => todo!(),
            MesgNum::DeviceSettings => todo!(),
            MesgNum::UserProfile => todo!(),
            MesgNum::HrmProfile => todo!(),
            MesgNum::SdmProfile => todo!(),
            MesgNum::BikeProfile => todo!(),
            MesgNum::HrZone => todo!(),
            MesgNum::PowerZone => todo!(),
            MesgNum::MetZone => todo!(),
            MesgNum::Goal => todo!(),
            MesgNum::Session => todo!(),
            MesgNum::Schedule => todo!(),
            MesgNum::WeightScale => todo!(),
            MesgNum::Course => todo!(),
            MesgNum::CoursePoint => todo!(),
            MesgNum::Totals => todo!(),
            MesgNum::Activity => todo!(),
            MesgNum::Software => todo!(),
            MesgNum::FileCapabilities => todo!(),
            MesgNum::MesgCapabilities => todo!(),
            MesgNum::FieldCapabilities => todo!(),
            MesgNum::BloodPressure => todo!(),
            MesgNum::SpeedZone => todo!(),
            MesgNum::Monitoring => todo!(),
            MesgNum::TrainingFile => todo!(),
            MesgNum::Hrv => todo!(),
            MesgNum::AntRx => todo!(),
            MesgNum::AntTx => todo!(),
            MesgNum::AntChannelId => todo!(),
            MesgNum::Length => todo!(),
            MesgNum::MonitoringInfo => todo!(),
            MesgNum::Pad => todo!(),
            MesgNum::SlaveDevice => todo!(),
            MesgNum::Connectivity => todo!(),
            MesgNum::WeatherConditions => todo!(),
            MesgNum::WeatherAlert => todo!(),
            MesgNum::CadenceZone => todo!(),
            MesgNum::Hr => todo!(),
            MesgNum::SegmentLap => todo!(),
            MesgNum::MemoGlob => todo!(),
            MesgNum::SegmentId => todo!(),
            MesgNum::SegmentLeaderboardEntry => todo!(),
            MesgNum::SegmentPoint => todo!(),
            MesgNum::SegmentFile => todo!(),
            MesgNum::WorkoutSession => todo!(),
            MesgNum::WatchfaceSettings => todo!(),
            MesgNum::GpsMetadata => todo!(),
            MesgNum::CameraEvent => todo!(),
            MesgNum::TimestampCorrelation => todo!(),
            MesgNum::GyroscopeData => todo!(),
            MesgNum::AccelerometerData => todo!(),
            MesgNum::ThreeDSensorCalibration => todo!(),
            MesgNum::VideoFrame => todo!(),
            MesgNum::ObdiiData => todo!(),
            MesgNum::NmeaSentence => todo!(),
            MesgNum::AviationAttitude => todo!(),
            MesgNum::Video => todo!(),
            MesgNum::VideoTitle => todo!(),
            MesgNum::VideoDescription => todo!(),
            MesgNum::VideoClip => todo!(),
            MesgNum::OhrSettings => todo!(),
            MesgNum::ExdScreenConfiguration => todo!(),
            MesgNum::ExdDataFieldConfiguration => todo!(),
            MesgNum::ExdDataConceptConfiguration => todo!(),
            MesgNum::MagnetometerData => todo!(),
            MesgNum::BarometerData => todo!(),
            MesgNum::OneDSensorCalibration => todo!(),
            MesgNum::MonitoringHrData => todo!(),
            MesgNum::TimeInZone => todo!(),
        }
    }
}

impl WorkoutSession {
    pub fn default() -> WorkoutSession {
        WorkoutSession {
            name: String::from(""),
            // workout_type: WorkoutType::Cycling,
            distance: None,
            moving_time: None,
            total_time: 0,
            start_date_time: chrono::Utc::now(),
            average_speed: None,
            max_speed: None,
            average_heart_rate: None,
            max_heart_rate: None,
            splits: None,
            time_series_data: TimeSeriesData::default(),
            power_zone_distribution: vec![],
            power_curve: vec![],
        }
    }
}