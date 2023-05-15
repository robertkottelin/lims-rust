use rusqlite::{params, Connection, Result};

struct Sample {
    id: i32,
    name: String,
    description: String,
}

struct Analysis {
    id: i32,
    sample_id: i32,
    result: String,
}

fn init_db() -> Result<Connection> {
    let conn = Connection::open("lims.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS samples (
            id              INTEGER PRIMARY KEY,
            name            TEXT NOT NULL,
            description     TEXT NOT NULL
        )",
        params![],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS analyses (
            id              INTEGER PRIMARY KEY,
            sample_id       INTEGER NOT NULL,
            result          TEXT NOT NULL,
            FOREIGN KEY(sample_id) REFERENCES samples(id)
        )",
        params![],
    )?;

    Ok(conn)
}

fn add_sample(conn: &Connection, sample: &Sample) -> Result<()> {
    conn.execute(
        "INSERT INTO samples (id, name, description) VALUES (?1, ?2, ?3)",
        params![sample.id, sample.name, sample.description],
    )?;
    Ok(())
}

fn add_analysis(conn: &Connection, analysis: &Analysis) -> Result<()> {
    conn.execute(
        "INSERT INTO analyses (id, sample_id, result) VALUES (?1, ?2, ?3)",
        params![analysis.id, analysis.sample_id, analysis.result],
    )?;
    Ok(())
}

fn query_samples(
    conn: &Connection,
    sample_id: Option<i32>,
) -> Result<Vec<Sample>, rusqlite::Error> {
    let mut stmt = match sample_id {
        Some(id) => conn.prepare("SELECT id, name, description FROM samples WHERE id = ?")?,
        None => conn.prepare("SELECT id, name, description FROM samples")?,
    };

    let rows = match sample_id {
        Some(id) => stmt.query_map(params![id], |row| {
            Ok(Sample {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
            })
        })?,
        None => stmt.query_map([], |row| {
            Ok(Sample {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
            })
        })?,
    };

    let mut samples = Vec::new();
    for sample in rows {
        samples.push(sample?);
    }

    Ok(samples)
}

fn update_sample_description(
    conn: &Connection,
    sample_id: i32,
    new_description: String,
) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "UPDATE samples SET description = ?1 WHERE id = ?2",
        params![new_description, sample_id],
    )
}

fn delete_sample(conn: &Connection, sample_id: i32) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM samples WHERE id = ?1", params![sample_id])
}

fn add_samples(conn: &Connection, samples: &Vec<Sample>) -> Result<(), rusqlite::Error> {
    let tx = conn.transaction()?;

    for sample in samples {
        tx.execute(
            "INSERT INTO samples (id, name, description) VALUES (?1, ?2, ?3)",
            params![sample.id, sample.name, sample.description],
        )?;
    }

    tx.commit()
}

struct User {
    id: i32,
    username: String,
    password: String, // TODO: hash and .env
    role: String,
}

fn add_user(conn: &Connection, user: &User) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO users (id, username, password, role) VALUES (?1, ?2, ?3, ?4)",
        params![user.id, user.username, user.password, user.role],
    )?;

    Ok(())
}

fn update_user_role(
    conn: &Connection,
    user_id: i32,
    new_role: String,
) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "UPDATE users SET role = ?1 WHERE id = ?2",
        params![new_role, user_id],
    )
}

fn delete_user(conn: &Connection, user_id: i32) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM users WHERE id = ?1", params![user_id])
}

struct InventoryItem {
    id: i32,
    name: String,
    quantity: i32,
}

fn add_inventory_item(conn: &Connection, item: &InventoryItem) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO inventory (id, name, quantity) VALUES (?1, ?2, ?3)",
        params![item.id, item.name, item.quantity],
    )?;

    Ok(())
}

fn update_inventory_item_quantity(
    conn: &Connection,
    item_id: i32,
    new_quantity: i32,
) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "UPDATE inventory SET quantity = ?1 WHERE id = ?2",
        params![new_quantity, item_id],
    )
}

fn delete_inventory_item(conn: &Connection, item_id: i32) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM inventory WHERE id = ?1", params![item_id])
}

struct Test {
    id: i32,
    name: String,
    description: String,
}

struct Schedule {
    id: i32,
    sample_id: i32,
    test_id: i32,
    scheduled_time: String, // Note: you'd likely want to use a DateTime type in a real application
}

fn add_test(conn: &Connection, test: &Test) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO tests (id, name, description) VALUES (?1, ?2, ?3)",
        params![test.id, test.name, test.description],
    )?;

    Ok(())
}

fn schedule_test(conn: &Connection, schedule: &Schedule) -> Result<(), rusqlite::Error> {
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

struct QualityControl {
    id: i32,
    sample_id: i32,
    test_id: i32,
    expected_result: String,
    actual_result: String,
}

fn record_quality_control(conn: &Connection, qc: &QualityControl) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO quality_control (id, sample_id, test_id, expected_result, actual_result) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![qc.id, qc.sample_id, qc.test_id, qc.expected_result, qc.actual_result],
    )?;

    Ok(())
}

fn main() -> Result<()> {
    let conn = init_db()?;

    // For simplicity, let's add a sample and an analysis statically
    let sample = Sample {
        id: 1,
        name: "Sample 1".to_string(),
        description: "First Sample".to_string(),
    };
    add_sample(&conn, &sample)?;

    let analysis = Analysis {
        id: 1,
        sample_id: 1,
        result: "Positive".to_string(),
    };
    add_analysis(&conn, &analysis)?;

    Ok(())
}