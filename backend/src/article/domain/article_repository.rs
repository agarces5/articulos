use async_trait::async_trait;
use mockall::predicate::*;
use mockall::*;

use crate::article::domain::article::Article;

#[automock]
#[async_trait]
pub trait ArticleRepository {
    async fn save(&self, article: &Article) -> anyhow::Result<i32>;
    async fn calc_new_id(&self, family_id: &str) -> anyhow::Result<i32>;
    async fn get_all_by_family(&self, family_id: &str) -> anyhow::Result<Vec<Article>>;
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<Article>>;
    async fn find_by_subname(&self, name: &str) -> anyhow::Result<Vec<Article>>;
    async fn update(
        &self,
        id: i32,
        new_name: Option<String>,
        new_family: Option<String>,
    ) -> anyhow::Result<()>;
    async fn remove(&self, id: i32) -> anyhow::Result<()>;
}
