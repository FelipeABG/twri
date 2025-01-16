use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        println!("Usage: twri [script]");
        std::process::exit(64)
    } else if args.len() == 2 {
        run_file(&args[1])?;
    } else {
        run_prompt()?;
    }

    Ok(())
}

fn run_file(path: &str) -> Result<(), std::io::Error> {
    let file = std::fs::read(path)?;
    run(&String::from_utf8(file).unwrap())?;
    Ok(())
}

fn run_prompt() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let mut input = String::new();

    loop {
        print!("> ");
        std::io::stdout().flush()?;

        input.clear();
        stdin.read_line(&mut input)?;

        if input.trim() == "exit" {
            break;
        }

        run(&input.trim())?;
    }

    Ok(())
}

fn run(input: &str) -> Result<(), std::io::Error> {
    println!("{input}");
    Ok(())
}
