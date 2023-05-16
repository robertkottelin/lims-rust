use serde::Serialize;

#[derive(Serialize)]
pub struct Sample {
    id: i32,
    name: String,
    description: String,
}
    
pub struct Analysis {
    id: i32,
    sample_id: i32,
    instrument_id: i32, // new field
    result: String,
}

pub struct User {
    id: i32,
    username: String,
    password: String, // TODO: hash and .env
    role: String,
}

pub struct InventoryItem {
    id: i32,
    name: String,
    quantity: i32,
}


pub struct Test {
    id: i32,
    name: String,
    description: String,
}

pub struct Schedule {
    id: i32,
    sample_id: i32,
    test_id: i32,
    scheduled_time: String, // Note: you'd likely want to use a DateTime type in a real application
}

pub struct QualityControl {
    id: i32,
    sample_id: i32,
    test_id: i32,
    expected_result: String,
    actual_result: String,
}

pub struct Instrument {
    id: i32,
    name: String,
    model: String,
    location: String,
}