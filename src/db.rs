use rusqlite::{params, Connection, Result};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("lims.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS samples (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL
        );",
        params![],
    )?;
    
    
    conn.execute(
        "INSERT INTO samples (name, description) VALUES (?, ?)",
        params!["Example Sample", "This is an example sample"],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS analyses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name          TEXT NOT NULL,
            description          TEXT NOT NULL
        )",
        params![],
    )?;
    
    conn.execute(
        "INSERT INTO analyses (name, description) VALUES (?, ?)",
        params!["Example Analysis", "This is an example analysis"],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS instruments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name            TEXT NOT NULL,
            model           TEXT NOT NULL,
            location        TEXT NOT NULL
        )",
        params![],
    )?;
    
    conn.execute(
        "INSERT INTO instruments (name, model, location) VALUES (?, ?, ?)",
        params!["Example Instrument", "Model X", "Location Y"],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tests (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sample_id       INTEGER,
            sample_name     TEXT NOT NULL,
            analysis_id     INTEGER,
            analysis_name   TEXT NOT NULL,
            instrument_id   INTEGER,
            instrument_name TEXT NOT NULL,
            result          TEXT NOT NULL,
            FOREIGN KEY (sample_id) REFERENCES samples(id),
            FOREIGN KEY (analysis_id) REFERENCES analyses(id),
            FOREIGN KEY (instrument_id) REFERENCES instruments(id)
        )",
        params![],
    )?;
    
    conn.execute(
        "INSERT INTO tests (sample_id, sample_name, analysis_id, analysis_name, instrument_id, instrument_name, result) 
        VALUES (?, ?, ?, ?, ?, ?, ?)",
        params![1, "Example Sample", 1, "Example Analysis", 1, "Example Instrument", "Positive"],
    )?;
       

    Ok(conn)
}
