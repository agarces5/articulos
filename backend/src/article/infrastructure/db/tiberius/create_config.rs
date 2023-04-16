use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

fn create_config(user: impl ToString, pass: impl ToString, database: impl ToString) -> Config {
    let mut config = Config::new();

    config.host("localhost");
    config.port(1433);
    config.authentication(AuthMethod::sql_server(user, pass));
    config.trust_cert();
    config.database(database);

    config
}

pub async fn set_tcp_client(
    database: impl ToString,
    user: impl ToString,
    pass: impl ToString,
) -> anyhow::Result<Client<Compat<TcpStream>>> {
    let config = create_config(user, pass, database);

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let client = Client::connect(config, tcp.compat_write()).await?;

    Ok(client)
}
