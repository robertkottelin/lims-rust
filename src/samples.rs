
use rusqlite::{params, Connection, Result, Row};
use crate::models::Sample;

// Basic CRUD for samples
pub fn add_sample(conn: &Connection, sample: &Sample) -> Result<()> {
    conn.execute(
        "INSERT INTO samples (name, description) VALUES (?1, ?2)",
        params![sample.name, sample.description],
    )?;
    Ok(())
}

pub fn update_sample_description(
    conn: &Connection,
    sample_id: i32,
    new_description: String,
) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "UPDATE samples SET description = ?1 WHERE id = ?2",
        params![new_description, sample_id],
    )
}

pub fn delete_sample(conn: &Connection, sample_id: i32) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM samples WHERE id = ?1", params![sample_id])
}

// Querying and Reporting
pub fn get_samples(conn: &Connection, sample_id: Option<i32>) -> Result<Vec<Sample>, rusqlite::Error> {
    let mut stmt =
        conn.prepare("SELECT id, name, description FROM samples WHERE (?1 IS NULL OR id = ?1)")?;

    let map_row = |row: &Row| {
        Ok(Sample {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
        })
    };

    let rows = match sample_id {
        Some(id) => stmt.query_map(params![id], map_row)?,
        None => stmt.query_map([], map_row)?,
    };

    let mut samples = Vec::new();
    for row in rows {
        samples.push(row?);
    }
    Ok(samples)
}

// Batch Operations
pub fn add_samples(conn: &mut Connection, samples: &Vec<Sample>) -> Result<(), rusqlite::Error> {
    let tx = conn.transaction()?;

    for sample in samples {
        tx.execute(
            "INSERT INTO samples (id, name, description) VALUES (?1, ?2, ?3)",
            params![sample.id, sample.name, sample.description],
        )?;
    }

    tx.commit()?;
    Ok(())
}