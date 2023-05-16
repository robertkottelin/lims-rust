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

use env_logger;
use rusqlite::Result;
use db::init_db;
use models::{Sample, Analysis, Instrument};
//use users::add_user;
use inventory::*;
use samples::*;
use analysis::*;
use instruments::*;

fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    log::info!("Starting up");

    let conn = init_db()?;

    let sample = Sample {
        id: 2,
        name: "Sample 1".to_string(),
        description: "Test Sample".to_string(),
    };
    add_sample(&conn, &sample)?;
    //delete_sample(&conn, 1);

    let analysis = Analysis {
        id: 2,
        sample_id: 0,
        result: "Positive".to_string(),
        instrument_id: 1,
    };
    add_analysis(&conn, &analysis)?;
    // delete_analysis(&conn, 1)?;

    let instrument = Instrument {
        id: 2,
        name: "Microscope".to_string(),
        model: "Microscope".to_string(),
        location: "Microscope".to_string(),
    };
    add_instrument(&conn, &instrument)?;
    // delete_instruments(&conn, 1)?;
    log::info!("Shutting down");
    Ok(())
}