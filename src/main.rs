use std::sync::{Arc, Mutex};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

#[derive(Debug)]
struct AppState {
    task_array: Vec<String>,
}

#[get("/api/list-task")]
async fn list_tasks(app: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let app = app.lock().unwrap();
    let json_response = json!({
        "tasks": app.task_array
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/api/add-task")]
async fn add_task(app: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let mut app = app.lock().unwrap();
    app.task_array.push("Hello".to_string());
    print!("{:#?}", app);
    HttpResponse::Ok().json(json!({
        "message": "Task added successfully!",
        "tasks": app.task_array
    }))
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(Arc::new(Mutex::new(AppState {
        task_array: Vec::new(),
    })));
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(list_tasks)
            .service(add_task)
            .route("/health", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
