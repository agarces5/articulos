use std::sync::Arc;

use articulos::article::application::create_article::CreateArticleUseCase;
use articulos::article::application::dto::article_id_dto::ArticleIdInputDTO;
use articulos::article::application::dto::article_name_dto::ArticleNameInputDTO;
use articulos::article::application::dto::create_article_dto::CreateArticleInputDTO;
use articulos::article::application::dto::update_article_dto::UpdateArticleInputDTO;
use articulos::article::application::find_article::FindArticleUseCase;
use articulos::article::application::remove_article::RemoveArticleUseCase;
use articulos::article::application::update_article::UpdateArticleUseCase;
use articulos::article::domain::{article::Article, article_repository::ArticleRepository};
use articulos::article::infrastructure::db::tiberius::mssql_article_repository::MssqlArticleRepository;
use articulos::common::infrastructure::db::tiberius::create_config::set_tcp_client;
use tokio::sync::Mutex;

const DATABASE: &str = "WTPV_CALEIA_TEST";
const USER: &str = "SA";
const PASSWORD: &str = "Sqlserver-2017";

async fn create_repo() -> anyhow::Result<MssqlArticleRepository> {
    let client = set_tcp_client(USER, PASSWORD, DATABASE).await.unwrap();
    let repo = MssqlArticleRepository::new(Arc::new(Mutex::new(client)));

    Ok(repo)
}

#[tokio::test]
async fn database_connection() {
    // Creamos una conexión a una base de datos de prueba
    let mut client = set_tcp_client(USER, PASSWORD, DATABASE).await.unwrap();
    let result = client
        .query(
            "SELECT DISTINCT 1
            FROM [dbo].[TP_ARTICULO]",
            &[],
        )
        .await
        .unwrap();
    let count: i32 = result.into_row().await.unwrap().unwrap().get(0).unwrap();
    assert_eq!(count, 1);
}
#[tokio::test]
async fn test_save_article() {
    // REPO
    let repo = Box::new(create_repo().await.unwrap());
    // USE CASE
    let mut create_article_use_case = CreateArticleUseCase::new(repo);
    let input = CreateArticleInputDTO::new("ARTICULO DE PRUEBA", "0001");
    let mut result = create_article_use_case.execute(input).await;
    while result.is_err() {
        let input = CreateArticleInputDTO::new("ARTICULO DE PRUEBA", "0001");
        result = create_article_use_case.execute(input).await;
    }
    let result = result.unwrap();
    // ASSERTS
    assert!(result.is_positive());

    // DESTROY!
    RemoveArticleUseCase::new(Box::new(create_repo().await.unwrap()))
        .execute(ArticleIdInputDTO::new(result))
        .await
        .unwrap();
}
#[tokio::test]
async fn test_calc_id() {
    let repo = create_repo().await.unwrap();

    let new_article = Article::new(
        repo.calc_new_id("0101").await.unwrap(),
        "PRUEBA DE CALC ID",
        "0101",
        "test",
    );

    let id = repo.save(&new_article).await.unwrap_or_else(|e| {
        println!("{e}");
        -1
    });

    let new_id: i32 = repo.calc_new_id(new_article.family_id()).await.unwrap();

    repo.remove(new_article.id())
        .await
        .unwrap_or_else(|e| eprintln!("{e}"));
    assert_eq!(id + 1, new_id);
}
#[tokio::test]
async fn test_find_article_by_id() {
    // PREPARATION
    let article = Article::new(201, "PRUEBA DE FIND BY ID", "0001", "test");
    let repo = Box::new(create_repo().await.unwrap());
    let mut create = repo.save(&article).await;
    while create.is_err() {
        create = repo.save(&article).await;
    }
    // INPUT
    let input = ArticleIdInputDTO::new(201);
    // REPO
    let repo = Box::new(create_repo().await.unwrap());
    // USE CASE
    let mut find_article_by_id_use_case = FindArticleUseCase::new(repo);
    let result = find_article_by_id_use_case.find_by_id(input).await;
    // ASSERTS
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap().unwrap(),
        Article::new(201, "PRUEBA DE FIND BY ID", "0001", "test")
    );

    // DESTROY!
    RemoveArticleUseCase::new(Box::new(create_repo().await.unwrap()))
        .execute(ArticleIdInputDTO::new(201))
        .await
        .unwrap();
}
#[tokio::test]
async fn test_remove_article() {
    let repo = Box::new(create_repo().await.unwrap());
    let new_article = Article::new(200, "PRUEBA DE REMOVE", "0001", "test");

    let mut id = repo.save(&new_article).await;
    while id.is_err() {
        id = repo.save(&new_article).await;
    }
    let id = id.unwrap();
    // INPUT
    let input = ArticleIdInputDTO::new(200);
    // USE CASE
    let mut remove_use_case = RemoveArticleUseCase::new(repo);
    let res = remove_use_case.execute(input).await;

    let repo = Box::new(create_repo().await.unwrap());

    // ASSERTS
    assert!(res.is_ok());
    assert!(repo.find_by_id(id).await.unwrap().is_none());
}
#[tokio::test]
async fn test_get_all_by_family_id() {
    let repo = create_repo().await.unwrap();
    let articles = vec![
        Article::new(301, "Articulo 1", "0201", "test"),
        Article::new(302, "Articulo 2", "0201", "test"),
        Article::new(303, "Articulo 3", "0201", "test"),
        Article::new(304, "Articulo 4", "0201", "test"),
        Article::new(305, "Articulo 5", "0201", "test"),
    ];
    for article in &articles {
        let mut id = repo.save(article).await;
        while id.is_err() {
            id = repo.save(article).await;
        }
    }

    let result = repo.get_all_by_family("0201").await.unwrap();

    assert_eq!(&result, &articles);
    RemoveArticleUseCase::new(Box::new(create_repo().await.unwrap()))
        .execute(ArticleIdInputDTO::new(301))
        .await
        .unwrap();
    RemoveArticleUseCase::new(Box::new(create_repo().await.unwrap()))
        .execute(ArticleIdInputDTO::new(302))
        .await
        .unwrap();
    RemoveArticleUseCase::new(Box::new(create_repo().await.unwrap()))
        .execute(ArticleIdInputDTO::new(303))
        .await
        .unwrap();
    RemoveArticleUseCase::new(Box::new(create_repo().await.unwrap()))
        .execute(ArticleIdInputDTO::new(304))
        .await
        .unwrap();
    RemoveArticleUseCase::new(Box::new(create_repo().await.unwrap()))
        .execute(ArticleIdInputDTO::new(305))
        .await
        .unwrap();
}
#[tokio::test]
async fn test_find_article_by_subname() {
    // FAKE DATA
    let repo = create_repo().await.unwrap();
    let articles = vec![
        Article::new(401, "FindArticleBySubname", "0001", "test"),
        Article::new(402, "FindArticleBySubname", "0001", "test"),
        Article::new(403, "FindArticleBySubname", "0001", "test"),
        Article::new(404, "FindArticleBySubname", "0001", "test"),
        Article::new(405, "FindArticleBySubname", "0001", "test"),
    ];
    for article in &articles {
        let mut id = repo.save(article).await;
        while id.is_err() {
            id = repo.save(article).await;
        }
    }

    // INPUTS
    let input_happy_path = ArticleNameInputDTO::new("bysub");
    let input_sqlinyection_or = ArticleNameInputDTO::new("whatever OR 1=1");
    let input_sqlinyection_or_equal = ArticleNameInputDTO::new(r#"" or ""=""#);
    let input_sqlinyection_statement = ArticleNameInputDTO::new("100001; DROP TABLE TP_ARTICULOS");
    // USE CASE
    let mut find_use_case = FindArticleUseCase::new(Box::new(repo));

    let case_1 = find_use_case.find_by_subname(input_happy_path).await;
    let articles_found = case_1.unwrap();
    let case_2 = find_use_case.find_by_subname(input_sqlinyection_or).await;
    let case_3 = find_use_case
        .find_by_subname(input_sqlinyection_or_equal)
        .await;
    let case_4 = find_use_case
        .find_by_subname(input_sqlinyection_statement)
        .await;

    // DESTROY!
    RemoveArticleUseCase::new(Box::new(create_repo().await.unwrap()))
        .execute(ArticleIdInputDTO::new(401))
        .await
        .unwrap();
    RemoveArticleUseCase::new(Box::new(create_repo().await.unwrap()))
        .execute(ArticleIdInputDTO::new(402))
        .await
        .unwrap();
    RemoveArticleUseCase::new(Box::new(create_repo().await.unwrap()))
        .execute(ArticleIdInputDTO::new(403))
        .await
        .unwrap();
    RemoveArticleUseCase::new(Box::new(create_repo().await.unwrap()))
        .execute(ArticleIdInputDTO::new(404))
        .await
        .unwrap();
    RemoveArticleUseCase::new(Box::new(create_repo().await.unwrap()))
        .execute(ArticleIdInputDTO::new(405))
        .await
        .unwrap();
    assert_eq!(&articles_found, &articles);
    assert!(case_2.unwrap().is_empty());
    assert!(case_3.unwrap().is_empty());
    assert!(case_4.unwrap().is_empty());
}
#[tokio::test]
async fn test_update_article() {
    // INPUT
    let inputs = vec![
        UpdateArticleInputDTO::new(501, Some("ARTICULO 1 UPDATED".to_string()), None),
        UpdateArticleInputDTO::new(502, None, Some("0001".to_string())),
        UpdateArticleInputDTO::new(
            503,
            Some("ARTICULO 3 UPDATED".to_string()),
            Some("0001".to_string()),
        ),
    ];
    // REPO
    let repo = Box::new(create_repo().await.unwrap());
    // PREPARE ARTICLES
    let articles = vec![
        Article::new(501, "ARTICULO 1", "0001", "test"),
        Article::new(502, "ARTICULO 2", "0001", "test"),
        Article::new(503, "ARTICULO 3", "0001", "test"),
    ];
    // SAVE
    for article in &articles {
        let mut id = repo.save(article).await;
        while id.is_err() {
            id = repo.save(article).await;
        }
    }

    //UPDATE USE CASE
    let mut update_article_use_case = UpdateArticleUseCase::new(repo);
    for input in inputs {
        update_article_use_case
            .execute(input)
            .await
            .unwrap_or_else(|e| println!("{e}"));
    }

    let repo = Box::new(create_repo().await.unwrap());
    assert_eq!(
        Some(Article::new(501, "ARTICULO 1 UPDATED", "0001", "test")),
        repo.find_by_id(501).await.unwrap()
    );
    assert_eq!(
        Some(Article::new(502, "ARTICULO 2", "0001", "test")),
        repo.find_by_id(502).await.unwrap()
    );
    assert_eq!(
        Some(Article::new(503, "ARTICULO 3 UPDATED", "0001", "test")),
        repo.find_by_id(503).await.unwrap()
    );
    //DESTROY!
    RemoveArticleUseCase::new(Box::new(create_repo().await.unwrap()))
        .execute(ArticleIdInputDTO::new(501))
        .await
        .unwrap();
    RemoveArticleUseCase::new(Box::new(create_repo().await.unwrap()))
        .execute(ArticleIdInputDTO::new(502))
        .await
        .unwrap();
    RemoveArticleUseCase::new(Box::new(create_repo().await.unwrap()))
        .execute(ArticleIdInputDTO::new(503))
        .await
        .unwrap();
}
