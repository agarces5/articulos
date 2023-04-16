use crate::article::domain::{article::Article, article_repository::ArticleRepository};

use crate::article::application::dto::create_article_dto::CreateArticleInputDTO;

pub struct CreateArticleUseCase {
    repository: Box<dyn ArticleRepository>,
}

impl CreateArticleUseCase {
    pub fn new(repository: Box<dyn ArticleRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&mut self, input: CreateArticleInputDTO) -> anyhow::Result<i32> {
        let (input_name, input_family) = (&input.name(), &input.family_id());
        let id = self.repository.calc_new_id(input_family).await?;
        let user = "test";
        let article = Article::new(id, input_name, input_family, user);
        self.repository.save(&article).await
    }
}
