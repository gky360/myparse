use std::io;
use std::io::{BufRead, Write};

use interpreter::Interpreter;
use parser::Ast;

mod interpreter;
mod lexer;
mod parser;

fn show_trace<E: std::error::Error>(err: E) {
    eprintln!("{}", err);
    let mut source = err.source();
    while let Some(err) = source {
        eprintln!("caused by: {}", err);
        source = err.source();
    }
}

fn prompt(s: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write(s.as_bytes())?;
    stdout.flush()
}

pub fn run() -> i32 {
    let mut interp = Interpreter::new();

    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stdin = io::BufReader::new(stdin);
    let mut lines = stdin.lines();

    loop {
        prompt("> ").unwrap();
        if let Some(Ok(line)) = lines.next() {
            let ast = match line.parse::<Ast>() {
                Ok(ast) => ast,
                Err(err) => {
                    err.show_diagnostic(&line);
                    show_trace(err);
                    continue;
                }
            };

            let n = match interp.eval(&ast) {
                Ok(n) => n,
                Err(err) => {
                    err.show_diagnostic(&line);
                    show_trace(err);
                    continue;
                }
            };

            println!("{}", n);
        } else {
            break;
        }
    }

    0
}
