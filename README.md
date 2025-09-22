# Shellko

Shellko is an interactive shell written in **Rust**. It provides a modern, minimalistic terminal with essential features to execute system commands, manage the current directory, and navigate command history easily.

## Features

- Execute **external commands** (e.g., `ls`, `echo`)  
- Built-in **`cd` command** to change directories  
- **Dynamic prompt** showing the current working directory  
- **Command history** with navigation using ↑ and ↓ keys  
- Proper **error handling** for commands not found  

## Installation

1. Clone the repository:
```bash
git clone https://github.com/your-username/Shellko.git
cd Shellko

2. Build and run:
```bash
cargo run

## Usage 
myshell:/current/path> ls
...
myshell:/current/path> cd ..
myshell:/next/path> history
