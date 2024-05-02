use anyhow::Error;

use crate::{connection::Connection, frame::Frame};

#[derive(Debug)]
pub struct Ping;

impl Ping {
    pub(crate) async fn apply(&self, conn: &mut Connection) -> Result<(), Error> {
        let frame = Frame::SimpleString(String::from("PONG"));

        conn.write_frame(&frame).await?;

        Ok(())
    }
}
