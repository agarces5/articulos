mod article;

use article::{
    domain::article_repository::ArticleRepository, infrastructure::mssql_article_repository::*,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let family_id = "0101".to_string();
    let _res = MssqlArticleRepository::calc_id(family_id).await;
    Ok(())
}
