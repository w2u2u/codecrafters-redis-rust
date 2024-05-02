use anyhow::Error;

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

    pub(crate) async fn apply(&self) -> Result<(), Error> {
        Ok(())
    }
}
