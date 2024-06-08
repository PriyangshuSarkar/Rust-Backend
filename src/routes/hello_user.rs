use actix_web::{get, web::Path,  HttpResponse,  Responder};
use serde::{Deserialize, Serialize};
use crate::routes::logging;

#[derive(Serialize, Deserialize)]
pub struct User {
    first_name: String,
    last_name: String,
}

impl User {
    fn new(first_name: String, last_name: String) -> Self {
        Self { first_name, last_name }
    }
}



#[get("/hello/{first_name}/{last_name}")]
pub async fn hello_user(params: Path<(String, String)>) -> impl Responder {
    let route=format!("GET: /hello/{}/{}", params.0.clone(), params.1.clone());
    logging(&route);
    let response = User::new(params.0.clone(), params.1.clone());
    HttpResponse::Ok().json(response)
}
