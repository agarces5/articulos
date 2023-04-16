use std::sync::Arc;

use async_trait::async_trait;
use tiberius::{numeric::Numeric, Client};
use tokio::sync::Mutex;

use crate::article::domain::{article::Article, article_repository::ArticleRepository};

use crate::article::infrastructure::db::tiberius::tiberius_article::TiberiusArticle;

type SqlClient = Client<tokio_util::compat::Compat<tokio::net::TcpStream>>;

#[derive(Clone)]
pub struct MssqlArticleRepository {
    client: Arc<Mutex<SqlClient>>,
}

impl MssqlArticleRepository {
    pub fn new(client: Arc<Mutex<SqlClient>>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl ArticleRepository for MssqlArticleRepository {
    async fn save(&self, article: &Article) -> anyhow::Result<i32> {
        let tiberius_article = TiberiusArticle::from(article);
        let query = "INSERT [dbo].[TP_ARTICULO] (Articulo, Nombre, Familia, Usuario) VALUES (@P1, @P2, @P3, @P4)";
        // let client = Arc::clone(&self.client);
        // Execute the query throught &self
        self.client
            .lock()
            .await
            .execute(
                query,
                &[
                    &tiberius_article.id(),
                    &tiberius_article.name(),
                    &tiberius_article.family_id(),
                    &tiberius_article.user(),
                ],
            )
            .await?;
        Ok(article.id())
    }
    async fn calc_new_id(&self, family_id: &str) -> anyhow::Result<i32> {
        let query = "SELECT TOP(1) Articulo FROM [WTPV_CALEIA_TEST].[dbo].[TP_ARTICULO] WHERE Familia=@P1 ORDER BY Articulo DESC";
        let mut client = self.client.lock().await;
        let row = client.query(query, &[&family_id]).await?;
        let row = row.into_row().await?;
        let res = match row {
            Some(row) => {
                (row.try_get::<Numeric, &str>("Articulo")?
                    .unwrap()
                    .int_part()
                    + 1) as i32
            }
            None => format!("{family_id}00001")
                .parse()
                .expect("Failed to convert Article id"),
        };
        Ok(res)
    }
    async fn get_all_by_family(&self, family_id: &str) -> anyhow::Result<Vec<Article>> {
        let mut articles = vec![];
        let query = "SELECT * FROM [WTPV_CALEIA_TEST].[dbo].[TP_ARTICULO] WHERE Familia=@P1";
        let mut client = self.client.lock().await;
        let rows = client.query(query, &[&family_id]).await?;

        for row in rows.into_first_result().await? {
            let tiberius_article = TiberiusArticle::new(
                row.get("Articulo")
                    .expect("Fallo al recibir la columna 'Articulo'"),
                row.get("Nombre")
                    .expect("Fallo al recibir la columna 'Nombre'"),
                row.get("Familia")
                    .expect("Fallo al recibir la columna 'Familia'"),
                row.get("Usuario")
                    .expect("Fallo al recibir la columna 'Usuario'"),
            );
            articles.push(tiberius_article.into());
        }

        Ok(articles)
    }
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<Article>> {
        let query = "SELECT * FROM [WTPV_CALEIA_TEST].[dbo].[TP_ARTICULO] WHERE Articulo=@P1";
        let mut client = self.client.lock().await;
        let row = client.query(query, &[&id]).await?;

        let find = row.into_row().await?;
        let res = match find {
            None => None,
            Some(res) => {
                let tiberius_article = TiberiusArticle::new(
                    res.get("Articulo")
                        .expect("Fallo al recibir la columna 'Articulo'"),
                    res.get("Nombre")
                        .expect("Fallo al recibir la columna 'Nombre'"),
                    res.get("Familia")
                        .expect("Fallo al recibir la columna 'Familia'"),
                    res.get("Usuario")
                        .expect("Fallo al recibir la columna 'Usuario'"),
                );
                let article = tiberius_article.into();
                Some(article)
            }
        };
        Ok(res)
    }
    async fn find_by_subname(&self, name: &str) -> anyhow::Result<Vec<Article>> {
        let mut articles = vec![];
        let query = "SELECT * FROM [WTPV_CALEIA_TEST].[dbo].[TP_ARTICULO] WHERE Nombre LIKE @P1";
        let name = format!("%{name}%");
        let mut client = self.client.lock().await;
        let rows = client.query(query, &[&name]).await?;

        for row in rows.into_first_result().await? {
            let tiberius_article = TiberiusArticle::new(
                row.get("Articulo")
                    .expect("Fallo al recibir la columna 'Articulo'"),
                row.get("Nombre")
                    .expect("Fallo al recibir la columna 'Nombre'"),
                row.get("Familia")
                    .expect("Fallo al recibir la columna 'Familia'"),
                row.get("Usuario")
                    .expect("Fallo al recibir la columna 'Usuario'"),
            );
            let article: Article = tiberius_article.into();
            articles.push(article);
        }

        Ok(articles)
    }
    async fn update(
        &self,
        id: i32,
        new_name: Option<String>,
        new_family: Option<String>,
    ) -> anyhow::Result<()> {
        if (new_name.clone(), new_family.clone()) == (None, None) {
            return Ok(());
        }
        if self.find_by_id(id).await?.is_none() {
            return Err(anyhow::Error::msg(format!(
                "Articulo {id} no encontrado en la base de datos"
            )));
        };
        let tiberius_article = match (new_name, new_family) {
            (None, None) => TiberiusArticle::from(&Article::new(-1, "Failure", "0001", "test")), // Never matching
            (None, Some(new_family)) => {
                let old_article = self.find_by_id(id).await?.unwrap();
                let mut new_article = TiberiusArticle::from(&old_article);
                new_article.set_family_id(&new_family);
                new_article
            }
            (Some(new_name), None) => {
                let old_article = self.find_by_id(id).await?.unwrap();
                let mut new_article = TiberiusArticle::from(&old_article);
                new_article.set_name(&new_name);
                new_article
            }
            (Some(new_name), Some(new_family)) => {
                let new_article = Article::new(id, &new_name, &new_family, "test");
                TiberiusArticle::from(&new_article)
            }
        };
        let query = "UPDATE [WTPV_CALEIA_TEST].[dbo].[TP_ARTICULO]
                            SET Nombre = @P2,
                               Familia = @P3
                            WHERE Articulo = @P1";
        self.client
            .lock()
            .await
            .execute(
                query,
                &[
                    &tiberius_article.id(),
                    &tiberius_article.name(),
                    &tiberius_article.family_id(),
                ],
            )
            .await?;
        Ok(())
    }

    async fn remove(&self, id: i32) -> anyhow::Result<()> {
        let query = "DELETE FROM [WTPV_CALEIA_TEST].[dbo].[TP_ARTICULO_CAJA] WHERE Articulo=@P1;
        DELETE FROM [WTPV_CALEIA_TEST].[dbo].[TP_PRECIOS] WHERE Articulo=@P1;
        DELETE FROM [WTPV_CALEIA_TEST].[dbo].[TP_ARTICULO] WHERE Articulo=@P1";

        // Execute the query throught &self
        self.client.lock().await.execute(query, &[&id]).await?;
        Ok(())
    }
}
