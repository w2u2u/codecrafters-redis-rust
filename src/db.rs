use std::{collections::HashMap, time::SystemTime};

use crate::util::time::is_expired;

pub trait Database {
    fn get(&mut self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: &str, exp: Option<SystemTime>);
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Value {
    data: String,
    exp: Option<SystemTime>,
}

impl Value {
    pub fn new(value: &str, exp: Option<SystemTime>) -> Self {
        Value {
            data: value.to_owned(),
            exp,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.exp.map(is_expired).is_some_and(|t| t)
    }
}

#[derive(Debug)]
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
    fn get(&mut self, key: &str) -> Option<String> {
        if let Some(value) = self.data.get(key) {
            if value.is_expired() {
                self.data.remove(key);
                return None;
            } else {
                return Some(value.data.to_owned());
            }
        }

        None
    }

    fn set(&mut self, key: &str, value: &str, exp: Option<SystemTime>) {
        self.data.insert(key.to_owned(), Value::new(value, exp));
    }
}
