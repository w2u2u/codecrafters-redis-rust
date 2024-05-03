use anyhow::Error;

use crate::{connection::Connection, frame::Frame, replication::Replication, util::hex};

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

                let frame = Frame::BulkBytes(hex::decode("524544495330303131fa0972656469732d76657205372e322e30fa0a72656469732d62697473c040fa056374696d65c26d08bc65fa08757365642d6d656dc2b0c41000fa08616f662d62617365c000fff06e3bfec0ff5aa2"));

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
