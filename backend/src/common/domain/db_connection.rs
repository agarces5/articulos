use async_trait::async_trait;
use mockall::automock;
use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::Compat;

#[automock]
#[async_trait]
pub trait DBConnection {
    async fn set_tcp_client(
        &self,
        user: &str,
        pass: &str,
        database: &str,
    ) -> anyhow::Result<Client<Compat<TcpStream>>>;
}
