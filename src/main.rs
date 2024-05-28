use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use uuid::Uuid;

use diesel::prelude::*;
use rust_actixv1::establish_connection;
use rust_actixv1::{
    models::{self as actix_models, NewTask, Task},
    schema::tasks,
};

#[derive(Debug, Serialize)]
struct AppState {
    task_array: Vec<TaskObj>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TaskObj {
    id: Option<Uuid>,
    title: String,
    is_completed: Option<bool>,
}

impl TaskObj {
    fn new(title: String, is_completed: Option<bool>) -> Self {
        TaskObj {
            id: Some(Uuid::new_v4()),
            title: title,
            is_completed: is_completed.or(Some(false)),
        }
    }
}

#[get("/api/list-task")]
async fn list_tasks(app: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    use rust_actixv1::schema::tasks::dsl::*;

    let connection = &mut establish_connection();

    let res = tasks
        .limit(5)
        .select(Task::as_select())
        .load(connection)
        .expect("Error loading tasks");

    let json_response = json!({
        "list": res
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/api/add-task")]
async fn add_task(
    app: web::Data<Arc<Mutex<AppState>>>,
    task: web::Json<TaskObj>,
) -> impl Responder {
    let task_obj = NewTask {
        title: &task.title,
        is_completed: task.is_completed.unwrap_or(false),
    };
    // app.task_array.push(task_obj);
    let connection = &mut establish_connection();
    // let task_obj = NewTask {

    diesel::insert_into(rust_actixv1::schema::tasks::table)
        .values(&task_obj)
        .returning(Task::as_returning())
        .get_result(connection)
        .expect("Data fetch error");

    HttpResponse::Ok().json(json!({
        "message": "Task added successfully!"
    }))
}

#[put("/api/update/{id}")]
async fn update_task(
    app: web::Data<Arc<Mutex<AppState>>>,
    task: web::Json<TaskObj>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let mut app = app.lock().unwrap();
    let task_id = path.into_inner();
    let mut found = false;
    for task_obj in &mut app.task_array {
        if task_obj.id == Some(task_id) {
            found = true;
            task_obj.title = task.title.clone();
            task_obj.is_completed = task.is_completed;
        }
    }

    if found {
        HttpResponse::Ok().json(json!({
            "message" : "Task id updated"
        }))
    } else {
        HttpResponse::NotFound().json(json!({
            "error" : "Task with this id not found"
        }))
    }
}

#[delete("/api/delete/{id}")]
async fn delete_task(
    app: web::Data<Arc<Mutex<AppState>>>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let mut app = app.lock().unwrap();
    let id = path.into_inner();
    let length = app.task_array.len();
    app.task_array.retain(|task| task.id != Some(id));

    if length > app.task_array.len() {
        HttpResponse::Ok().json(json!({
            "message" : "Task Has been deleted"
        }))
    } else {
        HttpResponse::NotFound().json(json!({
            "error" : "Task not found"
        }))
    }
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
            .service(update_task)
            .service(delete_task)
            .route("/health", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
