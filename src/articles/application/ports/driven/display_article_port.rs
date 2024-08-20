use crate::articles::application::domain::Article;

pub trait DisplayArticlePort {
    fn render(&self, article: Article) -> String;
}
