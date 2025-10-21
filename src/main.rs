mod output;
mod stats;
mod parsing;
mod utils;

use output::{print_stats_detailed, print_stats_json, print_stats_table};
use utils::{fetch_container_stats, get_container_runtime};
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone)]
enum OutputType {
    Json,
    Table,
    Detailed
}

#[derive(Parser, Debug)]
#[command(name = "docking")]
#[command(about = "Docking is a CLI tools to get snapshot of docker or podman resource usage")]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_enum, default_value = "table")]
    output: OutputType
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let runtime = get_container_runtime();
    println!("Using: {}", runtime);
    println!("Getting resource statistic...\n");

    let stats = fetch_container_stats(&runtime)?;

    if stats.is_empty() {
        println!("No container running!");
        return Ok(());
    }

    match args.output {
        OutputType::Json => print_stats_json(&stats),
        OutputType::Detailed => print_stats_detailed(&stats),
        OutputType::Table => print_stats_table(&stats),
    }

    Ok(())
}
