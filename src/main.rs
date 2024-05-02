use anyhow::Error;
use redis_starter_rust::{config::Config, connection::Connection, server::RedisServer};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Logs from your program will appear here!");

    let config = Config::parse();
    let server = RedisServer::new(config);

    let listener = server.listen().await?;

    while let Ok((stream, _)) = listener.accept().await {
        let conn = Connection::new(stream);

        server.handle_connection(conn).await?;
    }

    Ok(())
}
