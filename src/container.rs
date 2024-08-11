use crate::articles::adapters::persistence::{ArticleRepository, AuthorRepository};

#[derive(Clone)]
pub struct Container {
    pub article_repository: ArticleRepository,
    pub author_repository: AuthorRepository,
}

impl Container {
    pub fn new() -> Self {
        Self {
            article_repository: ArticleRepository {},
            author_repository: AuthorRepository {},
        }
    }
}

pub fn get_container() -> Container {
    Container::new()
}
