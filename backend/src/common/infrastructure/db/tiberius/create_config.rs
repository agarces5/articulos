use async_trait::async_trait;
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use crate::common::domain::db_connection::DBConnection;

pub struct TiberiusClient;

impl TiberiusClient {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DBConnection for TiberiusClient {
    async fn set_tcp_client(
        &self,
        user: &str,
        pass: &str,
        database: &str,
    ) -> anyhow::Result<Client<Compat<TcpStream>>> {
        let config = create_config(user, pass, database);

        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;

        let client = Client::connect(config, tcp.compat_write()).await?;

        Ok(client)
    }
}

fn create_config(user: impl ToString, pass: impl ToString, database: impl ToString) -> Config {
    let mut config = Config::new();

    config.host("localhost");
    config.port(1433);
    config.authentication(AuthMethod::sql_server(user, pass));
    config.trust_cert();
    config.database(database);

    config
}

// pub async fn set_tcp_client(
//     user: impl ToString,
//     pass: impl ToString,
//     database: impl ToString,
// ) -> anyhow::Result<Client<Compat<TcpStream>>> {
//     let config = create_config(user, pass, database);

//     let tcp = TcpStream::connect(config.get_addr()).await?;
//     tcp.set_nodelay(true)?;

//     let client = Client::connect(config, tcp.compat_write()).await?;

//     Ok(client)
// }
