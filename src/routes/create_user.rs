use serde::Serialize;

use actix_web::{http::StatusCode, post, web::Json, Responder};

use crate::routes::{User, logging};

#[derive(Serialize)]
struct CreateUserResponse {
    id: u32,
    user: User,
}

#[post("/user/create")]
pub async fn create_new_user(user: Json<User>) -> impl Responder {
    logging("POST: /user/create");

    let response = CreateUserResponse {
        id: 1,
        user: user.0,
    };

    (Json(response), StatusCode::CREATED)
}
