use clap::{Parser, ValueEnum};
use std::process::{Command, exit};


#[derive(ValueEnum, Clone, Debug)]
enum ActionType {
    Install,
    Uninstall,
    Remove,
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_enum)] action: ActionType,
    package: String,
}

fn main() {
    let args = Args::parse();
    
    let pacman_flags: &[&str] = match args.action {
        ActionType::Install => &["-S"],
        ActionType::Uninstall => &["-Rns"],
        ActionType::Remove => &["-Rdd"],
    };

    let status = Command::new("sudo")
        .arg("pacman")
        .args(pacman_flags)
        .arg(&args.package)
        .status();

    match status {
        Ok(exit_status) => {
            if !exit_status.success() {
                exit(exit_status.code().unwrap_or(1));
            }
        }
        Err(err) => {
            eprintln!("Failed to execute pacman: {}", err);
            exit(1);
        }
    }
}
