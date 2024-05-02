use anyhow::Error;

use crate::{connection::Connection, frame::Frame};

#[derive(Debug)]
pub struct Ping {
    msg: Option<String>,
}

impl Ping {
    pub(crate) fn new(msg: Option<&str>) -> Self {
        Ping {
            msg: msg.map(|s| s.to_owned()),
        }
    }

    pub(crate) async fn apply(&self, conn: &mut Connection) -> Result<(), Error> {
        let frame = Frame::SimpleString(String::from("PONG"));

        conn.write_frame(&frame).await?;

        Ok(())
    }

    pub(crate) async fn send(&self, conn: &mut Connection) -> Result<(), Error> {
        if let Some(msg) = self.msg.clone() {
            let frame = Frame::Arrays(vec![msg]);

            conn.write_frame(&frame).await?;
        }

        Ok(())
    }
}
