mod db;
mod power_curve;
mod structures;

use bson::to_document;
use db::DB;
use fitparser::{from_reader, profile::MesgNum};
use power_curve::calculate_power_curve;
use std::{collections::BTreeMap, sync::Arc};
use structures::*;
use tower_http::cors::CorsLayer;

use axum::{
    extract::{Multipart, Path, State},
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, StatusCode,
    },
    response::{Json, Response},
    routing::post,
    Router,
};
use axum::{handler::Handler, response::IntoResponse};

#[derive(Debug, serde::Serialize)]
struct UploadResponse {
    message: String,
}

async fn process_file(
    Path(user_id): Path<String>,
    State(app_state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Response, StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            let file_bytes = field.bytes().await.unwrap();
            let data = from_reader(&mut file_bytes.as_ref()).map_err(|e| {
                println!("Error parsing file {e:?}");
                StatusCode::BAD_REQUEST
            })?;
            println!("Length of fit file {}", data.len());
            // let mut workout_session = WorkoutSession::default();
            let data: FitDataMap = data.into_iter().fold(BTreeMap::new(), merge_by_kind);
            let power_data: Vec<u64> = data
                .get(&MesgNum::Record)
                .and_then(|x| {
                    Some(
                        x.iter()
                            .map(|entry| {
                                let value: i64 = entry
                                    .get("power")
                                    .and_then(|v| v.value.to_owned().try_into().ok())
                                    .unwrap_or_default();
                                value as u64
                            })
                            .collect(),
                    )
                })
                .unwrap_or_default();
            let mongo_doc = MongoSchema {
                user_id: user_id.clone(),
                fit_data: data,
                power_curve: calculate_power_curve(&power_data),
            };
            let document = to_document(&mongo_doc).map_err(|e| {
                println!("Error converting to document {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            app_state
                .db
                .collection
                .insert_one(document, None)
                .await
                .map_err(|e| {
                    println!("Error inserting into db {e:?}");
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
        }
    }

    // Return a response
    // Ok(Json(UploadResponse {
    //     message: "File processed successfully".to_string(),
    // }))
    Ok(Response::default())
    // .status(StatusCode::CREATED)
    // .body(boxed("OK".to_string()))
    // .unwrap());
}

// #[axum_macros::debug_handler]
// async fn process_file(
//     Path(user_id): Path<String>,
//     State(app_state): State<Arc<AppState>>,
//     mut multipart: Multipart,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     Ok((
//         StatusCode::CREATED,
//         Json(UploadResponse {
//             message: "File processed successfully".to_string(),
//         }),
//     ))
// }

#[derive(Clone)]
pub struct AppState {
    db: DB,
}

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    let db = DB::init().await?;

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = Router::new()
        .route(
            "/analytics-api/:user_id/upload_activity",
            post(process_file),
        )
        .with_state(Arc::new(AppState { db: db.clone() }))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
    // let file = "/Users/y/Downloads/2024-01-25-workout.fit";
    // // let file = "/Users/y/Downloads/2024-01-09-125540-Indoor Cycling.fit";

    // let mut fp = File::open(&file)?;

    // // .collect();
    // println!("COLLECTED POWER DATA");
    // let power_curve = calculate_power_curve(&power_data);
    // println!("Parsed power curve");
    // let mut pwr = File::create("power_curve.json")?;
    // pwr.write_all(serde_json::to_string(&power_curve)?.as_bytes())?;

    // let mut total = File::create("data.json")?;
    // total.write_all(serde_json::to_string(&data)?.as_bytes())?;

    // plot_power_curve(&power_curve.as_slice())?;

    // Ok(())
    // Optionally, export data for plotting
}

// use plotters::prelude::*;

// fn plot_power_curve(data: &[(usize, f32)]) -> Result<(), Box<dyn std::error::Error>> {
//     let root = BitMapBackend::new("power_curve.png", (640, 480)).into_drawing_area();
//     root.fill(&WHITE)?;

//     let max_duration = data.last().map(|x| x.0).unwrap_or(0);
//     let max_power = data.iter().map(|x| x.1).fold(0.0_f32, |a, b| a.max(b));

//     let mut chart = ChartBuilder::on(&root)
//         .caption("Power Curve", ("sans-serif", 40).into_font())
//         .margin(15)
//         .x_label_area_size(45)
//         .y_label_area_size(45)
//         .build_cartesian_2d(0..max_duration, 0.0_f32..max_power)?;

//     chart.configure_mesh().draw()?;

//     chart.draw_series(LineSeries::new(data.iter().map(|&(x, y)| (x, y)), &RED))?;

//     root.present()?;
//     Ok(())
// }

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
