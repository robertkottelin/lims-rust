use csv::Writer;
use rusqlite::{params, Connection, Result};

use crate::samples::get_samples;

pub fn export_samples(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let samples = get_samples(conn, None)?;

    let mut wtr = Writer::from_path("samples.csv")?;

    for sample in samples {
        wtr.serialize(sample)?;
    }

    wtr.flush()?;

    Ok(())
}
