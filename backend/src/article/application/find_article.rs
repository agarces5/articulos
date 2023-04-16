use crate::article::{
    application::dto::{article_id_dto::ArticleIdInputDTO, article_name_dto::ArticleNameInputDTO},
    domain::{article::Article, article_repository::ArticleRepository},
};

pub struct FindArticleUseCase {
    repository: Box<dyn ArticleRepository>,
}

impl FindArticleUseCase {
    pub fn new(repository: Box<dyn ArticleRepository>) -> Self {
        Self { repository }
    }

    pub async fn find_by_id(
        &mut self,
        input: ArticleIdInputDTO,
    ) -> anyhow::Result<Option<Article>> {
        let id = input.id();
        self.repository.find_by_id(id).await
    }
    pub async fn find_by_subname(
        &mut self,
        input: ArticleNameInputDTO,
    ) -> anyhow::Result<Vec<Article>> {
        self.repository.find_by_subname(input.subname()).await
    }
}

#[cfg(test)]
mod test {
    use crate::article::{
        application::{
            dto::{article_id_dto::ArticleIdInputDTO, article_name_dto::ArticleNameInputDTO},
            find_article::FindArticleUseCase,
        },
        domain::{article::Article, article_repository::MockArticleRepository},
    };
    use mockall::predicate::{eq, ne};

    #[tokio::test]
    async fn find_article_by_id() {
        let mock_article = Article::new(100001, "PRUEBA", "0001", "test");
        // MOCK REPO
        let mut repo = MockArticleRepository::new();
        let article = mock_article.clone();
        repo.expect_find_by_id()
            .with(eq(100001))
            .returning(move |_| anyhow::Ok(Some(article.clone())));
        repo.expect_find_by_id()
            .with(ne(100001))
            .returning(move |_| anyhow::Ok(None));
        // USE CASE
        let mut find_article_use_case = FindArticleUseCase::new(Box::new(repo));
        // DTO
        let input_happy_path = ArticleIdInputDTO::new(100001);
        let input_not_matching = ArticleIdInputDTO::new(0);

        assert_eq!(
            find_article_use_case
                .find_by_id(input_happy_path)
                .await
                .unwrap()
                .unwrap(),
            mock_article.clone()
        );
        assert!(find_article_use_case
            .find_by_id(input_not_matching)
            .await
            .unwrap()
            .is_none());
    }
    #[tokio::test]
    async fn find_article_by_name() {
        // MOCK ARTICLES
        let mock_articles = vec![
            Article::new(100001, "PRUEBA 1", "0001", "test"),
            Article::new(100002, "PRUEBA 2", "0001", "test"),
            Article::new(100003, "PRUEBA 3", "0001", "test"),
            Article::new(100004, "PRUEBA 4", "0001", "test"),
        ];
        // MOCK REPO
        let mut repo = MockArticleRepository::new();
        let articles = mock_articles.clone();
        repo.expect_find_by_subname()
            .with(eq("PRUEBA"))
            .returning(move |_| anyhow::Ok(articles.clone()));
        repo.expect_find_by_subname()
            .with(ne("PRUEBA"))
            .returning(move |_| anyhow::Ok(vec![]));
        // USE CASE
        let mut find_article_use_case = FindArticleUseCase::new(Box::new(repo));
        // DTO
        let input_happy_path = ArticleNameInputDTO::new("PRUEBA");
        let input_empty = ArticleNameInputDTO::new("");
        let input_not_existing_subname = ArticleNameInputDTO::new("NotExistingValue");
        assert_eq!(
            find_article_use_case
                .find_by_subname(input_happy_path)
                .await
                .unwrap(),
            mock_articles.clone()
        );
        assert_eq!(
            find_article_use_case
                .find_by_subname(input_empty)
                .await
                .unwrap(),
            vec![]
        );
        assert_eq!(
            find_article_use_case
                .find_by_subname(input_not_existing_subname)
                .await
                .unwrap(),
            vec![]
        );
    }
}
