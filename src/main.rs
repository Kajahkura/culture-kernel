use axum::{
    extract::{Path, State},
    routing::get,
    Router, Json,
};
use clap::{Parser, Subcommand};
use redb::{Database, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use colored::*;

// --- 1. DATA MODELS ---
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Ritual {
    id: String,
    title: String,
    trigger: String,
    script: String,
}

// Define the Database Table
const RITUALS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("rituals");

// --- 2. CLI STRUCTURE ---
#[derive(Parser)]
#[command(name = "Culture Kernel")]
#[command(about = "The Operating System for Organizational Culture", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the HTTP API Server
    Serve {
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    /// List all rituals in the database
    List,
    /// Seed the database 
    Seed,
}

// --- 3. MAIN KERNEL LOOP ---
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // Initialize Embedded DB (redb)
    let db = Database::create("culture.redb")?;
    let db_arc = Arc::new(db);

    match &cli.command {
        Some(Commands::Seed) => {
            seed_database(&db_arc)?;
            println!("{}", "Database seeded successfully.".green().bold());
        }
        Some(Commands::List) => {
            list_rituals(&db_arc)?;
        }
        Some(Commands::Serve { port }) => {
            println!("{} on port {}", "Starting Culture Kernel API".cyan(), port);
            start_server(db_arc, *port).await?;
        }
        None => {
            // Default behavior: Show Help
            println!("{}", "Culture Kernel v1.0".yellow().bold());
            println!("Run 'culture-kernel --help' for commands.");
        }
    }

    Ok(())
}

// --- 4. DATABASE LOGIC ---
fn seed_database(db: &Arc<Database>) -> anyhow::Result<()> {
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(RITUALS_TABLE)?;
        // Load from JSON file
        let data = std::fs::read_to_string("rituals.json")?;
        let rituals: Vec<Ritual> = serde_json::from_str(&data)?;
        
        for ritual in rituals {
            let json = serde_json::to_string(&ritual)?;
            table.insert(ritual.id.as_str(), json.as_str())?;
        }
    }
    write_txn.commit()?;
    Ok(())
}

fn list_rituals(db: &Arc<Database>) -> anyhow::Result<()> {
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(RITUALS_TABLE)?;
    
    println!("{}", " AVAILABLE RITUALS ".on_blue().white().bold());
    for item in table.iter()? {
        let (id, value) = item?;
        let ritual: Ritual = serde_json::from_str(value.value())?;
        println!("{} - {}", id.value().cyan(), ritual.title);
    }
    Ok(())
}

// --- 5. API SERVER LOGIC ---
async fn start_server(db: Arc<Database>, port: u16) -> anyhow::Result<()> {
    
    // --- SELF-HEALING LOGIC START ---
    // Check if the table exists. If not, SEED IT automatically.
    let needs_seeding = {
        let read_txn = db.begin_read()?;
        // Try to open the table; if it fails, we assume it doesn't exist
        read_txn.open_table(RITUALS_TABLE).is_err()
    };

    if needs_seeding {
        println!("{}", "Table 'rituals' missing. Auto-seeding kernel...".yellow());
        seed_database(&db)?;
        println!("{}", "Auto-seeding complete.".green());
    }
    // --- SELF-HEALING LOGIC END ---

    let app = Router::new()
        .route("/rituals", get(api_list_rituals))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn api_list_rituals(State(db): State<Arc<Database>>) -> Json<Vec<Ritual>> {
    let read_txn = db.begin_read().unwrap();
    let table = read_txn.open_table(RITUALS_TABLE).unwrap();
    
    let mut rituals = Vec::new();
    for item in table.iter().unwrap() {
        let (_, value) = item.unwrap();
        let ritual: Ritual = serde_json::from_str(value.value()).unwrap();
        rituals.push(ritual);
    }
    Json(rituals)
}
