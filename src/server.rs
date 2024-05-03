use std::sync::Arc;

use anyhow::Error;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use crate::{
    cmd::{ping::Ping, psync::Psync, replconf::Replconf, Command},
    config::Config,
    connection::Connection,
    db::Database,
    replication::Replication,
};

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
                Command::Replconf(replconf) => {
                    replconf.apply(&mut conn).await?;
                }
                Command::Psync(psync) => {
                    psync.apply(&mut conn, &self.replication).await?;
                }
                _ => {}
            }
        }
    }

    pub async fn connect_to_master(&self) -> Result<TcpStream, Error> {
        if let Some(replicaof) = self.config.replicaof.clone() {
            let stream = TcpStream::connect(replicaof).await?;
            return Ok(stream);
        }

        Err(Error::msg("no replica of"))
    }

    pub async fn handshake(&self, mut conn: Connection) -> Result<(), Error> {
        Ping::new(Some("ping")).send(&mut conn).await?;

        let _frame = conn.read_frame().await?;

        Replconf::new(vec![String::from("listening-port"), String::from("6380")])
            .send(&mut conn)
            .await?;

        let _frame = conn.read_frame().await?;

        Replconf::new(vec![String::from("capa"), String::from("psync2")])
            .send(&mut conn)
            .await?;

        let _frame = conn.read_frame().await?;

        Psync::new(vec![String::from("?"), String::from("-1")])
            .send(&mut conn)
            .await?;

        let _frame = conn.read_frame().await?;

        Ok(())
    }
}
