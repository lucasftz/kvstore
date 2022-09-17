use std::collections::HashMap;

fn main() {
    // get the arguments the program was called with
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap(); // unwrap will crash the program if not found
    let value = args.next().expect("Argument <value> not provided"); // expect works like unwrap but it can show a custom msg
    let mut database = Database::new().expect("Database::new() crashed");
    database.insert(key, value);
    database.flush().unwrap();
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // parse keys/values from kv.db
        let contents = match std::fs::read_to_string("kv.db") {
            Ok(c) => c, // bind contents to <c> and return it
            Err(e) => {
                return Err(e);
            }
        };
        // a more concise way to write this would be the following
        // let contents = std::fs::read_to_string("kv.db")?;

        // populate map with keys/values
        let mut map: HashMap<String, String> = HashMap::new();
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupted database");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { map: map })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(&self) -> Result<(), std::io::Error> {
        // writes keys and values from database to disk
        let mut contents = String::new();
        for (key, value) in &self.map {
            let kvpair = format!("{}\t{}\n", key, value);
            contents.push_str(&kvpair); // can also use + operator to concat strings
        }
        std::fs::write("kv.db", contents)
    }
}
