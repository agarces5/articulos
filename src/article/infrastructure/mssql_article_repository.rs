use futures::prelude::*;

use async_trait::async_trait;
use bigdecimal::BigDecimal;
use tiberius::Query;

use crate::article::{
    domain::{article::Article, article_repository::ArticleRepository},
    infrastructure::{create_config::set_tcp_client, utils::get_row_value},
};

pub struct MssqlArticleRepository;

impl MssqlArticleRepository {
    async fn get_all() -> anyhow::Result<()> {
        let mut client = set_tcp_client().await?;

        let query = Query::new("SELECT * FROM [WTPV_CALEIA_TEST].[dbo].[TP_ARTICULO]");

        let stream = query.query(&mut client).await?.into_results().await?;

        stream.iter().for_each(|vec| {
            vec.iter().for_each(|row| {
                let id = get_row_value::<BigDecimal>(row, "Articulo");
                let name = get_row_value::<&str>(row, "Nombre");
                let family_id = get_row_value::<&str>(row, "Familia");
                let art = Article::new(id, name, family_id);
                println!("{}", art.to_string());
            })
        });
        Ok(())
    }
}

#[async_trait]
impl ArticleRepository for MssqlArticleRepository {
    async fn save(id: String, nombre: String, familia: String) {
        todo!()
    }

    async fn calc_id(family_id: String) -> anyhow::Result<String> {
        let mut client = set_tcp_client().await?;

        let mut query = Query::new("SELECT TOP(1) * FROM [WTPV_CALEIA_TEST].[dbo].[TP_ARTICULO] WHERE Familia = @P1 ORDER BY Articulo DESC");
        query.bind(&family_id);

        // let stream = query.query(&mut client).await?;

        // let row: String = stream
        //     .iter()
        //     .map(|vec| {
        //         vec.iter()
        //             .map(|row| {
        //                 let id = get_row_value::<BigDecimal>(row, "Articulo");
        //                 println!("{}", &id.to_string());
        //                 id
        //             })
        //             .collect::<String>()
        //     })
        //     .collect();
        // future::ok(res).await
        let stream = query.query(&mut client).await?;

        let row = stream.into_row().await?;
        let id = get_row_value::<BigDecimal>(&row.unwrap(), "Articulo");
        // let res = res.unwrap().to_string();
        println!("{}", &id);
        let res = future::ok(id).await;
        res
        // Ok(res)
    }
}
