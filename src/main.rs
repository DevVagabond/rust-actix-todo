use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

struct AppState {
    task_array: Vec<String>,
}

#[get("/api/list-task")]
async fn list_tasks(tasks: web::Data<AppState>) -> impl Responder {
    let json_response = json!({
        "tasks": &tasks.task_array
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/api/add-task")]
async fn add_task(app: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(json!({
        "tasks": []
    }))
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let task_array = web::Data::new(AppState {
        task_array: Vec::new(),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(task_array.clone())
            .service(list_tasks)
            .service(add_task)
            .route("/health", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
