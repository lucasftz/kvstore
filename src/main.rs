use std::collections::HashMap;

fn main() {
    // get the arguments the program was called with
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap(); // unwrap will crash the program if not found
    let value = args.next().expect("Argument <value> not provided"); // expect works like unwrap but it can show a custom msg
    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents).unwrap();

    let _database = Database::new().expect("Database::new() crashed");
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
}
