use std::sync::Arc;

use anyhow::Error;
use tokio::sync::Mutex;

use crate::{connection::Connection, db::Database, frame::Frame};

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct Get {
    key: String,
}

impl Get {
    pub(crate) fn new(args: Vec<String>) -> Self {
        Get {
            key: args.get(1).cloned().unwrap_or(String::new()),
        }
    }

    pub(crate) async fn apply<D>(
        &self,
        conn: &mut Connection,
        db: Arc<Mutex<D>>,
    ) -> Result<(), Error>
    where
        D: Database,
    {
        let frame = if let Some(value) = db.lock().await.get(&self.key) {
            Frame::BulkString(value)
        } else {
            Frame::Null
        };

        conn.write_frame(&frame).await?;

        Ok(())
    }
}
