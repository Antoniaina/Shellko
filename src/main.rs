use rustyline::error::ReadlineError;
use rustyline::Editor; 
use rustyline::history::DefaultHistory;

use std::env;
use std::process::Command;

fn main() {
    let mut rl: Editor<(), DefaultHistory> = Editor::new().unwrap();

    loop {
        let path = env::current_dir().unwrap();
        let prompt = format!("Shellko • {}> ", path.display());

        let line = rl.readline(&prompt);
        match line {
            Ok(input) => {
                let input = input.trim();
                if input.is_empty() { continue; }
                if input == "exit" { break; }

                rl.add_history_entry(input);

                let parts: Vec<&str> = input.split_whitespace().collect();
                let command = parts[0];
                let args = &parts[1..];

                if command == "cd" {
                    let target = if args.is_empty() {
                        env::var("HOME").unwrap_or("/".to_string())
                    } else {
                        args[0].to_string()
                    };
                    if let Err(e) = env::set_current_dir(&target) {
                        println!("cd: {}: {}", target, e);
                    }
                    continue;
                }

                match Command::new(command).args(args).status() {
                    Ok(s) => {
                        if !s.success() {
                            println!("{} exited with status: {}", command, s);
                        }
                    }
                    Err(_) => println!("{}: command not found", command),
                }
            },
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
