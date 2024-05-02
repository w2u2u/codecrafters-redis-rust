use anyhow::Error;
use tokio::net::TcpListener;

use crate::{cmd::Command, config::Config, connection::Connection};

pub struct RedisServer {
    config: Config,
}

impl RedisServer {
    pub fn new(config: Config) -> Self {
        RedisServer { config }
    }

    pub async fn listen(&self) -> Result<TcpListener, Error> {
        let address = format!("127.0.0.1:{}", self.config.port);
        let listener = TcpListener::bind(address).await?;

        Ok(listener)
    }

    pub async fn handle_connection(&self, mut conn: Connection) -> Result<(), Error> {
        loop {
            let frame = conn.read_frame().await?;

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
                    get.apply().await?;
                }
                Command::Set(set) => {
                    set.apply().await?;
                }
                Command::Unknown => {}
            }
        }
    }
}
