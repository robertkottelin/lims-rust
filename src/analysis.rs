use crate::models::Analysis;
use rusqlite::{params, Connection, Result};

pub fn add_analysis(conn: &Connection, analysis: &Analysis) -> Result<()> {
    conn.execute(
        "INSERT INTO analyses (id, sample_id, instrument_id, result) VALUES (?1, ?2, ?3, ?4)",
        params![
            analysis.id,
            analysis.sample_id,
            analysis.instrument_id,
            analysis.result
        ],
    )?;
    Ok(())
}

pub fn delete_analysis(conn: &Connection, analysis_id: i32) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM analyses WHERE id = ?1", params![analysis_id])
}
