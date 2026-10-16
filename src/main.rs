use rust_environment_api::Environment;

fn print_help() {
    println!("Rust Environment API");
    println!();
    println!("Usage: rust-env <command> [argument]");
    println!();
    println!("Commands:");
    println!("  info              Show system and process information");
    println!("  vars              List environment variables");
    println!("  get <NAME>        Read one environment variable");
    println!("  args              Show process arguments");
    println!("  cwd               Show current working directory");
    println!("  executable        Show executable path");
    println!("  temp              Show temporary directory");
    println!("  system            Show system information");
    println!("  json              Export a complete environment snapshot as JSON");
    println!("  help              Show this help");
}

fn main() {
    let mut args = std::env::args_os();
    let _program = args.next();
    let command = args.next().and_then(|v| v.into_string().ok()).unwrap_or_else(|| "info".into());

    match command.as_str() {
        "info" => {
            let s = Environment::system();
            let cwd = Environment::current_dir().map(|p| p.display().to_string()).unwrap_or_else(|e| e.to_string());
            println!("Rust Environment API");
            println!("--------------------");
            println!("OS:             {}", s.os);
            println!("Family:         {}", s.family);
            println!("Architecture:   {}", s.architecture);
            println!("Process ID:     {}", s.process_id);
            println!("Executable:     {}", s.executable);
            println!("Working Dir:    {cwd}");
            println!("Environment:    {} variables", Environment::variables().len());
        }
        "vars" => {
            for (key, value) in Environment::variables() { println!("{key}={value}"); }
        }
        "get" => match args.next().and_then(|v| v.into_string().ok()) {
            Some(key) => match Environment::get(&key) { Ok(v) => println!("{v}"), Err(e) => { eprintln!("{e}"); std::process::exit(1); } },
            None => { eprintln!("Usage: rust-env get <NAME>"); std::process::exit(2); }
        },
        "args" => for (i, value) in Environment::arguments().to_lossy_strings().iter().enumerate() { println!("[{i}] {value}"); },
        "cwd" => match Environment::current_dir() { Ok(v) => println!("{}", v.display()), Err(e) => { eprintln!("{e}"); std::process::exit(1); } },
        "executable" => match Environment::executable() { Ok(v) => println!("{}", v.display()), Err(e) => { eprintln!("{e}"); std::process::exit(1); } },
        "temp" => println!("{}", Environment::temp_dir().display()),
        "system" => match serde_json::to_string_pretty(&Environment::system()) { Ok(v) => println!("{v}"), Err(e) => { eprintln!("{e}"); std::process::exit(1); } },
        "json" => match Environment::snapshot().and_then(|s| s.to_json_pretty()) { Ok(v) => println!("{v}"), Err(e) => { eprintln!("{e}"); std::process::exit(1); } },
        "help" | "--help" | "-h" => print_help(),
        other => { eprintln!("Unknown command: {other}"); print_help(); std::process::exit(2); }
    }
}
