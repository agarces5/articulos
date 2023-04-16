use backend::article::{
    application::{
        dto::{article_id_dto::ArticleIdInputDTO, update_article_dto::UpdateArticleInputDTO},
        remove_article::RemoveArticleUseCase,
        update_article::UpdateArticleUseCase,
    },
    domain::{article::Article, article_repository::MockArticleRepository},
};
use mockall::predicate::eq;

#[tokio::test]
async fn test_update_article_use_case() {
    // INPUT
    let input = UpdateArticleInputDTO::new(
        100001,
        Some("PRUEBA UPDATED".to_string()),
        Some("0001".to_string()),
    );
    // REPO
    let mut repo = MockArticleRepository::new();
    repo.expect_find_by_id()
        .with(eq(100001))
        .returning(move |_| Ok(Some(Article::new(100001, "PRUEBA", "0101", "test"))));
    repo.expect_update()
        .with(
            eq(100001),
            eq(Some("PRUEBA UPDATED".to_string())),
            eq(Some("0001".to_string())),
        )
        .returning(|_, _, _| Ok(()));

    // USE CASE
    let mut update_use_case = UpdateArticleUseCase::new(Box::new(repo));
    // ASSERTS
    assert!(update_use_case.execute(input).await.is_ok());
}
#[tokio::test]
async fn test_delete_article_use_case() {
    let mut repo = MockArticleRepository::new();
    repo.expect_remove()
        .with(eq(100001))
        .return_once(|_| anyhow::Ok(()));
    let mut remove_article_use_case = RemoveArticleUseCase::new(Box::new(repo));
    assert!(remove_article_use_case
        .execute(ArticleIdInputDTO::new(100001))
        .await
        .is_ok());
}
