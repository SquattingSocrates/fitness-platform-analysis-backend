use std::{
    collections::{BTreeMap, HashMap},
    convert, fmt,
};

use chrono::{DateTime, Utc};
use fitparser::{profile::MesgNum, FitDataField, FitDataRecord, Value};
use serde::{Deserialize, Serialize};

pub type FitDataMap = BTreeMap<MesgNum, Vec<BTreeMap<String, ValueWithUnitsName>>>;

#[derive(Clone, Debug, Serialize)]
struct FitDataList {
    kind: fitparser::profile::MesgNum,
    fields: BTreeMap<String, ValueWithUnitsName>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MongoSchema {
    pub user_id: String,
    pub fit_data: FitDataMap,
    pub power_curve: Vec<(usize, f32)>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ValueWithUnitsName {
    pub value: Value,
    pub units: String,
}

impl fmt::Display for ValueWithUnitsName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.units.is_empty() {
            write!(f, "{}", self.value)
        } else {
            write!(f, "{} {}", self.value, self.units)
        }
    }
}

impl convert::From<FitDataField> for ValueWithUnitsName {
    fn from(field: FitDataField) -> Self {
        ValueWithUnitsName {
            value: field.value().clone(),
            units: field.units().to_owned(),
        }
    }
}

impl FitDataList {
    fn new(record: fitparser::FitDataRecord) -> Self {
        FitDataList {
            kind: record.kind(),
            fields: record
                .into_vec()
                .into_iter()
                .map(|f| (f.name().to_owned(), ValueWithUnitsName::from(f)))
                .collect(),
        }
    }
}

pub fn merge_by_kind(mut map: FitDataMap, record: fitparser::FitDataRecord) -> FitDataMap {
    map.entry(record.kind()).or_insert_with(Vec::new).push(
        record
            .into_vec()
            .into_iter()
            .map(|f| (f.name().to_owned(), ValueWithUnitsName::from(f)))
            .collect(),
    );
    map
}

macro_rules! get_field_from_iter {
    ($iter:expr, $field_name:expr, $default_type:ty, $output_type:ty, $transform:expr, $default_str:expr) => {
        $iter
            .find(|f| f.name() == $field_name)
            .and_then(|f| {
                let value: $default_type = f.value().to_owned().try_into().unwrap();
                let units = f.units().to_owned();
                Some((value as $output_type, units))
            })
            .unwrap_or((<$output_type>::default(), $default_str.to_owned()))
            .into()
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkoutType {
    Cycling,
    Run,
    WeightTraining,
}

#[derive(Serialize, Debug, Clone)]
pub struct ValueWithUnit<T> {
    pub value: T,
    pub units: String,
}

impl<T, S> From<(T, S)> for ValueWithUnit<T>
where
    S: Into<String>,
{
    fn from((value, units): (T, S)) -> Self {
        ValueWithUnit {
            value,
            units: units.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Split {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub name: Option<String>,
    // ... other fields specific to the type of split ...
}

#[derive(Serialize, Debug, Clone)]
pub struct Record {
    pub accumulated_power: ValueWithUnit<u32>, // UInt32
    pub power: ValueWithUnit<u16>,             // UInt16
    pub timestamp: DateTime<Utc>,              // Timestamp
    pub fractional_cadence: ValueWithUnit<f64>,
    pub distance: ValueWithUnit<f64>,
    pub heart_rate: ValueWithUnit<u8>,
    pub position_long: ValueWithUnit<i32>,
    pub cadence: ValueWithUnit<u8>,
    pub position_lat: ValueWithUnit<i32>,
    pub enhanced_altitude: ValueWithUnit<f64>,
    pub gps_accuracy: ValueWithUnit<u8>,
    pub enhanced_speed: ValueWithUnit<f64>,
}

impl Record {
    pub fn from_fitentry(entry: &FitDataRecord) -> Self {
        let mut fields = entry.fields().iter();
        Record {
            cadence: get_field_from_iter!(fields, "cadence", i64, u8, value_to_i64, "rpm"),
            accumulated_power: get_field_from_iter!(
                fields,
                "accumulated_power",
                i64,
                u32,
                value_to_i64,
                "W"
            ),
            power: get_field_from_iter!(fields, "power", i64, u16, value_to_i64, "W"),
            timestamp: fields
                .find(|f| f.name() == "timestamp")
                .and_then(|f| match f.value().to_owned() {
                    Value::Timestamp(t) => Some(t.into()),
                    _ => None,
                })
                .unwrap_or_else(|| Utc::now()),
            fractional_cadence: get_field_from_iter!(
                fields,
                "fractional_cadence",
                f64,
                f64,
                value_to_f64,
                "rpm"
            ),
            distance: get_field_from_iter!(fields, "distance", f64, f64, value_to_f64, "m"),
            heart_rate: get_field_from_iter!(fields, "heart_rate", i64, u8, value_to_i64, "bpm"),
            position_long: get_field_from_iter!(
                fields,
                "position_long",
                i64,
                i32,
                value_to_i64,
                "semicircles"
            ),
            position_lat: get_field_from_iter!(
                fields,
                "position_lat",
                i64,
                i32,
                value_to_i64,
                "semicircles"
            ),
            enhanced_altitude: get_field_from_iter!(
                fields,
                "enhanced_altitude",
                f64,
                f64,
                value_to_f64,
                "m"
            ),
            gps_accuracy: get_field_from_iter!(fields, "gps_accuracy", i64, u8, value_to_i64, "m"),
            enhanced_speed: get_field_from_iter!(
                fields,
                "enhanced_speed",
                f64,
                f64,
                value_to_f64,
                "m/s"
            ),
        }
    }
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
    Record(Record),
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
        num_sessions: ValueWithUnit<u16>,     // UInt16
        timestamp: DateTime<Utc>,             // Timestamp
        total_timer_time: ValueWithUnit<f64>, // Float64
        type_: String,
    },
    Other,
}

fn value_to_string(field: &FitDataField) -> Option<String> {
    match field.value().to_owned() {
        Value::String(s) => Some(s.to_owned()),
        _ => None,
    }
}

fn value_to_i64(field: &FitDataField) -> Option<i64> {
    field.value().try_into().ok()
}

fn value_to_f64(field: &FitDataField) -> Option<f64> {
    field.value().to_owned().try_into().ok()
}

fn value_to_units(field: &FitDataField) -> Option<ValueWithUnit<f64>> {
    Some(ValueWithUnit {
        value: field.value().to_owned().try_into().unwrap(),
        units: field.units().to_owned(),
    })
}

fn to_timestamp(field: &FitDataField) -> Option<DateTime<Utc>> {
    match field.value().to_owned() {
        Value::Timestamp(t) => Some(t.into()),
        _ => None,
    }
}

macro_rules! extract_field {
    ($record:expr, $field_name:expr, $default_type:ty, $transform:expr) => {
        FitEntry::get_field($record, $field_name)
            .and_then($transform)
            .unwrap_or_else(|| <$default_type>::default())
    };
}

macro_rules! extract_value_with_unit {
    ($record:expr, $field_name:expr, $try_into_type:ty, $output_type:ty, $default_unit:expr) => {{
        FitEntry::get_field($record, $field_name)
            .and_then(|f| {
                let value: $try_into_type = f.value().to_owned().try_into().unwrap();
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

macro_rules! extract_fields {
    ($record:expr, [$(($field_name:expr, $default_type:ty, $transform:expr)),*]) => {{
        let mut result = ($(<$default_type>::default()),*);

        for field in $record.fields() {
            match field.name() {
                $(
                    $field_name => {
                        let transformed = $transform(field).unwrap_or_else(|| <$default_type>::default());
                        result.$n = transformed;
                    }
                )*
                _ => {}
            }
        }

        result
    }};
}

impl FitEntry {
    pub fn get_field<'a>(record: &'a FitDataRecord, field_name: &str) -> Option<&'a FitDataField> {
        record.fields().into_iter().find(|f| f.name() == field_name)
    }

    pub fn new(record: fitparser::FitDataRecord) -> Self {
        match record.kind() {
            MesgNum::FileId => FitEntry::FileId {
                manufacturer: extract_field!(&record, "manufacturer", String, value_to_string),
                product_name: extract_field!(&record, "product_name", String, value_to_string),
                serial_number: extract_field!(&record, "serial_number", i64, value_to_i64) as u32,
                time_created: extract_field!(&record, "time_created", DateTime<Utc>, to_timestamp),
                file_type: extract_field!(&record, "file_type", String, value_to_string),
            },
            MesgNum::FileCreator => FitEntry::FileCreator {
                software_version: extract_field!(&record, "software_version", i64, value_to_i64)
                    as u16,
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
                num_valid_steps: extract_field!(&record, "num_valid_steps", i64, value_to_i64)
                    as u16,
                sport: extract_field!(&record, "sport", String, value_to_string),
                wkt_name: extract_field!(&record, "wkt_name", String, value_to_string),
            },
            MesgNum::WorkoutStep => FitEntry::WorkoutStep {
                duration_time: extract_value_with_unit!(&record, "duration_time", f64, f64, ""),
                duration_type: extract_field!(&record, "duration_type", String, value_to_string),
                intensity: extract_field!(&record, "intensity", String, value_to_string),
                message_index: extract_field!(&record, "message_index", i64, value_to_i64),
                target_type: extract_field!(&record, "target_type", String, value_to_string),
                target_value: extract_field!(&record, "target_value", i64, value_to_i64) as u32,
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
                functional_threshold_power: extract_value_with_unit!(
                    &record,
                    "functional_threshold_power",
                    f64,
                    f64,
                    "W"
                ),
                pwr_calc_type: extract_field!(&record, "pwr_calc_type", String, value_to_string),
            },
            MesgNum::Record => FitEntry::Record(Record::from_fitentry(&record)),
            MesgNum::Lap => FitEntry::Lap {
                avg_cadence: extract_value_with_unit!(&record, "avg_cadence", f64, f64, "rpm"),
                avg_fractional_cadence: extract_value_with_unit!(
                    &record,
                    "avg_fractional_cadence",
                    f64,
                    f64,
                    "rpm"
                ),
                avg_heart_rate: extract_value_with_unit!(&record, "avg_heart_rate", f64, u8, "bpm"),
                avg_power: extract_value_with_unit!(&record, "avg_power", i64, u16, "W"),
                enhanced_avg_speed: extract_value_with_unit!(
                    &record,
                    "enhanced_avg_speed",
                    f64,
                    f64,
                    "m/s"
                ),
                enhanced_max_altitude: extract_value_with_unit!(
                    &record,
                    "enhanced_max_altitude",
                    f64,
                    f64,
                    "m"
                ),
                enhanced_max_speed: extract_value_with_unit!(
                    &record,
                    "enhanced_max_speed",
                    f64,
                    f64,
                    "m/s"
                ),
                enhanced_min_altitude: extract_value_with_unit!(
                    &record,
                    "enhanced_min_altitude",
                    f64,
                    f64,
                    "m"
                ),
                event: extract_field!(&record, "event", String, value_to_string),
                event_type: extract_field!(&record, "event_type", String, value_to_string),
                intensity: extract_field!(&record, "intensity", String, value_to_string),
                max_cadence: extract_value_with_unit!(&record, "max_cadence", f64, f64, "rpm"),
                max_fractional_cadence: extract_value_with_unit!(
                    &record,
                    "max_fractional_cadence",
                    f64,
                    f64,
                    "rpm"
                ),
                max_heart_rate: extract_value_with_unit!(&record, "max_heart_rate", i64, u8, "bpm"),
                max_power: extract_value_with_unit!(&record, "max_power", i64, u16, "W"),
                message_index: extract_field!(&record, "message_index", i64, value_to_i64),
                min_heart_rate: extract_value_with_unit!(&record, "min_heart_rate", i64, u8, "bpm"),
                sport: extract_field!(&record, "sport", String, value_to_string),
                start_time: extract_field!(&record, "start_time", DateTime<Utc>, to_timestamp),
                sub_sport: extract_field!(&record, "sub_sport", String, value_to_string),
                timestamp: extract_field!(&record, "timestamp", DateTime<Utc>, to_timestamp),
                total_calories: extract_value_with_unit!(
                    &record,
                    "total_calories",
                    i64,
                    u16,
                    "kcal"
                ),
                total_distance: extract_value_with_unit!(&record, "total_distance", f64, f64, "m"),
                total_elapsed_time: extract_value_with_unit!(
                    &record,
                    "total_elapsed_time",
                    f64,
                    f64,
                    "s"
                ),
                total_timer_time: extract_value_with_unit!(
                    &record,
                    "total_timer_time",
                    f64,
                    f64,
                    "s"
                ),
                wkt_step_index: extract_field!(&record, "wkt_step_index", i64, value_to_i64),
            },
            MesgNum::Activity => FitEntry::Activity {
                event: extract_field!(&record, "event", String, value_to_string),
                event_type: extract_field!(&record, "event_type", String, value_to_string),
                local_timestamp: extract_field!(
                    &record,
                    "local_timestamp",
                    DateTime<Utc>,
                    to_timestamp
                ),
                num_sessions: extract_value_with_unit!(&record, "num_sessions", i64, u16, ""),
                timestamp: extract_field!(&record, "timestamp", DateTime<Utc>, to_timestamp),
                total_timer_time: extract_value_with_unit!(
                    &record,
                    "total_timer_time",
                    f64,
                    f64,
                    "s"
                ),
                type_: extract_field!(&record, "type", String, value_to_string),
            },
            MesgNum::Session => FitEntry::Session {
                avg_cadence: extract_value_with_unit!(&record, "avg_cadence", f64, f64, "rpm"),
                avg_fractional_cadence: extract_value_with_unit!(
                    &record,
                    "avg_fractional_cadence",
                    f64,
                    f64,
                    "rpm"
                ),
                avg_heart_rate: extract_value_with_unit!(
                    &record,
                    "avg_heart_rate",
                    f64,
                    f64,
                    "bpm"
                ),
                avg_power: extract_value_with_unit!(&record, "avg_power", f64, f64, "W"),
                avg_temperature: extract_value_with_unit!(
                    &record,
                    "avg_temperature",
                    i64,
                    f64,
                    "°C"
                ),
                enhanced_avg_altitude: extract_value_with_unit!(
                    &record,
                    "enhanced_avg_altitude",
                    f64,
                    f64,
                    "m"
                ),
                enhanced_avg_speed: extract_value_with_unit!(
                    &record,
                    "enhanced_avg_speed",
                    f64,
                    f64,
                    "m/s"
                ),
                enhanced_max_altitude: extract_value_with_unit!(
                    &record,
                    "enhanced_max_altitude",
                    f64,
                    f64,
                    "m"
                ),
                enhanced_max_speed: extract_value_with_unit!(
                    &record,
                    "enhanced_max_speed",
                    f64,
                    f64,
                    "m/s"
                ),
                enhanced_min_altitude: extract_value_with_unit!(
                    &record,
                    "enhanced_min_altitude",
                    f64,
                    f64,
                    "m"
                ),
                event_type: extract_field!(&record, "event_type", String, value_to_string),
                first_lap_index: extract_value_with_unit!(&record, "first_lap_index", i64, f64, ""),
                max_cadence: extract_value_with_unit!(&record, "max_cadence", f64, f64, "rpm"),
                max_fractional_cadence: extract_value_with_unit!(
                    &record,
                    "max_fractional_cadence",
                    f64,
                    f64,
                    "rpm"
                ),
                max_heart_rate: extract_value_with_unit!(
                    &record,
                    "max_heart_rate",
                    f64,
                    f64,
                    "bpm"
                ),
                max_power: extract_value_with_unit!(&record, "max_power", f64, f64, "W"),
                message_index: extract_field!(&record, "message_index", i64, value_to_i64),
                min_heart_rate: extract_value_with_unit!(
                    &record,
                    "min_heart_rate",
                    f64,
                    f64,
                    "bpm"
                ),
                nec_lat: extract_value_with_unit!(&record, "nec_lat", i64, f64, "semicircles"),
                nec_long: extract_value_with_unit!(&record, "nec_long", i64, f64, "semicircles"),
                num_laps: extract_value_with_unit!(&record, "num_laps", i64, f64, ""),
                sport: extract_field!(&record, "sport", String, value_to_string),
                start_time: extract_field!(&record, "start_time", DateTime<Utc>, to_timestamp),
                sub_sport: extract_field!(&record, "sub_sport", String, value_to_string),
                swc_lat: extract_value_with_unit!(&record, "swc_lat", i64, f64, "semicircles"),
                swc_long: extract_value_with_unit!(&record, "swc_long", i64, f64, "semicircles"),
                threshold_power: extract_value_with_unit!(
                    &record,
                    "threshold_power",
                    i64,
                    f64,
                    "W"
                ),
                timestamp: extract_field!(&record, "timestamp", DateTime<Utc>, to_timestamp),
                total_ascent: extract_value_with_unit!(&record, "total_ascent", i64, f64, "m"),
                total_calories: extract_value_with_unit!(
                    &record,
                    "total_calories",
                    i64,
                    f64,
                    "kcal"
                ),
                total_distance: extract_value_with_unit!(&record, "total_distance", f64, f64, "m"),
                total_elapsed_time: extract_value_with_unit!(
                    &record,
                    "total_elapsed_time",
                    f64,
                    f64,
                    "s"
                ),
                total_timer_time: extract_value_with_unit!(
                    &record,
                    "total_timer_time",
                    f64,
                    f64,
                    "s"
                ),
                trigger: extract_field!(&record, "trigger", String, value_to_string),
            },
            // TODO: this is useful
            MesgNum::Set => FitEntry::Other,
            MesgNum::StressLevel => FitEntry::Other,
            MesgNum::MaxMetData => FitEntry::Other,
            MesgNum::DiveSettings => FitEntry::Other,
            MesgNum::DiveGas => FitEntry::Other,
            MesgNum::DiveAlarm => FitEntry::Other,
            MesgNum::ExerciseTitle => FitEntry::Other,
            MesgNum::DiveSummary => FitEntry::Other,
            MesgNum::Spo2Data => FitEntry::Other,
            MesgNum::SleepLevel => FitEntry::Other,
            MesgNum::Jump => FitEntry::Other,
            MesgNum::BeatIntervals => FitEntry::Other,
            MesgNum::RespirationRate => FitEntry::Other,
            MesgNum::Split => FitEntry::Other,
            // MesgNum::Split => FitEntry::Split {
            //     start_time: FitEntry::get_field(&record, "start_time")
            //         .and_then(to_timestamp)
            //         .unwrap_or_else(|| Utc::now()),
            //     end_time: FitEntry::get_field(&record, "end_time")
            //         .and_then(to_timestamp)
            //         .unwrap_or_else(|| Utc::now()),
            //     name: FitEntry::get_field(&record, "name").and_then(value_to_string),
            // },
            MesgNum::ClimbPro => FitEntry::Other,
            MesgNum::TankUpdate => FitEntry::Other,
            MesgNum::TankSummary => FitEntry::Other,
            MesgNum::SleepAssessment => FitEntry::Other,
            MesgNum::HrvStatusSummary => FitEntry::Other,
            MesgNum::HrvValue => FitEntry::Other,
            MesgNum::DeviceAuxBatteryInfo => FitEntry::Other,
            MesgNum::DiveApneaAlarm => FitEntry::Other,
            MesgNum::MfgRangeMin => FitEntry::Other,
            MesgNum::MfgRangeMax => FitEntry::Other,
            MesgNum::Value(_) => FitEntry::Other,
            MesgNum::Capabilities => FitEntry::Other,
            MesgNum::DeviceSettings => FitEntry::Other,
            MesgNum::UserProfile => FitEntry::Other,
            MesgNum::HrmProfile => FitEntry::Other,
            MesgNum::SdmProfile => FitEntry::Other,
            MesgNum::BikeProfile => FitEntry::Other,
            MesgNum::HrZone => FitEntry::Other,
            MesgNum::PowerZone => FitEntry::Other,
            MesgNum::MetZone => FitEntry::Other,
            MesgNum::Goal => FitEntry::Other,
            MesgNum::Session => FitEntry::Other,
            MesgNum::Schedule => FitEntry::Other,
            MesgNum::WeightScale => FitEntry::Other,
            MesgNum::Course => FitEntry::Other,
            MesgNum::CoursePoint => FitEntry::Other,
            MesgNum::Totals => FitEntry::Other,
            MesgNum::Activity => FitEntry::Other,
            MesgNum::Software => FitEntry::Other,
            MesgNum::FileCapabilities => FitEntry::Other,
            MesgNum::MesgCapabilities => FitEntry::Other,
            MesgNum::FieldCapabilities => FitEntry::Other,
            MesgNum::BloodPressure => FitEntry::Other,
            MesgNum::SpeedZone => FitEntry::Other,
            MesgNum::Monitoring => FitEntry::Other,
            MesgNum::TrainingFile => FitEntry::Other,
            MesgNum::Hrv => FitEntry::Other,
            MesgNum::AntRx => FitEntry::Other,
            MesgNum::AntTx => FitEntry::Other,
            MesgNum::AntChannelId => FitEntry::Other,
            MesgNum::Length => FitEntry::Other,
            MesgNum::MonitoringInfo => FitEntry::Other,
            MesgNum::Pad => FitEntry::Other,
            MesgNum::SlaveDevice => FitEntry::Other,
            MesgNum::Connectivity => FitEntry::Other,
            MesgNum::WeatherConditions => FitEntry::Other,
            MesgNum::WeatherAlert => FitEntry::Other,
            MesgNum::CadenceZone => FitEntry::Other,
            MesgNum::Hr => FitEntry::Other,
            MesgNum::SegmentLap => FitEntry::Other,
            MesgNum::MemoGlob => FitEntry::Other,
            MesgNum::SegmentId => FitEntry::Other,
            MesgNum::SegmentLeaderboardEntry => FitEntry::Other,
            MesgNum::SegmentPoint => FitEntry::Other,
            MesgNum::SegmentFile => FitEntry::Other,
            MesgNum::WorkoutSession => FitEntry::Other,
            MesgNum::WatchfaceSettings => FitEntry::Other,
            MesgNum::GpsMetadata => FitEntry::Other,
            MesgNum::CameraEvent => FitEntry::Other,
            MesgNum::TimestampCorrelation => FitEntry::Other,
            MesgNum::GyroscopeData => FitEntry::Other,
            MesgNum::AccelerometerData => FitEntry::Other,
            MesgNum::ThreeDSensorCalibration => FitEntry::Other,
            MesgNum::VideoFrame => FitEntry::Other,
            MesgNum::ObdiiData => FitEntry::Other,
            MesgNum::NmeaSentence => FitEntry::Other,
            MesgNum::AviationAttitude => FitEntry::Other,
            MesgNum::Video => FitEntry::Other,
            MesgNum::VideoTitle => FitEntry::Other,
            MesgNum::VideoDescription => FitEntry::Other,
            MesgNum::VideoClip => FitEntry::Other,
            MesgNum::OhrSettings => FitEntry::Other,
            MesgNum::ExdScreenConfiguration => FitEntry::Other,
            MesgNum::ExdDataFieldConfiguration => FitEntry::Other,
            MesgNum::ExdDataConceptConfiguration => FitEntry::Other,
            MesgNum::MagnetometerData => FitEntry::Other,
            MesgNum::BarometerData => FitEntry::Other,
            MesgNum::OneDSensorCalibration => FitEntry::Other,
            MesgNum::MonitoringHrData => FitEntry::Other,
            MesgNum::TimeInZone => FitEntry::Other,
        }
    }
}
