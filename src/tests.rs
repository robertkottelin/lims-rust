use crate::models::{Schedule, Test};
use rusqlite::{params, Connection, Result};

pub fn add_test(conn: &Connection, test: &Test) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO tests (id, name, description) VALUES (?1, ?2, ?3)",
        params![test.id, test.name, test.description],
    )?;

    Ok(())
}

pub fn schedule_test(conn: &Connection, schedule: &Schedule) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO schedule (id, sample_id, test_id, scheduled_time) VALUES (?1, ?2, ?3, ?4)",
        params![
            schedule.id,
            schedule.sample_id,
            schedule.test_id,
            schedule.scheduled_time
        ],
    )?;

    Ok(())
}
