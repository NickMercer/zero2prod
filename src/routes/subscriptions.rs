use actix_web::{web, HttpResponse};

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}


#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String
}