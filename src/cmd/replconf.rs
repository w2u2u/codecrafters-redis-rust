use anyhow::Error;

use crate::{connection::Connection, frame::Frame};

#[derive(Debug)]
pub(crate) struct Replconf {
    conf: Vec<String>,
}

impl Replconf {
    pub(crate) fn new(conf: Vec<String>) -> Self {
        Replconf { conf }
    }

    pub(crate) async fn apply(&self, conn: &mut Connection) -> Result<(), Error> {
        let frame = Frame::SimpleString(String::from("OK"));

        conn.write_frame(&frame).await?;

        Ok(())
    }

    pub(crate) async fn send(&self, conn: &mut Connection) -> Result<(), Error> {
        let frame = Frame::Arrays([vec![String::from("REPLCONF")], self.conf.clone()].concat());

        conn.write_frame(&frame).await?;

        Ok(())
    }
}
