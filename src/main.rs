#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]

mod analysis;
mod db;
mod export;
mod instruments;
mod inventory;
mod models;
mod qc;
mod samples;
mod tests;
mod users;

use actix_web::{web, App, HttpServer, Responder};
use env_logger;
use rusqlite::Result;

use db::init_db;
use models::{Sample, Analysis, Instrument};
use samples::*;
use analysis::*;
use instruments::*;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    log::info!("Starting up");

    let conn = Arc::new(Mutex::new(init_db().unwrap())); // Handle this unwrap properly

    HttpServer::new(move || {
        App::new()
            .data(conn.clone()) // Pass database connection to the application so handlers can use it
            .route("/samples/{id}", web::get().to(get_samples)) // Add route for getting a sample
            .route("/samples", web::post().to(add_sample)) // Add route for creating a sample
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_samples(
    db: web::Data<Arc<Mutex<rusqlite::Connection>>>,
    id: web::Path<i32>,
) -> impl Responder {
    let samples = samples::get_samples(&*db.lock().unwrap(), Some(*id));
    format!("Sample: {:?}", samples) // Convert the Sample to a String for now
}

async fn add_sample(
    db: web::Data<Arc<Mutex<rusqlite::Connection>>>,
    new_sample: web::Json<Sample>,
) -> impl Responder {
    let result = samples::add_sample(&*db.lock().unwrap(), &new_sample.into_inner());
    match result {
        Ok(()) => format!("Sample created"),
        Err(e) => format!("Failed to create sample: {}", e),
    }
}
