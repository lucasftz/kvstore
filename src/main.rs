fn main() {
    // get the arguments the program was called with
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap(); // unwrap will crash the program if not found
    let value = args.next().expect("Argument <value> not provided"); // expect works like unwrap but it can show a custom msg
    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents).unwrap();
}
