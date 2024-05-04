use std::sync::Arc;

use anyhow::Error;
use redis_starter_rust::{
    config::Config, connection::Connection, db::KeyValueDb, server::RedisServer,
};
use tokio::sync::{broadcast, Mutex};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Logs from your program will appear here!");

    let config = Config::parse();
    let db = Arc::new(Mutex::new(KeyValueDb::default()));
    let server = Arc::new(RedisServer::new(config, db));
    let (sender, _rx) = broadcast::channel(16);
    let sender = Arc::new(sender);

    if let Ok(stream) = server.connect_to_master().await {
        let conn = Connection::new(stream);

        server.handshake(conn).await?;
    }

    let listener = server.listen().await?;

    while let Ok((stream, _)) = listener.accept().await {
        let conn = Connection::new(stream);
        let server = Arc::clone(&server);
        let sender = Arc::clone(&sender);

        tokio::spawn(async move {
            if let Err(err) = server.handle_connection(conn, sender).await {
                eprintln!("Failed to handle connection: {err}");
            }
        });
    }

    Ok(())
}
