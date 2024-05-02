use std::collections::HashMap;

pub trait Database {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: &str, exp: Option<&str>);
}

#[allow(dead_code)]
pub struct Value {
    data: String,
    exp: Option<String>,
}

impl Value {
    pub fn new(value: &str, exp: Option<&str>) -> Self {
        Value {
            data: value.to_owned(),
            exp: exp.map(|e| e.to_owned()),
        }
    }
}

pub struct KeyValueDb {
    data: HashMap<String, Value>,
}

impl KeyValueDb {
    pub fn new() -> Self {
        KeyValueDb {
            data: HashMap::new(),
        }
    }
}

impl Default for KeyValueDb {
    fn default() -> Self {
        Self::new()
    }
}

impl Database for KeyValueDb {
    fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).map(|v| v.data.to_owned())
    }

    fn set(&mut self, key: &str, value: &str, exp: Option<&str>) {
        self.data.insert(key.to_owned(), Value::new(value, exp));
    }
}
