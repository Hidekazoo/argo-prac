use clap::{Parser, Subcommand};
use std::process;
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "pipeline-cli", about = "A simple pipeline CLI tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Step 1: Fetch data
    Fetch,
    /// Step 2: Transform data
    Transform,
    /// Step 3: Report results
    Report,
    /// Generate a JSON list of items
    List,
    /// Process a single item
    Process {
        /// Item name to process
        #[arg(long)]
        item: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fetch => {
            println!("[fetch] Fetching data from source...");
            println!("[fetch] Retrieved 42 records.");
            println!("[fetch] Done.");
        }
        Commands::Transform => {
            println!("[transform] Transforming data...");
            println!("[transform] Applied 3 filters, 1 aggregation.");
            println!("[transform] Done.");
        }
        Commands::Report => {
            println!("[report] Generating report...");
            println!("[report] Summary: 42 records processed, 38 passed, 4 failed.");
            println!("[report] Done.");
        }
        Commands::List => {
            // API呼び出しの代わりにダミーリストを生成
            let items: Vec<String> = (1..=10).map(|i| format!("item-{i:03}")).collect();
            println!("{}", serde_json::to_string(&items).unwrap());
        }
        Commands::Process { item } => {
            println!("[process] Starting: {item}");
            thread::sleep(Duration::from_secs(3));
            // item-003 と item-007 は失敗するシミュレーション
            if item == "item-003" || item == "item-007" {
                eprintln!("[process] ERROR: {item} failed!");
                process::exit(1);
            }
            println!("[process] Completed: {item}");
        }
    }
}
