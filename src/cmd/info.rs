use anyhow::Error;

use crate::{
    config::Config,
    connection::Connection,
    frame::Frame,
    replication::{Replication, Role},
};

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
        let s = match repl.role {
            Role::Master => {
                let info = [
                    format!("role:{}", repl.role),
                    format!("master_replid:{}", repl.master_replid),
                    format!("master_repl_offset:{}", repl.master_repl_offset),
                ];

                info.join("\n")
            }
            Role::Slave => format!("role:{}", repl.role),
        };

        let frame = Frame::BulkString(s);

        conn.write_frame(&frame).await?;

        Ok(())
    }
}
