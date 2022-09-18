use std::collections::HashMap;

fn main() {
    let mut database = Database::new().expect("Database::new() crashed");
    let mut args = std::env::args().skip(1);
    let command = args.next().throw_error("argument <command> not provided");
    match command.as_str() {
        "get" => {
            let key = args.next().throw_error("argument <key> not provided");
            println!("{}", database.get(key).throw_error("unknown key"))
        }
        "add" => {
            let key = args.next().throw_error("argument <key> not provided");
            let value = args.next().throw_error("argument <value> not provided");
            database.insert(key, value);
            database.flush().unwrap();
        }
        "remove" => {
            let key = args.next().throw_error("argument <key> not provided");
            database.remove(key).throw_error("unknown key");
            database.flush().unwrap();
        }
        "list" => {
            let contents = database.keys_as_str().throw_error("empty database");
            println!("{contents}");
        }
        _ => {
            database.unknown().throw_error("unknown command");
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
                let (key, value) = line.split_once('\t').expect("corrupted database");
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

    fn remove(&mut self, key: String) -> Option<String> {
        return self.map.remove(&key);
    }

    fn keys_as_str(&self) -> Option<String> {
        let mut contents = String::new();
        for key in self.map.keys() {
            if !key.starts_with(".") {
                contents.push_str(&format!("{key}\n"));
            }
        }
        return match contents.as_str() {
            "" => None,
            _ => Some(contents.trim_end().to_owned()),
        };
    }

    fn flush(&self) -> Result<(), std::io::Error> {
        // write keys and values from database to disk
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(&format!("{key}\t{value}\n"));
        }
        std::fs::write("kv.db", contents)
    }

    fn unknown(&self) -> Option<()> {
        return match "" {
            _ => None,
        };
    }
}

trait OptionWrapper<T> {
    fn throw_error(self, msg: &str) -> T;
}

impl<T> OptionWrapper<T> for Option<T> {
    fn throw_error(self, msg: &str) -> T {
        match self {
            Some(value) => value,
            None => {
                println!("\x1b[31;1merror\x1b[0m: {msg}");
                std::process::exit(1)
            }
        }
    }
}
