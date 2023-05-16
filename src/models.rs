use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sample {
    pub id: i32,
    pub name: String,
    pub description: String,
}
    
pub struct Analysis {
    pub id: i32,
    pub sample_id: i32,
    pub instrument_id: i32, // new field
    pub result: String,
}

pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String, // TODO: hash and .env
    pub role: String,
}

pub struct InventoryItem {
    pub id: i32,
    pub name: String,
    pub quantity: i32,
}

pub struct Test {
    pub id: i32,
    pub name: String,
    pub description: String,
}

pub struct Schedule {
    pub id: i32,
    pub sample_id: i32,
    pub test_id: i32,
    pub scheduled_time: String, // Note: you'd likely want to use a DateTime type in a real application
}

pub struct QualityControl {
    pub id: i32,
    pub sample_id: i32,
    pub test_id: i32,
    pub expected_result: String,
    pub actual_result: String,
}

pub struct Instrument {
    pub id: i32,
    pub name: String,
    pub model: String,
    pub location: String,
}