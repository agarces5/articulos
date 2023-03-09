use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

pub fn create_config() -> Config {
    let mut config = Config::new();

    config.host("localhost");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("SA", "Sqlserver-2017"));
    config.trust_cert();

    config
}

pub async fn set_tcp_client(
) -> anyhow::Result<Client<tokio_util::compat::Compat<tokio::net::TcpStream>>> {
    let config = create_config();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let client = Client::connect(config, tcp.compat_write()).await?;

    Ok(client)
}
