use std::io;
use std::io::{BufRead, Write};

mod lexer;
mod parser;

fn prompt(s: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write(s.as_bytes())?;
    stdout.flush()
}

pub fn run() -> i32 {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stdin = io::BufReader::new(stdin);
    let mut lines = stdin.lines();

    loop {
        prompt("> ").unwrap();
        if let Some(Ok(line)) = lines.next() {
            let tokens = lexer::Lexer::new(&line).lex();
            println!("{:?}", tokens);
        } else {
            break;
        }
    }

    0
}
