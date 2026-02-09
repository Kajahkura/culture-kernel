use axum::{
    extract::{State, Request},
    response::{IntoResponse, Response},
    routing::get,
    Router, Json,
};
use clap::{Parser, Subcommand};
use redb::{Database, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use colored::*; // For ANSI colors
use tower_http::cors::CorsLayer;

// --- 1. DATA MODELS ---
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Ritual {
    id: String,
    title: String,
    trigger: String,
    script: String,
}

const RITUALS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("rituals");

// --- 2. CLI STRUCTURE ---
#[derive(Parser)]
#[command(name = "Culture Kernel")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Serve {
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    List,
    Seed,
}

// --- 3. MAIN KERNEL LOOP ---
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let db = Database::create("culture.redb")?;
    let db_arc = Arc::new(db);

    match &cli.command {
        Some(Commands::Seed) => {
            seed_database(&db_arc)?;
            println!("{}", "Database seeded successfully.".green().bold());
        }
        Some(Commands::List) => {
            list_rituals_cli(&db_arc)?;
        }
        Some(Commands::Serve { port }) => {
            start_server(db_arc, *port).await?;
        }
        None => {
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

// Logic for local CLI listing
fn list_rituals_cli(db: &Arc<Database>) -> anyhow::Result<()> {
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

// --- 5. API SERVER LOGIC (THE UPGRADE) ---
async fn start_server(db: Arc<Database>, port: u16) -> anyhow::Result<()> {
    // Self-Healing: Seed if empty
    let needs_seeding = {
        let read_txn = db.begin_read()?;
        read_txn.open_table(RITUALS_TABLE).is_err()
    };

    if needs_seeding {
        println!("{}", "Auto-seeding kernel...".yellow());
        seed_database(&db)?;
    }

    // --- CORS ---
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any) // Allow Blinkhost, Localhost, etc.
        .allow_methods(tower_http::cors::Any); // Allow GET, POST, etc.

    let app = Router::new()
        // We now pass the 'Request' to the handler to check headers
        .route("/rituals", get(api_handle_rituals))
        .with_state(db)
        .layer(cors);

    println!("{} on port {}", "KERNEL LIVE".green().bold(), port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

// THE DUAL-MODE HANDLER
async fn api_handle_rituals(
    State(db): State<Arc<Database>>, 
    req: Request
) -> Response {
    // 1. Fetch Data
    let read_txn = db.begin_read().unwrap();
    let table = read_txn.open_table(RITUALS_TABLE).unwrap();
    
    let mut rituals = Vec::new();
    for item in table.iter().unwrap() {
        let (_, value) = item.unwrap();
        let ritual: Ritual = serde_json::from_str(value.value()).unwrap();
        rituals.push(ritual);
    }

    // 2. Detect User-Agent (Is it Curl?)
    let user_agent = req.headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");

    let is_terminal = user_agent.to_lowercase().contains("curl") 
                   || user_agent.to_lowercase().contains("wget");

    // 3. Return the correct format
    if is_terminal {
        // Render ANSI Art Table
        let mut output = String::new();
        output.push_str(&format!("{}\n", "╔════════════════════════════════════════════════╗".bright_cyan()));
        output.push_str(&format!("║  {}  ║\n", "CULTURE KERNEL :: ACTIVE RITUALS".yellow().bold()));
        output.push_str(&format!("{}\n\n", "╚════════════════════════════════════════════════╝".bright_cyan()));

        for r in rituals {
            output.push_str(&format!("> {}\n", r.title.green().bold()));
            output.push_str(&format!("  ID:      {}\n", r.id.cyan()));
            output.push_str(&format!("  TRIGGER: {}\n", r.trigger));
            output.push_str(&format!("  SCRIPT:  {}\n", r.script.italic()));
            output.push_str("\n──────────────────────────────────────────────────\n\n");
        }
        return output.into_response();
    }

    // Default: Return JSON for Frontend
    Json(rituals).into_response()
}
