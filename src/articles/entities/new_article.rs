use derive_builder::Builder;
use derive_getters::Getters;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Getters, Builder)]
pub struct NewArticle {
    slug: String,
    title: String,
    summary: String,
    content: String,
    author_id: Uuid,
    thumbnail_uri: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_a_new_article_and_returns_the_correct_properties() {
        let slug = "lorem-ipsum-dolor-sit-amet".to_string();
        let title = "Lorem ipsum dolor sit amet".to_string();
        let summary = "In this article, we will talk about lorem ipsum.".to_string();
        let content = r#"
            Lorem Ipsum is simply dummy text of the printing and typesetting industry.
            Lorem Ipsum has been the industry's standard dummy text ever since the 1500s,
            when an unknown printer took a galley of type and scrambled it to make a type
        "#
        .to_string();
        let author_id = Uuid::new_v4();

        let new_article = NewArticleBuilder::default()
            .author_id(author_id)
            .slug(slug.clone())
            .summary(summary.clone())
            .title(title.clone())
            .content(content.clone())
            .thumbnail_uri("/static/images/articles/thumbnail.jpg".to_string())
            .build()
            .unwrap();

        assert_eq!(new_article.slug(), &slug);
        assert_eq!(new_article.title(), &title);
        assert_eq!(new_article.summary(), &summary);
        assert_eq!(new_article.content(), &content);
        assert_eq!(new_article.author_id(), &author_id);
    }
}
