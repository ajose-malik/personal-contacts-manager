use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug)]
struct Record {
    id: i64,
    name: String,
    email: Option<String>,
}

#[derive(Debug)]
struct Records {
    inner: HashMap<i64, Record>,
}

impl Records {
    fn new() -> Self {
      self { 
          inner: HashMap::new(),
        }
    }

    fn add(&mut self, record: Record) {
        self.inner.insert(record.id, record)
    }
}

#[derive(Error, Debug)]
enum ParserError {
    #[error("id must be a number: {0}")]
    InvalidId(#[from] std::num::ParseIntError)
    #[error("empty record")]
    Emptyrecord,
    #[error("missing field: {0}")]
    MissingField(String)
}


fn main () {
    
}