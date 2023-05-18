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

use analysis::*;
use db::init_db;
use instruments::*;
use models::{Analysis, Instrument, Sample};
use samples::*;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    log::info!("Starting up");

    let conn = Arc::new(Mutex::new(init_db().unwrap()));

    HttpServer::new(move || {
        App::new()
            .data(conn.clone())
            .route("/samples/{id}", web::get().to(get_samples))
            .route("/samples", web::post().to(add_sample))
            .route("/add_instrument", web::post().to(add_instrument))
            .route("/add_analysis", web::post().to(add_analysis))
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
    format!("Sample: {:?}", samples)
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

async fn add_instrument(
    db: web::Data<Arc<Mutex<rusqlite::Connection>>>,
    new_instrument: web::Json<Instrument>,
) -> impl Responder {
    let result = instruments::add_instrument(&*db.lock().unwrap(), &new_instrument.into_inner());
    match result {
        Ok(()) => format!("Instrument created"),
        Err(e) => format!("Failed to create instrument: {}", e),
    }
}

async fn add_analysis(
    db: web::Data<Arc<Mutex<rusqlite::Connection>>>,
    new_analysis: web::Json<Analysis>,
) -> impl Responder {
    let result = analysis::add_analysis(&*db.lock().unwrap(), &new_analysis.into_inner());
    match result {
        Ok(()) => format!("Analysis created"),
        Err(e) => format!("Failed to create analysis: {}", e),
    }
}
