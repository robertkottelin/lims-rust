use actix_web::web::Json;
use actix_web::{web, App, HttpResponse, HttpServer};
use rusqlite::{params, Connection, Result};
use rusqlite::{params, Connection, Result, Row};
use serde::{Deserialize, Serialize};

struct Sample {
    id: i32,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct SamplesRequest {
    samples: Vec<SampleRequest>,
}

struct Analysis {
    id: i32,
    sample_id: i32,
    result: String,
}

#[derive(Serialize, Deserialize)]
struct AnalysisRequest {
    sample_id: i32,
    result: String,
}

#[derive(Serialize)]
struct AnalysisResponse {
    id: i32,
    sample_id: i32,
    result: String,
}

#[derive(Serialize, Deserialize)]
struct UserRequest {
    username: String,
    password: String,
    role: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: i32,
    username: String,
    role: String,
}

#[derive(Serialize, Deserialize)]
struct InventoryItemRequest {
    name: String,
    quantity: i32,
}

#[derive(Serialize)]
struct InventoryItemResponse {
    id: i32,
    name: String,
    quantity: i32,
}

#[derive(Serialize, Deserialize)]
struct TestRequest {
    name: String,
    description: String,
}

#[derive(Serialize)]
struct TestResponse {
    id: i32,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct ScheduleRequest {
    sample_id: i32,
    test_id: i32,
    scheduled_time: String,
}

#[derive(Serialize)]
struct ScheduleResponse {
    id: i32,
    sample_id: i32,
    test_id: i32,
    scheduled_time: String,
}

#[derive(Serialize, Deserialize)]
struct QualityControlRequest {
    sample_id: i32,
    test_id: i32,
    expected_result: String,
    actual_result: String,
}

#[derive(Serialize)]
struct QualityControlResponse {
    id: i32,
    sample_id: i32,
    test_id: i32,
    expected_result: String,
    actual_result: String,
}

async fn add_sample_handler(
    conn: web::Data<Connection>,
    item: web::Json<SampleRequest>,
) -> HttpResponse {
    let new_sample = Sample {
        id: 0, // Replace with proper id generation
        name: item.name.clone(),
        description: item.description.clone(),
    };
    match add_sample(&conn, &new_sample) {
        Ok(_) => HttpResponse::Created().json(SampleResponse {
            id: new_sample.id,
            name: new_sample.name,
            description: new_sample.description,
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_samples_handler(conn: web::Data<Connection>) -> HttpResponse {
    match get_samples(&conn, None) {
        Ok(samples) => HttpResponse::Ok().json(
            samples
                .into_iter()
                .map(|sample| SampleResponse {
                    id: sample.id,
                    name: sample.name,
                    description: sample.description,
                })
                .collect::<Vec<SampleResponse>>(),
        ),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn add_analysis_handler(
    conn: web::Data<Connection>,
    item: web::Json<AnalysisRequest>,
) -> HttpResponse {
    let new_analysis = Analysis {
        id: 0, // Replace with proper id generation
        sample_id: item.sample_id,
        result: item.result.clone(),
    };
    match add_analysis(&conn, &new_analysis) {
        Ok(_) => HttpResponse::Created().json(AnalysisResponse {
            id: new_analysis.id,
            sample_id: new_analysis.sample_id,
            result: new_analysis.result,
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn update_sample_description_handler(
    conn: web::Data<Connection>,
    item: web::Path<(i32, String)>,
) -> HttpResponse {
    match update_sample_description(&conn, item.0, item.1.clone()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_sample_handler(conn: web::Data<Connection>, item: web::Path<i32>) -> HttpResponse {
    match delete_sample(&conn, item.into_inner()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn add_samples_handler(
    conn: web::Data<Connection>,
    items: web::Json<SamplesRequest>,
) -> HttpResponse {
    let new_samples = items
        .samples
        .iter()
        .map(|item| Sample {
            id: 0, // Replace with proper id generation
            name: item.name.clone(),
            description: item.description.clone(),
        })
        .collect::<Vec<Sample>>();

    match add_samples(&conn, &new_samples) {
        Ok(_) => HttpResponse::Created().json(
            new_samples
                .into_iter()
                .map(|sample| SampleResponse {
                    id: sample.id,
                    name: sample.name,
                    description: sample.description,
                })
                .collect::<Vec<SampleResponse>>(),
        ),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn add_user_handler(
    conn: web::Data<Connection>,
    item: web::Json<UserRequest>,
) -> HttpResponse {
    let new_user = User {
        id: 0, // Replace with proper id generation
        username: item.username.clone(),
        password: item.password.clone(),
        role: item.role.clone(),
    };
    match add_user(&conn, &new_user) {
        Ok(_) => HttpResponse::Created().json(UserResponse {
            id: new_user.id,
            username: new_user.username,
            role: new_user.role,
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn add_user_handler(
    conn: web::Data<Connection>,
    item: web::Json<UserRequest>,
) -> HttpResponse {
    let new_user = User {
        id: 0, // Replace with proper id generation
        username: item.username.clone(),
        password: item.password.clone(),
        role: item.role.clone(),
    };
    match add_user(&conn, &new_user) {
        Ok(_) => HttpResponse::Created().json(UserResponse {
            id: new_user.id,
            username: new_user.username,
            role: new_user.role,
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn update_user_role_handler(
    conn: web::Data<Connection>,
    item: web::Path<(i32, String)>,
) -> HttpResponse {
    match update_user_role(&conn, item.0, item.1.clone()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_user_handler(conn: web::Data<Connection>, item: web::Path<i32>) -> HttpResponse {
    match delete_user(&conn, item.into_inner()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn add_inventory_item_handler(
    conn: web::Data<Connection>,
    item: web::Json<InventoryItemRequest>,
) -> HttpResponse {
    let new_item = InventoryItem {
        id: 0, // Replace with proper id generation
        name: item.name.clone(),
        quantity: item.quantity,
    };
    match add_inventory_item(&conn, &new_item) {
        Ok(_) => HttpResponse::Created().json(InventoryItemResponse {
            id: new_item.id,
            name: new_item.name,
            quantity: new_item.quantity,
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn update_inventory_item_quantity_handler(
    conn: web::Data<Connection>,
    item: web::Path<(i32, i32)>,
) -> HttpResponse {
    match update_inventory_item_quantity(&conn, item.0, item.1) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_inventory_item_handler(
    conn: web::Data<Connection>,
    item: web::Path<i32>,
) -> HttpResponse {
    match delete_inventory_item(&conn, item.into_inner()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn add_test_handler(
    conn: web::Data<Connection>,
    item: web::Json<TestRequest>,
) -> HttpResponse {
    let new_test = Test {
        id: 0, // Replace with proper id generation
        name: item.name.clone(),
        description: item.description.clone(),
    };
    match add_test(&conn, &new_test) {
        Ok(_) => HttpResponse::Created().json(TestResponse {
            id: new_test.id,
            name: new_test.name,
            description: new_test.description,
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn schedule_test_handler(
    conn: web::Data<Connection>,
    item: web::Json<ScheduleRequest>,
) -> HttpResponse {
    let new_schedule = Schedule {
        id: 0, // Replace with proper id generation
        sample_id: item.sample_id,
        test_id: item.test_id,
        scheduled_time: item.scheduled_time.clone(),
    };
    match schedule_test(&conn, &new_schedule) {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn record_quality_control_handler(
    conn: web::Data<Connection>,
    item: web::Json<QualityControlRequest>,
) -> HttpResponse {
    let new_qc = QualityControl {
        id: 0, // Replace with proper id generation
        sample_id: item.sample_id,
        test_id: item.test_id,
        expected_result: item.expected_result.clone(),
        actual_result: item.actual_result.clone(),
    };
    match record_quality_control(&conn, &new_qc) {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
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

// Create server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(Connection::open_in_memory().unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            // Handlers for Samples
            .service(
                web::resource("/samples")
                    .route(web::post().to(add_sample_handler))
                    .route(web::get().to(get_samples_handler)),
            )
            // Handlers for Analyses
            .service(web::resource("/analyses").route(web::post().to(add_analysis_handler)))
            // Handlers for Users
            .service(
                web::resource("/users")
                    .route(web::post().to(add_user_handler))
                    .route(web::delete().to(delete_user_handler))
                    .route(web::patch().to(update_user_role_handler)),
            )
            // Handlers for Inventory Items
            .service(
                web::resource("/inventory")
                    .route(web::post().to(add_inventory_item_handler))
                    .route(web::delete().to(delete_inventory_item_handler))
                    .route(web::patch().to(update_inventory_item_quantity_handler)),
            )
            // Handlers for Tests
            .service(web::resource("/tests").route(web::post().to(add_test_handler)))
            // Handlers for Schedules
            .service(web::resource("/schedules").route(web::post().to(schedule_test_handler)))
            // Handlers for QualityControl
            .service(
                web::resource("/qualitycontrol")
                    .route(web::post().to(record_quality_control_handler)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
