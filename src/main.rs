use std::collections::HashMap;

fn main() {
    let mut database = Database::new().expect("Database::new() crashed");
    let mut args = std::env::args().skip(1);
    let command = args.next().expect("argument <command> not provided");
    match command.as_str() {
        "get" => {
            let key = args.next().expect("argument <key> not provided");
            match database.get(key) {
                Some(value) => println!("{value}"),
                None => {
                    println!("unknown key");
                    std::process::exit(1);
                }
            };
        }
        "add" => {
            let key = args.next().expect("argument <key> not provided");
            let value = args.next().expect("argument <value> not provided");
            database.insert(key, value);
            database.flush().unwrap();
        }
        _ => {
            println!("unknown command <{command}>");
            std::process::exit(1);
        }
    }
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map: HashMap<String, String> = HashMap::new();
        let is_database = std::path::Path::new("kv.db").exists();
        if is_database {
            // parse keys/values from kv.db
            let contents = std::fs::read_to_string("kv.db")?;
            // populate map with keys/values
            for line in contents.lines() {
                let (key, value) = line.split_once('\t').expect("Corrupted database");
                map.insert(key.to_owned(), value.to_owned());
            }
        } else {
            std::fs::File::create("kv.db").expect("Failed to create database");
        }
        Ok(Database { map })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn get(&self, key: String) -> Option<&String> {
        return self.map.get(&key);
    }

    fn flush(&self) -> Result<(), std::io::Error> {
        // write keys and values from database to disk
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(&format!("{key}\t{value}\n"));
        }
        std::fs::write("kv.db", contents)
    }
}
