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
        "CREATE TABLE IF NOT EXISTS analyses (
            id              INTEGER PRIMARY KEY,
            sample_id       INTEGER NOT NULL,
            instrument_id   INTEGER NOT NULL,
            result          TEXT NOT NULL,
            FOREIGN KEY(sample_id) REFERENCES samples(id),
            FOREIGN KEY(instrument_id) REFERENCES instruments(id)
        )",
        params![],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS instruments (
          id              INTEGER PRIMARY KEY,
          name            TEXT NOT NULL,
          model           TEXT NOT NULL,
          location        TEXT NOT NULL
      )",
        params![],
    )?;

    Ok(conn)}
