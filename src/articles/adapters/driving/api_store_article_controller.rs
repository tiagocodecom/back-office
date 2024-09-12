use crate::articles::application::ports::driving::StoreArticleService;
use crate::articles::application::use_cases::StoreArticleUseCase;
use crate::articles::entities::{NewArticle, NewArticleBuilder, RenderOutput};
use crate::framework::container::Container;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct NewArticleRequest {
    #[validate(length(min = 10))]
    pub slug: String,
    #[validate(length(min = 10))]
    pub title: String,
    #[validate(length(min = 50))]
    pub summary: String,
    #[validate(length(min = 500))]
    pub content: String,
    pub author_id: Uuid,
}

impl From<NewArticleRequest> for NewArticle {
    fn from(value: NewArticleRequest) -> Self {
        NewArticleBuilder::default()
            .slug(value.slug)
            .title(value.title)
            .summary(value.summary)
            .content(value.content)
            .author_id(value.author_id)
            .thumbnail_uri("/static/images/articles/thumbnail.jpg".to_string())
            .build()
            .unwrap()
    }
}

#[tracing::instrument(
    name = "ApiStoreArticleController",
    skip(sc, request),
    fields(
        title = %request.title,
        author_id = %request.author_id
    )
)]
pub async fn handle(sc: Data<Container<'_>>, request: Json<NewArticleRequest>) -> HttpResponse {
    match request.validate() {
        Ok(_) => (),
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    }

    let repository = &sc.postgres_article_repository;
    let presenter = &sc.json_get_article_presenter;
    let result = StoreArticleUseCase::new(repository, presenter)
        .execute(&request.into_inner().into())
        .await
        .unwrap();

    match result {
        RenderOutput::Json(json) => HttpResponse::Ok().json(json),
        _ => HttpResponse::InternalServerError().finish(),
    }
}
