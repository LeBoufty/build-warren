use build_warren::build_parser::fetch_build_order;
use build_warren::handlers::{fetch_latest, fetch_segment};
use build_warren::index_manager::get_st_highest_index;
use clap::{Parser, Subcommand};
use console::{Emoji, style};
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

    /// Fetch a segment of build orders
    FetchSegment {
        /// The starting index of the segment
        start: u32,
        /// The ending index of the segment
        end: u32,
    },
}

static CLIPBOARD_EMOJI: Emoji = Emoji("📋 ", "");
static OUTPUT_EMOJI: Emoji = Emoji("📂 ", "");

fn main() {
    let cli = Cli::parse();

    println!(
        "{} {} {}",
        CLIPBOARD_EMOJI,
        style("Build Warren CLI").bold().magenta(),
        style("v0.1.0").dim()
    );

    match &cli.command {
        Some(Commands::BuildCount) => {
            let highest_index = get_st_highest_index();
            if let Some(output_file) = &cli.output {
                fs::write(output_file, highest_index.to_string())
                    .expect("Failed to write to output file");
                println!(
                    "{} {}Highest build index written to {}",
                    OUTPUT_EMOJI,
                    style("Success : ").green(),
                    output_file
                );
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
                    println!(
                        "{} {}Build order {} written to {}",
                        OUTPUT_EMOJI,
                        style("Success : ").green(),
                        id,
                        output_file
                    );
                } else {
                    println!("{}", json_output);
                }
            }
            Err(e) => eprintln!("Error fetching build order: {}", e),
        },
        Some(Commands::FetchLatest { count }) => {
            let build_orders = fetch_latest(*count);
            let json_output = serde_json::to_string_pretty(&build_orders)
                .expect("Failed to serialize build orders to JSON");
            if let Some(output_file) = &cli.output {
                fs::write(output_file, json_output)
                    .expect("Failed to write build orders to output file");
                println!(
                    "{} {}Latest {} build orders written to {}",
                    OUTPUT_EMOJI,
                    style("Success : ").green(),
                    count,
                    output_file
                );
            } else {
                println!("{}", json_output);
            }
        }
        None => {
            eprintln!("No command provided. Use --help to see available commands.");
        }
        Some(Commands::FetchSegment { start, end }) => {
            let build_orders = fetch_segment(*start, *end);
            let json_output = serde_json::to_string_pretty(&build_orders)
                .expect("Failed to serialize build orders to JSON");
            if let Some(output_file) = &cli.output {
                fs::write(output_file, json_output)
                    .expect("Failed to write build orders to output file");
                println!(
                    "{} {}Build orders from {} to {} written to {}",
                    OUTPUT_EMOJI,
                    style("Success : ").green(),
                    start,
                    end,
                    output_file
                );
            } else {
                println!("{}", json_output);
            }
        }
    }
}
