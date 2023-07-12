use crate::models::{Schedule, Test};
use rusqlite::{params, Connection, Result};

pub fn add_test(conn: &Connection, test: &Test) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO tests (sample_name, analysis_name, instrument_name, result) VALUES (?1, ?2, ?3, ?4)",
        params![test.sample_name, test.analysis_name, test.instrument_name, test.result],
    )?;

    Ok(())
}

pub fn get_all_tests(conn: &Connection) -> Result<Vec<Test>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT * FROM tests")?;
    let test_iter = stmt.query_map(params![], |row| {
        Ok(Test {
            id: row.get(0)?,
            sample_name: row.get(1)?,
            analysis_name: row.get(2)?,
            instrument_name: row.get(3)?,
            result: row.get(4)?,
        })
    })?;

    let mut tests = Vec::new();
    for test in test_iter {
        tests.push(test?);
    }

    Ok(tests)
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
