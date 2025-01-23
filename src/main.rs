use interp::{error::SyntaxError, lexer::Lexer, parser::Parser};
use std::io::Write;

struct Args {
    commands: Vec<String>,
}

impl Args {
    fn parse() -> Self {
        Self {
            commands: std::env::args().collect(),
        }
    }
}

fn main() {
    let args = Args::parse();

    match args.commands.len() {
        1 => {
            if let Err(e) = run_prompt() {
                eprintln!("{e}");
                std::process::exit(65)
            }
        }
        2 => {
            if let Err(e) = run_file(&args.commands[1]) {
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

fn run_prompt() -> Result<(), SyntaxError> {
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

        match run(&input) {
            Ok(_) => continue,
            Err(e) => eprintln!("{e}"),
        }
    }
}

fn run_file(path: &str) -> Result<(), SyntaxError> {
    let f = std::fs::read(path).unwrap();
    run(&String::from_utf8(f).unwrap())
}

fn run(source: &str) -> Result<(), SyntaxError> {
    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer.tokenized()?.map(|t| t.clone()).collect());
    println!("{:#?}", parser.parse()?);

    Ok(())
}
