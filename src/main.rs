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

fn parse_records(records: String, verbose: bool) -> Records {
    let mut recs = Records::new();
    for (num, record) in records.split('\n').enumerate() {
        if record != "" {
            match parse_record(record) {
                Ok(rec) => recs.add(rec),
                Err(e) => {
                    if verbose {
                        println!(
                            "error on line number {}:  {}\n  > \"{}\"\n",
                            num + 1,
                            e,
                            record
                        )
                    }
                }
            }
        }
    }
}

fn load_records(file_name: PathBuf, verbose; bool) -> std::io::Result<Records> {
    let mut file = File::open(file_name)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(parse_records(buffer, verbose))

}


fn main () {
    
}