use anyhow::Error;

use crate::{connection::Connection, frame::Frame};

#[derive(Debug)]
pub(crate) struct Echo {
    msg: String,
}

impl Echo {
    pub(crate) fn new(args: Vec<String>) -> Self {
        Echo {
            msg: args.get(1).cloned().unwrap_or(String::new()),
        }
    }

    pub(crate) async fn apply(&self, conn: &mut Connection) -> Result<(), Error> {
        let frame = Frame::BulkString(self.msg.clone());

        conn.write_frame(&frame).await?;

        Ok(())
    }
}
