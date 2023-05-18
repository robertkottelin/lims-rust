use crate::models::InventoryItem;
use rusqlite::{params, Connection, Result};

pub fn add_inventory_item(conn: &Connection, item: &InventoryItem) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO inventory (id, name, quantity) VALUES (?1, ?2, ?3)",
        params![item.id, item.name, item.quantity],
    )?;

    Ok(())
}

pub fn update_inventory_item_quantity(
    conn: &Connection,
    item_id: i32,
    new_quantity: i32,
) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "UPDATE inventory SET quantity = ?1 WHERE id = ?2",
        params![new_quantity, item_id],
    )
}

pub fn delete_inventory_item(conn: &Connection, item_id: i32) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM inventory WHERE id = ?1", params![item_id])
}
