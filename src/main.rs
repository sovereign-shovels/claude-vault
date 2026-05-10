use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod config;
mod db;
mod importer;

use config::Config;
use db::Vault;

#[derive(Parser)]
#[command(name = "claude-vault")]
#[command(about = "Local-first, vendor-agnostic vault for all your AI conversations")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Import conversations from a JSON export file
    Import {
        /// Path to the export JSON file
        path: PathBuf,
        /// Provider type (claude, chatgpt, auto)
        #[arg(short, long, default_value = "auto")]
        provider: String,
    },
    /// Search conversations using full-text search
    Search {
        /// Search query
        query: String,
    },
    /// List all conversations
    List,
    /// Tag a conversation
    Tag {
        /// Conversation ID
        conversation_id: String,
        /// Tag to add
        tag: String,
    },
    /// List tags (for a conversation or globally)
    Tags {
        /// Optional conversation ID
        conversation_id: Option<String>,
    },
    /// Show vault statistics
    Stats,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load();
    let vault_path = config.vault_path.unwrap_or_else(|| {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("claude-vault")
            .join("vault.db")
    });

    // Ensure parent directory exists
    if let Some(parent) = vault_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let vault = Vault::open(&vault_path)?;

    match cli.command {
        Commands::Import { path, provider } => {
            let detected = if provider == "auto" {
                importer::detect_provider(&path).unwrap_or("claude")
            } else {
                &provider
            };

            let count = match detected {
                "claude" => importer::import_claude(&vault, &path)?,
                "chatgpt" => importer::import_chatgpt(&vault, &path)?,
                _ => {
                    eprintln!("Unknown provider: {}. Use 'claude' or 'chatgpt'.", detected);
                    std::process::exit(1);
                }
            };

            println!("Imported {} messages from {} export.", count, detected);
            let convs = vault.conversation_count()?;
            let msgs = vault.message_count()?;
            println!("Vault now has {} conversations, {} messages.", convs, msgs);
        }

        Commands::Search { query } => {
            let results = vault.search(&query)?;
            if results.is_empty() {
                println!("No results found for '{}'.", query);
            } else {
                println!("Found {} result(s):\n", results.len());
                for r in results {
                    let preview: String = r.content.chars().take(200).collect();
                    println!(
                        "[{}] {} ({}):\n  {}\n",
                        r.provider,
                        r.title,
                        r.role,
                        preview
                    );
                }
            }
        }

        Commands::List => {
            let convs = vault.list_conversations()?;
            if convs.is_empty() {
                println!("No conversations in vault. Use 'import' to add some.");
            } else {
                for c in convs {
                    let tags = vault.list_tags(&c.id).unwrap_or_default();
                    let tag_str = if tags.is_empty() {
                        String::new()
                    } else {
                        format!(" [{}]", tags.join(", "))
                    };
                    println!(
                        "[{}] {} ({}){}\n  {}",
                        c.provider,
                        c.title,
                        c.id,
                        tag_str,
                        c.updated_at
                    );
                }
            }
        }

        Commands::Tag { conversation_id, tag } => {
            vault.add_tag(&conversation_id, &tag)?;
            println!("Tagged conversation {} with '{}'.", conversation_id, tag);
        }

        Commands::Tags { conversation_id } => {
            if let Some(id) = conversation_id {
                let tags = vault.list_tags(&id)?;
                if tags.is_empty() {
                    println!("No tags for conversation {}.", id);
                } else {
                    println!("Tags: {}", tags.join(", "));
                }
            } else {
                let tags = vault.all_tags()?;
                if tags.is_empty() {
                    println!("No tags in vault.");
                } else {
                    println!("All tags: {}", tags.join(", "));
                }
            }
        }

        Commands::Stats => {
            let convs = vault.conversation_count()?;
            let msgs = vault.message_count()?;
            let tags = vault.all_tags()?;
            println!("Vault: {}", vault_path.display());
            println!("Conversations: {}", convs);
            println!("Messages: {}", msgs);
            println!("Unique tags: {}", tags.len());
        }
    }

    Ok(())
}
