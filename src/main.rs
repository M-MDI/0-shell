use std::io::{self, Write};
use std::env;
use std::fs;
use std::path::Path;

// ANSI color codes
const RED: &str = "\x1b[91m";
const GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const BLUE: &str = "\x1b[94m";
const MAGENTA: &str = "\x1b[95m";
const CYAN: &str = "\x1b[96m";
const WHITE: &str = "\x1b[97m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

fn main() {
    display_banner();
    
    loop {
        // Print colorful prompt
        print!("{}{}${} ", CYAN, BOLD, RESET);
        io::stdout().flush().unwrap();
        
        // Read input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF (Ctrl+D)
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() {
                    continue;
                }
                
                // Parse and execute command
                execute_command(input);
            }
            Err(_) => break,
        }
    }
    
    println!("{}{}Goodbye from 0-shell!{}", RED, BOLD, RESET);
}

fn display_banner() {
    println!("{}{}╔══════════════════════════════════════════════════════════════════════╗{}", RED, BOLD, RESET);
    println!("{}{}║                                                                      ║{}", RED, BOLD, RESET);
    println!("{}{}║  ██████╗       ███████╗██╗  ██╗███████╗██╗     ██╗                   ║{}", RED, BOLD, RESET);
    println!("{}{}║ ██╔═████╗      ██╔════╝██║  ██║██╔════╝██║     ██║                   ║{}", RED, BOLD, RESET);
    println!("{}{}║ ██║██╔██║█████╗███████╗███████║█████╗  ██║     ██║                   ║{}", RED, BOLD, RESET);
    println!("{}{}║ ████╔╝██║╚════╝╚════██║██╔══██║██╔══╝  ██║     ██║                   ║{}", RED, BOLD, RESET);
    println!("{}{}║ ╚██████╔╝      ███████║██║  ██║███████╗███████╗███████╗              ║{}", RED, BOLD, RESET);
    println!("{}{}║  ╚═════╝       ╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝              ║{}", RED, BOLD, RESET);
    println!("{}{}║                                                                      ║{}", RED, BOLD, RESET);
    println!("{}{}║           {}🚀 Minimalist Unix-like Shell in Rust 🦀{}{}             ║{}", RED, BOLD, YELLOW, RED, BOLD, RESET);
    println!("{}{}║                                                                      ║{}", RED, BOLD, RESET);
    println!("{}{}╚══════════════════════════════════════════════════════════════════════╝{}", RED, BOLD, RESET);
    println!();
    println!("{}{}Welcome to 0-Shell!{} Type {}help{} for available commands or {}exit{} to quit.", GREEN, BOLD, RESET, CYAN, RESET, CYAN, RESET);
    println!("{}Press Ctrl+D to exit gracefully.{}", YELLOW, RESET);
    println!();
}

fn execute_command(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }
    
    let command = parts[0];
    let args = &parts[1..];
    
    match command {
        "exit" => {
            println!("{}{}Thanks for using 0-shell! 👋{}", GREEN, BOLD, RESET);
            std::process::exit(0);
        },
        "help" => cmd_help(),
        "echo" => cmd_echo(args),
        "pwd" => cmd_pwd(),
        "cd" => cmd_cd(args),
        "ls" => cmd_ls(args),
        "cat" => cmd_cat(args),
        "mkdir" => cmd_mkdir(args),
        "rm" => cmd_rm(args),
        "cp" => cmd_cp(args),
        "mv" => cmd_mv(args),
        _ => println!("{}Command '{}{}{}' not found. Type '{}help{}' for available commands.{}", 
                     RED, YELLOW, command, RED, CYAN, RED, RESET),
    }
}

fn cmd_help() {
    println!("{}{}📋 0-Shell Available Commands:{}", BLUE, BOLD, RESET);
    println!();
    println!("  {}echo{} [text...]     - Print text to stdout", CYAN, RESET);
    println!("  {}pwd{}               - Print working directory", CYAN, RESET);
    println!("  {}cd{} [directory]    - Change current directory", CYAN, RESET);
    println!("  {}ls{} [-l] [-a] [-F] - List directory contents", CYAN, RESET);
    println!("    {}Options:{}", YELLOW, RESET);
    println!("      {}-l{} : Long format with file details", MAGENTA, RESET);
    println!("      {}-a{} : Show hidden files (starting with .)", MAGENTA, RESET);
    println!("      {}-F{} : Add file type indicators (/ for dirs)", MAGENTA, RESET);
    println!("  {}cat{} [file...]     - Display file contents", CYAN, RESET);
    println!("  {}mkdir{} [dir...]    - Create directories", CYAN, RESET);
    println!("  {}cp{} <src> <dest>   - Copy files", CYAN, RESET);
    println!("  {}rm{} [-r] [file...] - Remove files or directories", CYAN, RESET);
    println!("    {}Options:{}", YELLOW, RESET);
    println!("      {}-r{} : Remove directories recursively", MAGENTA, RESET);
    println!("  {}mv{} <src> <dest>   - Move/rename files", CYAN, RESET);
    println!("  {}help{}              - Show this help message", CYAN, RESET);
    println!("  {}exit{}              - Exit the shell", CYAN, RESET);
    println!();
    println!("{}💡 Tips:{}", YELLOW, RESET);
    println!("  • Use Ctrl+D to exit gracefully");
    println!("  • Commands support multiple arguments where applicable");
    println!("  • File paths can be relative or absolute");
    println!();
}

fn cmd_echo(args: &[&str]) {
    if args.is_empty() {
        println!();
        return;
    }
    println!("{}{}{}", GREEN, args.join(" "), RESET);
}

fn cmd_pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}{}{}", BLUE, path.display(), RESET),
        Err(e) => eprintln!("{}pwd: {}{}", RED, e, RESET),
    }
}

fn cmd_cd(args: &[&str]) {
    let path = if args.is_empty() {
        env::var("HOME").unwrap_or_else(|_| env::var("USERPROFILE").unwrap_or(".".to_string()))
    } else {
        args[0].to_string()
    };
    
    if let Err(e) = env::set_current_dir(&path) {
        eprintln!("cd: {}", e);
    }
}

fn cmd_ls(args: &[&str]) {
    let mut show_all = false;
    let mut long_format = false;
    let mut classify = false;
    let mut path = ".";
    
    for arg in args {
        if arg.starts_with('-') {
            for ch in arg.chars().skip(1) {
                match ch {
                    'a' => show_all = true,
                    'l' => long_format = true,
                    'F' => classify = true,
                    _ => {}
                }
            }
        } else {
            path = arg;
        }
    }
    
    match fs::read_dir(path) {
        Ok(entries) => {
            let mut files: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
            
            for entry in files {
                let name = entry.file_name().to_string_lossy().to_string();
                
                if !show_all && name.starts_with('.') {
                    continue;
                }
                
                if long_format {
                    let metadata = entry.metadata().unwrap_or_else(|_| panic!("Failed to get metadata"));
                    let file_type = if metadata.is_dir() { 
                        format!("{}d{}", BLUE, RESET) 
                    } else { 
                        format!("{}-{}", WHITE, RESET) 
                    };
                    let size = metadata.len();
                    let name_colored = if metadata.is_dir() {
                        format!("{}{}/{}", BLUE, name, RESET)
                    } else {
                        format!("{}{}{}", WHITE, name, RESET)
                    };
                    println!("{} {:>8} {}", file_type, size, name_colored);
                } else {
                    let suffix = if classify && entry.metadata().map(|m| m.is_dir()).unwrap_or(false) {
                        "/"
                    } else {
                        ""
                    };
                    let colored_name = if entry.metadata().map(|m| m.is_dir()).unwrap_or(false) {
                        format!("{}{}{}{}{}", BLUE, BOLD, name, suffix, RESET)
                    } else {
                        format!("{}{}{}{}", WHITE, name, suffix, RESET)
                    };
                    println!("{}", colored_name);
                }
            }
        }
        Err(e) => eprintln!("{}ls: {}{}", RED, e, RESET),
    }
}

fn cmd_cat(args: &[&str]) {
    if args.is_empty() {
        eprintln!("{}cat: missing file arguments{}", RED, RESET);
        return;
    }
    
    for file in args {
        match fs::read_to_string(file) {
            Ok(content) => print!("{}{}{}", CYAN, content, RESET),
            Err(e) => eprintln!("{}cat: {}: {}{}", RED, file, e, RESET),
        }
    }
}

fn cmd_mkdir(args: &[&str]) {
    if args.is_empty() {
        eprintln!("{}mkdir: missing directory arguments{}", RED, RESET);
        return;
    }
    
    for dir in args {
        match fs::create_dir_all(dir) {
            Ok(_) => println!("{}📁 Created directory: {}{}{}", GREEN, BLUE, dir, RESET),
            Err(e) => eprintln!("{}mkdir: {}: {}{}", RED, dir, e, RESET),
        }
    }
}

fn cmd_rm(args: &[&str]) {
    if args.is_empty() {
        eprintln!("{}rm: missing file arguments{}", RED, RESET);
        return;
    }
    
    let mut recursive = false;
    let mut files = Vec::new();
    
    for arg in args {
        if *arg == "-r" {
            recursive = true;
        } else {
            files.push(*arg);
        }
    }
    
    for file in files {
        let path = Path::new(file);
        if path.is_dir() {
            if recursive {
                match fs::remove_dir_all(path) {
                    Ok(_) => println!("{}🗑️  Removed directory: {}{}{}", GREEN, YELLOW, file, RESET),
                    Err(e) => eprintln!("{}rm: {}: {}{}", RED, file, e, RESET),
                }
            } else {
                eprintln!("{}rm: {}: is a directory (use -r to remove){}", RED, file, RESET);
            }
        } else {
            match fs::remove_file(path) {
                Ok(_) => println!("{}🗑️  Removed file: {}{}{}", GREEN, YELLOW, file, RESET),
                Err(e) => eprintln!("{}rm: {}: {}{}", RED, file, e, RESET),
            }
        }
    }
}

fn cmd_cp(args: &[&str]) {
    if args.len() != 2 {
        eprintln!("{}cp: need exactly 2 arguments{}", RED, RESET);
        return;
    }
    
    match fs::copy(args[0], args[1]) {
        Ok(_) => println!("{}📋 Copied: {}{}{} → {}{}{}", GREEN, CYAN, args[0], GREEN, CYAN, args[1], RESET),
        Err(e) => eprintln!("{}cp: {}{}", RED, e, RESET),
    }
}

fn cmd_mv(args: &[&str]) {
    if args.len() != 2 {
        eprintln!("{}mv: need exactly 2 arguments{}", RED, RESET);
        return;
    }
    
    match fs::rename(args[0], args[1]) {
        Ok(_) => println!("{}📦 Moved: {}{}{} → {}{}{}", GREEN, CYAN, args[0], GREEN, CYAN, args[1], RESET),
        Err(e) => eprintln!("{}mv: {}{}", RED, e, RESET),
    }
}
