use std::collections::HashMap;

fn main() {
    let mut database = Database::new().expect("Database::new() crashed");
    let mut args = std::env::args().skip(1);
    let command = args.next().if_error("argument <command> not provided");
    match command.as_str() {
        "get" => {
            let key = args.next().if_error("argument <key> not provided");
            println!("{}", database.get(key).if_error("unknown key"))
        }
        "add" => {
            let key = args.next().if_error("argument <key> not provided");
            let value = args.next().if_error("argument <value> not provided");
            database.insert(key, value);
            database.flush().unwrap();
        }
        "remove" => {
            let key = args.next().if_error("argument <key> not provided");
            database.remove(key).if_error("unknown key");
            database.flush().unwrap();
        }
        "list" => {
            let contents = database.keys_as_str().if_error("empty database");
            println!("{contents}");
        }
        "help" => {
            println!("{}", database.get_help_txt());
        }
        _ => {
            database.unknown().if_error("unknown command");
        }
    }
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map: HashMap<String, String> = HashMap::new();
        let is_database = database_path().exists();
        if is_database {
            // parse keys/values from kv.db
            let contents = std::fs::read_to_string(database_path())?;
            // populate map with keys/values
            for line in contents.lines() {
                let (key, value) = line.split_once('\t').expect("corrupted database");
                map.insert(key.to_owned(), value.to_owned());
            }
        } else {
            std::fs::File::create(database_path()).expect("failed to create database");
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

    fn get_help_txt(&self) -> String {
        return "A key value store written in Rust.\nCOMMANDS:\n\tadd <KEY> <VALUE>\tAdd a key value pair to the database\n\tget <KEY>\t\tAccess the corresponding value of <KEY>\n\thelp\t\t\tList all commands and their uses\n\tlist\t\t\tList all keys stored in the database\n\tremove <KEY>\t\tRemove a key value pair from the database\n\nSECRET KEYS:\nAdding a key prefixed by a dot will make it a secret key! It won't be listed though the list command, but you will be able to access its value normally.\n".to_owned();
    }

    fn flush(&self) -> Result<(), std::io::Error> {
        // write keys and values from database to disk
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(&format!("{key}\t{value}\n"));
        }
        std::fs::write(database_path(), contents)
    }

    fn unknown(&self) -> Option<()> {
        return match "" {
            _ => None,
        };
    }
}

trait OptionWrapper<T> {
    fn if_error(self, msg: &str) -> T;
}

impl<T> OptionWrapper<T> for Option<T> {
    fn if_error(self, msg: &str) -> T {
        match self {
            Some(value) => value,
            None => {
                println!("\x1b[31;1merror\x1b[0m: {msg}");
                std::process::exit(1)
            }
        }
    }
}

fn database_path() -> std::path::PathBuf {
    return match home::home_dir() {
        Some(mut directory) => {
            directory.push(".kv.db");
            directory
        }
        None => panic!("home directory not found"),
    };
}
