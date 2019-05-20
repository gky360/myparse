use std::io;
use std::io::{BufRead, Write};
use structopt::StructOpt;

use interpreter::Interpreter;
use parser::Ast;
use rpn_compiler::RpnCompiler;

mod interpreter;
mod lexer;
mod parser;
mod rpn_compiler;

/// Command line options
#[derive(StructOpt, Debug)]
#[structopt()]
pub struct Opt {
    /// Use RPN compiler mode
    #[structopt(short = "c", long = "compiler")]
    pub use_compiler: bool,
}

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

pub fn run(opt: &Opt) -> i32 {
    let mut interp = Interpreter::new();
    let mut compiler = RpnCompiler::new();

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

            if opt.use_compiler {
                let rpn = compiler.compile(&ast);
                println!("{}", rpn);
            } else {
                let n = match interp.eval(&ast) {
                    Ok(n) => n,
                    Err(err) => {
                        err.show_diagnostic(&line);
                        show_trace(err);
                        continue;
                    }
                };
                println!("{}", n);
            }
        } else {
            break;
        }
    }

    0
}
