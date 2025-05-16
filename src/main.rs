use std::env;

fn main() {
    // let args: Vec<std::ffi::OsString> = std::env::args_os().collect();
    // dbg!(args);
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}
