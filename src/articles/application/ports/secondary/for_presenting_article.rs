use crate::articles::domain::Article;

pub trait ForPresentingArticle {
    fn render(&self, article: Article) -> String;
}
