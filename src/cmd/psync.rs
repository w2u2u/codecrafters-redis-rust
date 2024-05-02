use anyhow::Error;

use crate::{connection::Connection, frame::Frame, server::Replication};

#[derive(Debug)]
pub(crate) struct Psync {
    args: Vec<String>,
}

impl Psync {
    pub(crate) fn new(args: Vec<String>) -> Self {
        Psync { args }
    }

    pub(crate) async fn apply(
        &self,
        conn: &mut Connection,
        repl: &Replication,
    ) -> Result<(), Error> {
        match (self.args.get(1), self.args.get(2)) {
            (Some(a), Some(b)) if a == "?" && b == "-1" => {
                let frame = Frame::SimpleString(format!("FULLRESYNC {} 0", repl.master_replid));

                conn.write_frame(&frame).await?;
            }
            _ => {}
        }

        Ok(())
    }

    pub(crate) async fn send(&self, conn: &mut Connection) -> Result<(), Error> {
        let frame = Frame::Arrays([vec![String::from("PSYNC")], self.args.clone()].concat());

        conn.write_frame(&frame).await?;

        Ok(())
    }
}
