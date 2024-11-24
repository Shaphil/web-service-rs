use crate::models::{users_list, Response, User};

use actix_web::{get, web, HttpResponse, Responder};
use std::collections::HashMap;

#[get("/")]
async fn home() -> impl Responder {
    let mut routes = HashMap::new();
    routes.insert("users".to_string(), "/users".to_string());
    routes.insert("user/{id}".to_string(), "/users/1".to_string());

    HttpResponse::Ok().json(routes)
}

#[get("/users/{id}")]
async fn get_user(id: web::Path<i32>) -> impl Responder {
    let users = users_list();
    let user: Option<&User> = users.get((id.clone() - 1) as usize);
    match user {
        None => {
            let response = Response {
                message: format!("User with id: {} not found", id.clone()),
            };
            HttpResponse::NotFound().json(response)
        }
        Some(u) => HttpResponse::Ok().json(u),
    }
}

#[get("/users")]
async fn get_users() -> impl Responder {
    HttpResponse::Ok().json(users_list())
}
