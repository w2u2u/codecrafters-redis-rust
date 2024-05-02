use std::sync::Arc;

use anyhow::Error;
use tokio::sync::Mutex;

use crate::{connection::Connection, db::Database, frame::Frame};

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct Set {
    key: String,
    value: String,
    exp: Option<String>,
}

impl Set {
    pub(crate) fn new(args: Vec<String>) -> Self {
        let exp = {
            let unit = args.get(3).map(|s| s.as_str());
            let value = args.get(4);

            match (value, unit) {
                (Some(t), Some("ex")) => Some(format!("{t}s")),
                (Some(t), Some("px")) => Some(format!("{t}ms")),
                (_, _) => None,
            }
        };

        Set {
            key: args.get(1).cloned().unwrap_or(String::new()),
            value: args.get(2).cloned().unwrap_or(String::new()),
            exp,
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
        db.lock().await.set(&self.key, &self.value, None);

        let frame = Frame::SimpleString(String::from("OK"));

        conn.write_frame(&frame).await?;

        Ok(())
    }
}
