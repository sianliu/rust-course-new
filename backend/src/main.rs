#[macro_use]
extern crate rocket;

mod task;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{launch, routes};
use task::{load_tasks, save_tasks, Task};

#[get("/tasks")]
fn fetch_tasks() -> Json<Vec<Task>> {
    let tasks = load_tasks();
    Json(tasks)
}

#[post("/tasks", data = "<task>")]
fn create_task(task: Json<Task>) -> Status {
    let mut tasks = load_tasks();
    tasks.push(task.into_inner());
    match save_tasks(&tasks) {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}

#[put("/tasks", data = "<updated_task>")]
fn update_task(updated_task: Json<Task>) -> Status {
    let mut tasks = load_tasks();

    if let Some(index) = tasks.iter().position(|item| item.id == updated_task.id) {
        tasks[index] = updated_task.into_inner();
        match save_tasks(&tasks) {
            Ok(_) => Status::Ok,
            Err(_) => Status::InternalServerError,
        }
    } else {
        Status::NotFound
    }
}

#[delete("/tasks", data = "<task_to_delete>")]
fn delete_task(task_to_delete: Json<Task>) -> Status {
    let mut tasks = load_tasks();

    if let Some(index) = tasks.iter().position(|item| item.id == task_to_delete.id) {
        tasks.remove(index);
        match save_tasks(&tasks) {
            Ok(_) => Status::Ok,
            Err(_) => Status::InternalServerError,
        }
    } else {
        Status::NotFound
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![fetch_tasks, create_task, update_task, delete_task],
    )
}
