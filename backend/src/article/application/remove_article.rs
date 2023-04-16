use anyhow::Result;

use crate::article::{
    application::dto::article_id_dto::ArticleIdInputDTO,
    domain::article_repository::ArticleRepository,
};

pub struct RemoveArticleUseCase {
    repository: Box<dyn ArticleRepository>,
}

impl RemoveArticleUseCase {
    pub fn new(repository: Box<dyn ArticleRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&mut self, input: ArticleIdInputDTO) -> Result<()> {
        self.repository.remove(input.id()).await?;
        Ok(())
    }
}
