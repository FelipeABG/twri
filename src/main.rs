use interp::{error::InterpErr, interp::Interpreter, lexer::Lexer, parser::Parser};
use std::io::Write;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut interp = Interpreter::new();

    match args.len() {
        1 => {
            if let Err(e) = repl(&mut interp) {
                eprintln!("{e}");
                std::process::exit(65)
            }
        }
        2 => {
            if let Err(e) = run_file(&args[1], &mut interp) {
                eprintln!("{e}");
                std::process::exit(65)
            }
        }
        _ => {
            eprintln!("Usage: twli [script]");
            std::process::exit(64)
        }
    }
}

fn repl(interp: &mut Interpreter) -> Result<(), InterpErr> {
    let mut input = String::new();
    let stdin = std::io::stdin();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        input.clear();
        stdin.read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            return Ok(());
        }

        match run(&input, interp) {
            Ok(_) => continue,
            Err(e) => eprintln!("> {e}"),
        }
    }
}

fn run_file(path: &str, interp: &mut Interpreter) -> Result<(), InterpErr> {
    let f = std::fs::read(path).unwrap();
    run(&String::from_utf8(f).unwrap(), interp)
}

fn run(source: &str, interp: &mut Interpreter) -> Result<(), InterpErr> {
    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.tokenized()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    interp.interpret(ast)?;
    Ok(())
}
