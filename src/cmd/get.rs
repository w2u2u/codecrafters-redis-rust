use anyhow::Error;

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

    pub(crate) async fn apply(&self) -> Result<(), Error> {
        Ok(())
    }
}
