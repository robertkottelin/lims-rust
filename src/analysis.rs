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

// get all analyses
pub fn get_all_analyses(conn: &Connection) -> Result<Vec<Analysis>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT * FROM analyses")?;
    let analysis_iter = stmt.query_map(params![], |row| {
        Ok(Analysis {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
        })
    })?;

    let mut analyses = Vec::new();
    for analysis in analysis_iter {
        analyses.push(analysis?);
    }

    Ok(analyses)
}

pub fn delete_analysis(conn: &Connection, analysis_id: i32) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM analyses WHERE id = ?1", params![analysis_id])
}
