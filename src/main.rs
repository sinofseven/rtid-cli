use clap::Parser;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {}

fn main() {
    let _ = Cli::parse();

    if let Err(msg) = run() {
        eprint!("{}", msg);
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let now = SystemTime::now();
    let unixtime = now
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("failed to get unixtime: {}", e))?;
    let max_number: u128 = 9007199254740991; // 2u64.pow(53) - 1
    let rtid = max_number - unixtime.as_millis();

    println!("{:016}", rtid);

    Ok(())
}
