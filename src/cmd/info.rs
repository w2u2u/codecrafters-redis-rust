use anyhow::Error;

use crate::{config::Config, connection::Connection, frame::Frame, server::Replication};

#[derive(Debug)]
pub(crate) struct Info;

impl Info {
    pub(crate) fn new() -> Self {
        Info
    }

    pub(crate) async fn apply(
        &self,
        conn: &mut Connection,
        _config: &Config,
        repl: &Replication,
    ) -> Result<(), Error> {
        let frame = Frame::BulkString(format!("role:{}", repl.role));

        conn.write_frame(&frame).await?;

        Ok(())
    }
}
