use std::env;

mod bf_interpreter;

use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

fn print_uses() {
    println!("use: neurodivergence \"<inlined-code>\"");
    println!("or:  neurodivergence f <filename>");
    panic!();
}

fn get_raw_from_file(path: &Path) -> Result<Vec<u8>, Error> {
    let mut buf: Vec<u8> = Vec::new();
    let raw: Result<Vec<u8>, Error>  = match File::open(path) {
        Err(why) => Err(why),
        Ok(mut file) => { file.read_to_end(&mut buf).unwrap(); Ok(buf)}
    };

    return raw;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file_mode = false;

    let raw_bytes: Vec<u8>;

    if !(args.len() >= 2 && args.len() <= 3) {
        print_uses();
    }

    if args.len() == 3 && args[1] != "f" {
        print_uses();
    }
    else if args.len() == 3 && args[1] == "f" {
        file_mode = true;
    }

    if file_mode {
        match get_raw_from_file(Path::new(&args[2])) {
            Ok(val) => raw_bytes = val,
            Err(_) => panic!("Arquivo inválido")
        }
    }
    else {
        raw_bytes = args[1].clone().into_bytes();
    }
    bf_interpreter::interpret(raw_bytes);

}
