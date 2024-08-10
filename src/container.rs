use crate::articles::adapters::article_repository::ArticlesRepository;

#[derive(Clone, Debug)]
pub struct Container {
    pub articles_repository: ArticlesRepository,
}

impl Container {
    pub fn load() -> Self {
        Self {
            articles_repository: ArticlesRepository {},
        }
    }
}
