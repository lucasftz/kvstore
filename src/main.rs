fn main() {
    // get the arguments the program was called with
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap(); // unwrap will crash the program if not found
    let value = args.next().expect("Argument <value> not provided"); // expect works like unwrap but it can show a custom msg
    println!("The key is {} and the value is {}", key, value);
}
