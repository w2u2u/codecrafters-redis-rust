use std::sync::Arc;

use anyhow::Error;
use tokio::{net::TcpListener, sync::Mutex};

use crate::{cmd::Command, config::Config, connection::Connection, db::Database};

pub(crate) enum Role {
    Master,
    Slave,
}

#[allow(dead_code)]
pub(crate) struct Replication {
    role: Role,
    connected_slaves: u16,
    master_replid: String,
    master_repl_offset: i8,
    second_repl_offset: i8,
}

impl Replication {
    fn new(config: &Config) -> Self {
        Replication {
            role: if config.replicaof.is_some() {
                Role::Slave
            } else {
                Role::Master
            },
            connected_slaves: 0,
            master_replid: String::from("8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb"),
            master_repl_offset: -1,
            second_repl_offset: 0,
        }
    }
}

pub struct RedisServer<D>
where
    D: Database,
{
    replication: Replication,
    config: Config,
    db: Arc<Mutex<D>>,
}

impl<D> RedisServer<D>
where
    D: Database,
{
    pub fn new(config: Config, db: Arc<Mutex<D>>) -> Self {
        RedisServer {
            replication: Replication::new(&config),
            config,
            db,
        }
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
                Command::Info(info) => {
                    info.apply(&mut conn, &self.config, &self.replication)
                        .await?;
                }
                Command::Unknown => {}
            }
        }
    }
}
