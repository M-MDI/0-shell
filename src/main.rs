use std::io::{self, Write};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("0-shell - Simple Unix Shell");
    
    loop {
        // Print prompt
        print!("$ ");
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
    
    println!("Goodbye!");
}

fn execute_command(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }
    
    let command = parts[0];
    let args = &parts[1..];
    
    match command {
        "exit" => std::process::exit(0),
        "echo" => cmd_echo(args),
        "pwd" => cmd_pwd(),
        "cd" => cmd_cd(args),
        "ls" => cmd_ls(args),
        "cat" => cmd_cat(args),
        "mkdir" => cmd_mkdir(args),
        "rm" => cmd_rm(args),
        "cp" => cmd_cp(args),
        "mv" => cmd_mv(args),
        _ => println!("Command '{}' not found", command),
    }
}

fn cmd_echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

fn cmd_pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("pwd: {}", e),
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
                    let file_type = if metadata.is_dir() { "d" } else { "-" };
                    println!("{} {}", file_type, name);
                } else {
                    let suffix = if classify && entry.metadata().map(|m| m.is_dir()).unwrap_or(false) {
                        "/"
                    } else {
                        ""
                    };
                    println!("{}{}", name, suffix);
                }
            }
        }
        Err(e) => eprintln!("ls: {}", e),
    }
}

fn cmd_cat(args: &[&str]) {
    if args.is_empty() {
        eprintln!("cat: missing file arguments");
        return;
    }
    
    for file in args {
        match fs::read_to_string(file) {
            Ok(content) => print!("{}", content),
            Err(e) => eprintln!("cat: {}: {}", file, e),
        }
    }
}

fn cmd_mkdir(args: &[&str]) {
    if args.is_empty() {
        eprintln!("mkdir: missing directory arguments");
        return;
    }
    
    for dir in args {
        if let Err(e) = fs::create_dir_all(dir) {
            eprintln!("mkdir: {}: {}", dir, e);
        }
    }
}

fn cmd_rm(args: &[&str]) {
    if args.is_empty() {
        eprintln!("rm: missing file arguments");
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
                if let Err(e) = fs::remove_dir_all(path) {
                    eprintln!("rm: {}: {}", file, e);
                }
            } else {
                eprintln!("rm: {}: is a directory", file);
            }
        } else if let Err(e) = fs::remove_file(path) {
            eprintln!("rm: {}: {}", file, e);
        }
    }
}

fn cmd_cp(args: &[&str]) {
    if args.len() != 2 {
        eprintln!("cp: need exactly 2 arguments");
        return;
    }
    
    if let Err(e) = fs::copy(args[0], args[1]) {
        eprintln!("cp: {}", e);
    }
}

fn cmd_mv(args: &[&str]) {
    if args.len() != 2 {
        eprintln!("mv: need exactly 2 arguments");
        return;
    }
    
    if let Err(e) = fs::rename(args[0], args[1]) {
        eprintln!("mv: {}", e);
    }
}
