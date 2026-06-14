#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "aero-codex-agent")]
#[command(about = "Agentic AeroCodex developer CLI scaffold")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    BuildIndex {
        #[arg(long)]
        check: bool,
    },
    ListTools,
    Schema {
        tool_name: String,
    },
    Search {
        query: String,
    },
    Explain {
        codex_id: String,
    },
    Verify {
        codex_id: String,
    },
    Invoke {
        tool_name: String,
        #[arg(long)]
        json: String,
    },
    TraceReplay {
        trace_file: String,
    },
    GraphExport,
    Doctor,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::BuildIndex { check } => {
            println!("TODO build agent index; check={check}");
        }
        Commands::ListTools => println!("TODO list registered agent tools"),
        Commands::Schema { tool_name } => println!("TODO print schema for {tool_name}"),
        Commands::Search { query } => println!("TODO search agent registry for: {query}"),
        Commands::Explain { codex_id } => println!("TODO explain Codex ID: {codex_id}"),
        Commands::Verify { codex_id } => println!("TODO verify Codex ID: {codex_id}"),
        Commands::Invoke { tool_name, json } => {
            println!("TODO invoke {tool_name} with input file {json}");
        }
        Commands::TraceReplay { trace_file } => println!("TODO replay trace {trace_file}"),
        Commands::GraphExport => println!("TODO export capability graph"),
        Commands::Doctor => println!("TODO run agentic workspace diagnostics"),
    }
}
