use std::sync::Arc;

use anyhow::Error;
use redis_starter_rust::{
    config::Config, connection::Connection, db::KeyValueDb, server::RedisServer,
};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Logs from your program will appear here!");

    let config = Config::parse();
    let db = Arc::new(Mutex::new(KeyValueDb::default()));
    let server = Arc::new(RedisServer::new(config, db));

    let listener = server.listen().await?;

    while let Ok((stream, _)) = listener.accept().await {
        let conn = Connection::new(stream);
        let server = Arc::clone(&server);

        tokio::spawn(async move {
            if let Err(err) = server.handle_connection(conn).await {
                eprintln!("Failed to handle connection: {err}");
            }
        });
    }

    Ok(())
}
