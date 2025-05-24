use build_warren::build_parser::fetch_build_order;
use build_warren::index_manager::get_st_highest_index;
use clap::{Parser, Subcommand};
use serde_json;
use std::fs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    output: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch the highest build index from Spawning Tool
    BuildCount,

    /// Fetch a specific build order by ID
    Fetch {
        /// The ID of the build order to fetch
        id: u32,
    },

    /// Fetch the latest N build orders (default: 1)
    FetchLatest {
        /// The number of latest build orders to fetch
        #[arg(default_value_t = 1)]
        count: u32,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::BuildCount) => {
            let highest_index = get_st_highest_index();
            if let Some(output_file) = &cli.output {
                fs::write(output_file, highest_index.to_string())
                    .expect("Failed to write to output file");
            } else {
                println!("Highest build index: {}", highest_index);
            }
        }
        Some(Commands::Fetch { id }) => match fetch_build_order(*id) {
            Ok(build_order) => {
                let json_output = serde_json::to_string_pretty(&build_order)
                    .expect("Failed to serialize build order to JSON");
                if let Some(output_file) = &cli.output {
                    fs::write(output_file, json_output)
                        .expect("Failed to write build order to output file");
                } else {
                    println!("{}", json_output);
                }
            }
            Err(e) => eprintln!("Error fetching build order: {}", e),
        },
        Some(Commands::FetchLatest { count }) => {
            let highest_index = get_st_highest_index();
            let start_index = if *count > highest_index {
                1
            } else {
                highest_index - *count + 1
            };
            let mut increment = 0;
            let mut build_orders = Vec::new();
            while increment < *count {
                let current_id = start_index + increment;
                match fetch_build_order(current_id) {
                    Ok(build_order) => {
                        build_orders.push(build_order);
                        increment += 1;
                    }
                    Err(e) => {
                        eprintln!("Error fetching build order {}: {}", current_id, e);
                        increment += 1; // Increment to avoid infinite loop
                        continue; // Some builds might not be available, continue fetching
                    }
                }
            }
            let json_output = serde_json::to_string_pretty(&build_orders)
                .expect("Failed to serialize build orders to JSON");
            if let Some(output_file) = &cli.output {
                fs::write(output_file, json_output)
                    .expect("Failed to write build orders to output file");
            } else {
                println!("{}", json_output);
            }
        }
        None => {
            eprintln!("No command provided. Use --help to see available commands.");
        }
    }
}
