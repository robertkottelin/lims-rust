use crate::models::Analysis;
use rusqlite::{params, Connection, Result};

pub fn add_analysis(conn: &Connection, analysis: &Analysis) -> Result<()> {
    conn.execute(
        "INSERT INTO analyses (name, description) VALUES (?1, ?2)",
        params![
            analysis.name,
            analysis.description        ],
    )?;
    Ok(())
}

pub fn delete_analysis(conn: &Connection, analysis_id: i32) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM analyses WHERE id = ?1", params![analysis_id])
}
