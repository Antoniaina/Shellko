use std::env;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        let path = env::current_dir().unwrap();
        print!("Shellko • {}> ", path.display());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let input = input.trim();
        if input.is_empty() { continue; }
        if input == "exit" { break; }

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
                println!("{}: {}", target, e);
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
    }
}
