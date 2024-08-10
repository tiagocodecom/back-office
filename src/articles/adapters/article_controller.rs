use crate::articles::application::services::get_articles_service;
use crate::container::Container;
use actix_web::{web, HttpResponse};

pub async fn get_articles(container: web::Data<Container>) -> impl actix_web::Responder {
    let results = get_articles_service::execute(&container.articles_repository).await;

    match results {
        Ok(_articles) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
