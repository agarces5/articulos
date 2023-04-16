use anyhow::Result;

use crate::article::domain::article_repository::ArticleRepository;

use crate::article::application::dto::update_article_dto::UpdateArticleInputDTO;

pub struct UpdateArticleUseCase {
    repository: Box<dyn ArticleRepository>,
}

impl UpdateArticleUseCase {
    pub fn new(repository: Box<dyn ArticleRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&mut self, input: UpdateArticleInputDTO) -> Result<()> {
        self.repository
            .update(input.id(), input.name(), input.family_id())
            .await?;
        Ok(())
    }
}
