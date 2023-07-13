use crate::models::{Schedule, Test, TestInput};
use rusqlite::{params, Connection, Result};

pub fn add_test(conn: &Connection, test_input: &TestInput) -> Result<(), rusqlite::Error> {
    let sample_id: i32 = conn.query_row(
        "SELECT id FROM samples WHERE name = ?1",
        params![test_input.sample_name],
        |row| row.get(0),
    )?;

    let analysis_id: i32 = conn.query_row(
        "SELECT id FROM analyses WHERE name = ?1",
        params![test_input.analysis_name],
        |row| row.get(0),
    )?;

    let instrument_id: i32 = conn.query_row(
        "SELECT id FROM instruments WHERE name = ?1",
        params![test_input.instrument_name],
        |row| row.get(0),
    )?;

    let test = Test {
        id: None,
        sample_id,
        sample_name: test_input.sample_name.clone(),
        analysis_id,
        analysis_name: test_input.analysis_name.clone(),
        instrument_id,
        instrument_name: test_input.instrument_name.clone(),
        result: test_input.result.clone(),
    };

    conn.execute(
        "INSERT INTO tests (sample_id, sample_name, analysis_id, analysis_name, instrument_id, instrument_name, result) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![test.sample_id, test.sample_name, test.analysis_id, test.analysis_name, test.instrument_id, test.instrument_name, test.result],
    )?;

    Ok(())
}

pub fn get_all_tests(conn: &Connection) -> Result<Vec<Test>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT * FROM tests")?;
    let test_iter = stmt.query_map(params![], |row| {
        Ok(Test {
            id: row.get(0)?,
            sample_id: row.get(1)?,
            sample_name: row.get(2)?,
            analysis_id: row.get(3)?,
            analysis_name: row.get(4)?,
            instrument_id: row.get(5)?,
            instrument_name: row.get(6)?,
            result: row.get(7)?,
        })
    })?;

    let mut tests = Vec::new();
    for test in test_iter {
        tests.push(test?);
    }

    Ok(tests)
}
