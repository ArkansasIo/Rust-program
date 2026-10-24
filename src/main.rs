use rust_environment_api::{
    Account, ApiApplication, ApiApplicationConfig, Environment, Settings, default_menu,
};
use std::str::FromStr;

fn print_help() {
    println!("Rust Environment + Mathematics API");
    println!();
    println!("Usage: rust-env <command> [arguments]");
    println!();
    println!("Core: info | vars | get <NAME> | args | cwd | executable | temp | system | json");
    println!("Application: account | menu | settings | api | api-app");
    println!(
        "Math: math prime <N> | math classify <N> | math gcd <A> <B> | math calc <OP> <A> <B>"
    );
    println!("Use `rust-env help` for this command list.");
}

fn parse_i64(value: Option<std::ffi::OsString>, usage: &str) -> i64 {
    match value
        .and_then(|v| v.into_string().ok())
        .and_then(|v| i64::from_str(&v).ok())
    {
        Some(v) => v,
        None => {
            eprintln!("Invalid or missing integer. Usage: {usage}");
            std::process::exit(2);
        }
    }
}

fn run_math(args: &mut impl Iterator<Item = std::ffi::OsString>) {
    use rust_environment_api::math::{checked_integer_operation, classify_integer, gcd, is_prime};
    let action = args
        .next()
        .and_then(|v| v.into_string().ok())
        .unwrap_or_default();
    match action.as_str() {
        "prime" => {
            let n = parse_i64(args.next(), "rust-env math prime <N>");
            println!("{}", is_prime(n));
        }
        "classify" => {
            let n = parse_i64(args.next(), "rust-env math classify <N>");
            println!(
                "{}",
                serde_json::to_string_pretty(&classify_integer(n))
                    .expect("classification serializes")
            );
        }
        "gcd" => {
            let a = parse_i64(args.next(), "rust-env math gcd <A> <B>");
            let b = parse_i64(args.next(), "rust-env math gcd <A> <B>");
            match gcd(a, b) {
                Some(value) => println!("{value}"),
                None => {
                    eprintln!("Cannot represent absolute magnitude of i64::MIN.");
                    std::process::exit(1);
                }
            }
        }
        "calc" => {
            let op = args
                .next()
                .and_then(|v| v.into_string().ok())
                .unwrap_or_default();
            let a = parse_i64(args.next(), "rust-env math calc <OP> <A> <B>");
            let b = parse_i64(args.next(), "rust-env math calc <OP> <A> <B>");
            match checked_integer_operation(&op, a, b) {
                Some(value) => println!("{value}"),
                None => {
                    eprintln!(
                        "Invalid operation or result: check operator, division by zero, integer division, and overflow."
                    );
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Usage: rust-env math <prime|classify|gcd|calc> ...");
            std::process::exit(2);
        }
    }
}

fn main() {
    let mut args = std::env::args_os();
    let _program = args.next();
    let command = args
        .next()
        .and_then(|v| v.into_string().ok())
        .unwrap_or_else(|| "info".into());

    match command.as_str() {
        "info" => {
            let s = Environment::system();
            let cwd = Environment::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|e| e.to_string());
            println!("Rust Environment API");
            println!("--------------------");
            println!("OS:             {}", s.os);
            println!("Family:         {}", s.family);
            println!("Architecture:   {}", s.architecture);
            println!("Process ID:     {}", s.process_id);
            println!("Executable:     {}", s.executable);
            println!("Working Dir:    {cwd}");
            println!(
                "Environment:    {} variables",
                Environment::variables().len()
            );
        }
        "vars" => {
            for (key, value) in Environment::variables() {
                println!("{key}={value}");
            }
        }
        "get" => match args.next().and_then(|v| v.into_string().ok()) {
            Some(key) => match Environment::get(&key) {
                Ok(v) => println!("{v}"),
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(1);
                }
            },
            None => {
                eprintln!("Usage: rust-env get <NAME>");
                std::process::exit(2);
            }
        },
        "args" => {
            for (i, value) in Environment::arguments()
                .to_lossy_strings()
                .iter()
                .enumerate()
            {
                println!("[{i}] {value}");
            }
        }
        "cwd" => match Environment::current_dir() {
            Ok(v) => println!("{}", v.display()),
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        },
        "executable" => match Environment::executable() {
            Ok(v) => println!("{}", v.display()),
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        },
        "temp" => println!("{}", Environment::temp_dir().display()),
        "system" => match serde_json::to_string_pretty(&Environment::system()) {
            Ok(v) => println!("{v}"),
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        },
        "json" => match Environment::snapshot().and_then(|s| s.to_json_pretty()) {
            Ok(v) => println!("{v}"),
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        },
        "account" => println!(
            "{}",
            serde_json::to_string_pretty(&Account::new("acct-1", "demo", "demo@example.invalid"))
                .unwrap()
        ),
        "menu" => println!("{}", serde_json::to_string_pretty(&default_menu()).unwrap()),
        "settings" => println!(
            "{}",
            serde_json::to_string_pretty(&Settings::default()).unwrap()
        ),
        "api" => println!(
            "{}",
            serde_json::to_string_pretty(&ApiApplicationConfig::default()).unwrap()
        ),
        "api-app" => {
            let app = ApiApplication::new("app-1", "Rust Math Client", "acct-1");
            println!("{}", serde_json::to_string_pretty(&app).unwrap());
        }
        "math" => run_math(&mut args),
        "help" | "--help" | "-h" => print_help(),
        other => {
            eprintln!("Unknown command: {other}");
            print_help();
            std::process::exit(2);
        }
    }
}
