use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    // now we need to store these
    let query = &args[1];
    let file_path = &args[2];

    println!("The query is {query}");
    println!("The file path is {file_path}")
}
