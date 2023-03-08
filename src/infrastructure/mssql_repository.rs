use bigdecimal::BigDecimal;
use tiberius::Query;

use crate::domain::articulo::articulo::Articulo;
use crate::infrastructure::utils::get_row_value;

use crate::infrastructure::create_config::set_tcp_client;

pub async fn get_all() -> anyhow::Result<()> {
    let mut client = set_tcp_client().await?;

    let query = Query::new("SELECT * FROM [WTPV_CALEIA_TEST].[dbo].[TP_ARTICULO]");

    let stream = query.query(&mut client).await?.into_results().await?;

    stream.iter().for_each(|vec| {
        vec.iter().for_each(|row| {
            let id = get_row_value::<BigDecimal>(row, "Articulo");
            let nombre = get_row_value::<&str>(row, "Nombre");
            let familia = get_row_value::<&str>(row, "Familia");
            let art = Articulo::new(id, nombre, familia);
            println!("{}", art.to_string());
        })
    });
    Ok(())
}
