use crate::models::Instrument;
use rusqlite::{params, Connection, Result};

pub fn add_instrument(conn: &Connection, instrument: &Instrument) -> Result<()> {
    conn.execute(
        "INSERT INTO instruments (name, model, location) VALUES (?1, ?2, ?3)",
        params![
            instrument.name,
            instrument.model,
            instrument.location
        ],
    )?;
    Ok(())
}

// get all instruments
pub fn get_all_instruments(conn: &Connection) -> Result<Vec<Instrument>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT * FROM instruments")?;
    let instrument_iter = stmt.query_map(params![], |row| {
        Ok(Instrument {
            id: row.get(0)?,
            name: row.get(1)?,
            model: row.get(2)?,
            location: row.get(3)?,
        })
    })?;

    let mut instruments = Vec::new();
    for instrument in instrument_iter {
        instruments.push(instrument?);
    }

    Ok(instruments)
}

pub fn delete_instruments(conn: &Connection, instrument_id: i32) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "DELETE FROM instruments WHERE id = ?1",
        params![instrument_id],
    )
}
