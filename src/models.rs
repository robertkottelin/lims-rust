use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sample {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Analysis {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String, // TODO: hash and .env
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InventoryItem {
    pub id: i32,
    pub name: String,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pub id: Option<i32>,
    pub sample_id: i32,
    pub sample_name: String,
    pub analysis_id: i32,
    pub analysis_name: String,
    pub instrument_id: i32,
    pub instrument_name: String,
    pub result: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestInput {
    pub sample_name: String,
    pub analysis_name: String,
    pub instrument_name: String,
    pub result: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Schedule {
    pub id: i32,
    pub sample_id: i32,
    pub test_id: i32,
    pub scheduled_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QualityControl {
    pub id: i32,
    pub sample_id: i32,
    pub test_id: i32,
    pub expected_result: String,
    pub actual_result: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Instrument {
    pub id: Option<i32>,
    pub name: String,
    pub model: String,
    pub location: String,
}
