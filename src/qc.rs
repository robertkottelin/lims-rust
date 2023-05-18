use crate::models::QualityControl;
use rusqlite::{params, Connection, Result};

pub fn record_quality_control(
    conn: &Connection,
    qc: &QualityControl,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO quality_control (id, sample_id, test_id, expected_result, actual_result) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![qc.id, qc.sample_id, qc.test_id, qc.expected_result, qc.actual_result],
    )?;

    Ok(())
}
