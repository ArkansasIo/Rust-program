use rust_environment_api::{
    default_menu, Account, ApiApplication, ApiApplicationConfig, Environment, Settings,
};

fn print_help() {
    println!("Rust Environment + Mathematics API");
    println!();
    println!("Usage: rust-env <command> [argument]");
    println!();
    println!("Core:");
    println!("  info              Show system and process information");
    println!("  vars              List environment variables (likely secrets redacted)");
    println!("  vars --raw        List every environment variable without redaction");
    println!("  get <NAME>        Read one environment variable");
    println!("  args              Show process arguments");
    println!("  cwd               Show current working directory");
    println!("  executable        Show executable path");
    println!("  temp              Show temporary directory");
    println!("  system            Show system information");
    println!("  json              Export a redacted environment snapshot as JSON");
    println!("  json --raw        Export an unredacted snapshot (may expose secrets)");
    println!();
    println!("Application:");
    println!("  account           Show default account model");
    println!("  menu              Show menus and submenus as JSON");
    println!("  settings          Show default settings as JSON");
    println!("  api               Show API application configuration");
    println!("  api-app           Show an example API application");
    println!("  help              Show this help");
}

fn print_json<T: serde::Serialize>(value: &T) {
    match serde_json::to_string_pretty(value) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("Failed to serialize JSON: {error}");
            std::process::exit(1);
        }
    }
}

fn main() {
    let mut args = std::env::args_os();
    let _program = args.next();
    let command = args
        .next()
        .and_then(|value| value.into_string().ok())
        .unwrap_or_else(|| "info".into());

    match command.as_str() {
        "info" => {
            let system = Environment::system();
            let cwd = Environment::current_dir()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|error| error.to_string());
            println!("Rust Environment API");
            println!("--------------------");
            println!("OS:             {}", system.os);
            println!("Family:         {}", system.family);
            println!("Architecture:   {}", system.architecture);
            println!("Process ID:     {}", system.process_id);
            println!("Executable:     {}", system.executable);
            println!("Working Dir:    {cwd}");
            println!("Environment:    {} variables", Environment::variables().len());
        }
        "vars" => {
            let raw = args.next().is_some_and(|value| value == "--raw");
            if raw {
                eprintln!("Warning: raw environment output can expose secrets.");
            }
            let variables = if raw {
                Environment::variables_unredacted()
            } else {
                Environment::variables()
            };
            for (key, value) in variables {
                println!("{key}={value}");
            }
        }
        "get" => match args.next().and_then(|value| value.into_string().ok()) {
            Some(key) => match Environment::get(&key) {
                Ok(value) => {
                    if Environment::is_sensitive_variable_name(&key) {
                        eprintln!("Warning: {key} appears sensitive; printing its value explicitly.");
                    }
                    println!("{value}");
                }
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            },
            None => {
                eprintln!("Usage: rust-env get <NAME>");
                std::process::exit(2);
            }
        },
        "args" => {
            for (index, value) in Environment::arguments().to_lossy_strings().iter().enumerate() {
                println!("[{index}] {value}");
            }
        }
        "cwd" => match Environment::current_dir() {
            Ok(value) => println!("{}", value.display()),
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        },
        "executable" => match Environment::executable() {
            Ok(value) => println!("{}", value.display()),
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        },
        "temp" => println!("{}", Environment::temp_dir().display()),
        "system" => print_json(&Environment::system()),
        "json" => {
            let raw = args.next().is_some_and(|value| value == "--raw");
            if raw {
                eprintln!("Warning: raw environment JSON can expose secrets.");
            }
            let snapshot = if raw {
                Environment::snapshot_unredacted()
            } else {
                Environment::snapshot()
            };
            match snapshot.and_then(|value| value.to_json_pretty()) {
                Ok(json) => println!("{json}"),
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            }
        }
        "account" => print_json(&Account::new(
            "acct-1",
            "demo",
            "demo@example.invalid",
        )),
        "menu" => print_json(&default_menu()),
        "settings" => print_json(&Settings::default()),
        "api" => print_json(&ApiApplicationConfig::default()),
        "api-app" => {
            let application = ApiApplication::new("app-1", "Rust Math Client", "acct-1");
            print_json(&application);
        }
        "help" | "--help" | "-h" => print_help(),
        other => {
            eprintln!("Unknown command: {other}");
            print_help();
            std::process::exit(2);
        }
    }
}
