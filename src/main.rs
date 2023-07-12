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
use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use serde_json::to_string;

use analysis::*;
use db::init_db;
use instruments::*;
use models::{Analysis, Instrument, Sample, Test};
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
            .route("/sample_id/{id}", web::get().to(get_sample_id))
            .route("/add_sample", web::post().to(add_sample))
            .route("/samples", web::get().to(get_all_samples))
            .route("/add_instrument", web::post().to(add_instrument))
            .route("/instruments", web::get().to(get_all_instruments))
            .route("/add_analysis", web::post().to(add_analysis))
            .route("/analyses", web::get().to(get_all_analyses))     
            .route("/add_test", web::post().to(add_test))
            .route("/tests", web::get().to(get_all_tests))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_sample_id(
    db: web::Data<Arc<Mutex<rusqlite::Connection>>>,
    id: web::Path<i32>,
) -> impl Responder {
    let samples = samples::get_sample_id(&*db.lock().unwrap(), Some(*id));
    format!("Sample: {:?}", samples)
}

async fn get_all_samples(
    db: web::Data<Arc<Mutex<rusqlite::Connection>>>,
) -> HttpResponse {
    let samples = samples::get_all_samples(&*db.lock().unwrap());
    match samples {
        Ok(samples) => HttpResponse::Ok().json(samples),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to get samples: {}", e)),
    }
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

async fn get_all_instruments(
    db: web::Data<Arc<Mutex<rusqlite::Connection>>>,
) -> HttpResponse {
    let instruments = instruments::get_all_instruments(&*db.lock().unwrap());
    match instruments {
        Ok(instruments) => HttpResponse::Ok().json(instruments),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to get instruments: {}", e)),
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

async fn get_all_analyses(
    db: web::Data<Arc<Mutex<rusqlite::Connection>>>,
) -> HttpResponse {
    let analyses = analysis::get_all_analyses(&*db.lock().unwrap());
    match analyses {
        Ok(analyses) => HttpResponse::Ok().json(analyses),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to get analyses: {}", e)),
    }
}

async fn add_test(
    db: web::Data<Arc<Mutex<rusqlite::Connection>>>,
    new_test: web::Json<Test>,
) -> impl Responder {
    let result = tests::add_test(&*db.lock().unwrap(), &new_test.into_inner());
    match result {
        Ok(()) => format!("Test created"),
        Err(e) => format!("Failed to create test: {}", e),
    }
}

async fn get_all_tests(
    db: web::Data<Arc<Mutex<rusqlite::Connection>>>,
) -> HttpResponse {
    let tests = tests::get_all_tests(&*db.lock().unwrap());
    match tests {
        Ok(tests) => HttpResponse::Ok().json(tests),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to get tests: {}", e)),
    }
}