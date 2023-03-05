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

                    interpret(&buf, mode);
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
        "吾有一術名之曰「階乘」欲行是術必先得一數曰「甲」乃行是術曰
        若「甲」等於一者。
            乃得「甲」
        若非
            減「甲」以一名之曰「乙」
            施「階乘」於「乙」名之曰「丙」
            乘「丙」以「甲」。名之曰「丁」
            乃得「丁」
        云云
    是謂「階乘」之術也
    
    施「階乘」於五書之",
        VMMode::Run,
    );
}
