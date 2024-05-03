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
            Frame::BulkString(s) => {
                self.write_bulk(s).await?;
            }
            Frame::Arrays(a) => {
                let l = a.len().to_string();
                self.stream.write_u8(b'*').await?;
                self.stream.write_all(l.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
                for s in a {
                    self.write_bulk(s).await?;
                }
            }
            Frame::BulkBytes(b) => {
                self.write_bytes(b).await?;
            }
            Frame::Error => {}
        }

        self.stream.flush().await?;

        Ok(())
    }

    async fn write_bulk(&mut self, s: &str) -> Result<(), Error> {
        let l = s.len().to_string();
        self.stream.write_u8(b'$').await?;
        self.stream.write_all(l.as_bytes()).await?;
        self.stream.write_all(b"\r\n").await?;
        self.stream.write_all(s.as_bytes()).await?;
        self.stream.write_all(b"\r\n").await?;
        Ok(())
    }

    async fn write_bytes(&mut self, b: &[u8]) -> Result<(), Error> {
        let l = b.len().to_string();
        self.stream.write_u8(b'$').await?;
        self.stream.write_all(l.as_bytes()).await?;
        self.stream.write_all(b"\r\n").await?;
        self.stream.write_all(b).await?;
        Ok(())
    }
}
