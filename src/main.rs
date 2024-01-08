use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
    } else if let Some(filename) = args.get(1) {
        run_file(filename.clone())
    } else {
        run_prompt()
    }
}

fn run_file(filename: String){
    println!("Starting file {}", filename);
}

fn run_prompt(){
    println!("Starting prompt");
}
