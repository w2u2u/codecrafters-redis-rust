use std::sync::Arc;

use anyhow::Error;
use tokio::{net::TcpListener, sync::Mutex};

use crate::{cmd::Command, config::Config, connection::Connection, db::Database};

pub struct RedisServer<D>
where
    D: Database,
{
    config: Config,
    db: Arc<Mutex<D>>,
}

impl<D> RedisServer<D>
where
    D: Database,
{
    pub fn new(config: Config, db: Arc<Mutex<D>>) -> Self {
        RedisServer { config, db }
    }

    pub async fn listen(&self) -> Result<TcpListener, Error> {
        let address = format!("127.0.0.1:{}", self.config.port);
        let listener = TcpListener::bind(address).await?;

        Ok(listener)
    }

    pub async fn handle_connection(&self, mut conn: Connection) -> Result<(), Error> {
        loop {
            let Ok(frame) = conn.read_frame().await else {
                break Err(Error::msg("Unable to read frame"));
            };

            println!("Frame: {frame:?}");

            let cmd = Command::parse(&frame);

            println!("Command: {cmd:?}");

            match cmd {
                Command::Ping(ping) => {
                    ping.apply(&mut conn).await?;
                }
                Command::Echo(echo) => {
                    echo.apply(&mut conn).await?;
                }
                Command::Get(get) => {
                    let db = Arc::clone(&self.db);
                    get.apply(&mut conn, db).await?;
                }
                Command::Set(set) => {
                    let db = Arc::clone(&self.db);
                    set.apply(&mut conn, db).await?;
                }
                Command::Unknown => {}
            }
        }
    }
}
