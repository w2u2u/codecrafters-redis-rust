use anyhow::Error;
use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

use crate::frame::Frame;

pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(1024),
        }
    }

    pub async fn read_frame(&mut self) -> Result<Frame, Error> {
        self.buffer.clear();
        self.stream.read_buf(&mut self.buffer).await?;

        Frame::parse(&self.buffer[..])
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> Result<(), Error> {
        match frame {
            Frame::SimpleString(s) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(s.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            }
            Frame::BulkString(_) => todo!(),
            Frame::Arrays(_) => todo!(),
            Frame::Unknown => todo!(),
        }

        self.stream.flush().await?;

        Ok(())
    }
}
