use rusqlite::{params, Connection, Result};
use crate::models::Instrument;

pub fn add_instrument(conn: &Connection, instrument: &Instrument) -> Result<()> {
    conn.execute(
        "INSERT INTO instruments (id, name, model, location) VALUES (?1, ?2, ?3, ?4)",
        params![
            instrument.id,
            instrument.name,
            instrument.model,
            instrument.location
        ],
    )?;
    Ok(())
}

pub fn delete_instruments(conn: &Connection, instrument_id: i32) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM instruments WHERE id = ?1", params![instrument_id])
}