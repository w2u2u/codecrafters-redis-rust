use std::{sync::Arc, time::SystemTime};

use anyhow::Error;
use tokio::sync::Mutex;

use crate::{
    connection::Connection,
    db::Database,
    frame::Frame,
    util::time::{current_time_with_milliseconds, current_time_with_seconds},
};

#[derive(Debug)]
pub(crate) struct Set {
    key: String,
    value: String,
    exp: Option<SystemTime>,
}

impl Set {
    pub(crate) fn new(args: Vec<String>) -> Option<Self> {
        let exp = {
            let unit = args.get(3).cloned().unwrap_or_default();
            let value = args.get(4).map(|s| s.parse::<u64>().unwrap_or(0));

            match (value, unit.to_lowercase().as_str()) {
                (Some(t), "ex") if t > 0 => Some(current_time_with_seconds(t)),
                (Some(t), "px") if t > 0 => Some(current_time_with_milliseconds(t)),
                (_, _) => None,
            }
        };

        match (args.get(1).cloned(), args.get(2).cloned()) {
            (Some(key), Some(value)) => Some(Set { key, value, exp }),
            (_, _) => None,
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
        db.lock().await.set(&self.key, &self.value, self.exp);

        let frame = Frame::SimpleString(String::from("OK"));

        conn.write_frame(&frame).await?;

        Ok(())
    }
}
