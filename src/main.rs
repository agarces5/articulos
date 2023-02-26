mod application;
mod domain;
mod infrastructure;

use anyhow::Ok;
use infrastructure::mssql_repository::get_all;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _res = get_all().await;
    Ok(())
}
