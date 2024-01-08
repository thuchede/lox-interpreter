use std::{env, io};
use std::fs::{File, read_to_string};
use std::io::{BufReader, Error, ErrorKind, Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
    } else if let Some(filename) = args.get(1) {
        run_file(filename).expect("File should exist")
    } else {
        run_prompt()
    }
}

fn run_file(filename: &String) -> Result<(), Error>{
    let file = read_to_string(filename.clone());

    if file.is_err() {
        return Err(Error::new(ErrorKind::NotFound,format!("file {} does not exist!", filename)));
    } else {
        let file_content = file.unwrap();
        println!("{}", file_content);
        Ok(())
    }
}

fn run_prompt(){
    println!("Starting prompt");
    let mut input = String::new();
    loop {
        print!("ƛ ");
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("Stopping prompt...");
                break;
            }
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);
            }
            Err(error) => println!("error: {error}"),
        }
        input.clear();
    }
}

fn run(source: &str) {
    todo!()
}
