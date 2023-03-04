mod chunk;
mod compiler;
mod convert;
mod debug;
mod interner;
mod interpreter;
mod memory;
mod object;
mod opcode;
mod statements;
mod tokenize;
mod value;
mod vm;

use std::{fs::File, io::Read};

use clap::Parser;
use interpreter::interpret;
use vm::VMMode;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: String,
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

fn main() {
    let cli = Cli::parse();
    let s = cli.path.clone();
    match File::open(cli.path) {
        Ok(mut file) => {
            let mut buf: String = String::new();
            match file.read_to_string(&mut buf) {
                Ok(_) => {
                    let mode = match cli.debug {
                        true => VMMode::Debug,
                        false => VMMode::Run,
                    };

                    interpret(&buf, mode)
                }
                Err(e) => {
                    eprintln!("{}", &s);
                    eprintln!("Error: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("{}", &s);
            eprintln!("Error: {}", e);
        }
    }
}

#[test]

fn run() {
    interpret(
        "吾有一數曰一名之曰「甲」 為是五遍 若「甲」等於三者 乃止 云云 加「甲」以一 昔之「甲」者今其是矣 吾有一言曰「「一遍」」書之 云云",
        VMMode::Debug,
    );
}
