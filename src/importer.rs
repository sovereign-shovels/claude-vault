use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::db::{Conversation, Message, Vault};

#[derive(Debug, Deserialize)]
struct ClaudeExport {
    uuid: String,
    name: String,
    #[serde(default)]
    chat_messages: Vec<ClaudeMessage>,
    #[serde(default)]
    created_at: String,
    #[serde(default)]
    updated_at: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeMessage {
    #[allow(dead_code)]
    uuid: String,
    #[serde(default)]
    sender: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct ChatGptExport {
    #[serde(default)]
    title: String,
    #[serde(default)]
    id: String,
    #[serde(default)]
    mapping: HashMap<String, ChatGptNode>,
    #[serde(default)]
    create_time: Option<f64>,
    #[serde(default)]
    update_time: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct ChatGptNode {
    message: Option<ChatGptMessage>,
}

#[derive(Debug, Deserialize)]
struct ChatGptMessage {
    #[serde(default)]
    author: ChatGptAuthor,
    #[serde(default)]
    content: ChatGptContent,
    #[serde(default)]
    create_time: Option<f64>,
}

#[derive(Debug, Deserialize, Default)]
struct ChatGptAuthor {
    role: String,
}

#[derive(Debug, Deserialize, Default)]
struct ChatGptContent {
    #[serde(default)]
    parts: Vec<String>,
}

pub fn import_claude(vault: &Vault, path: &Path) -> Result<usize> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;

    let exports: Vec<ClaudeExport> = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse Claude export {}", path.display()))?;

    let mut count = 0;
    for conv in exports {
        let conv_id = conv.uuid.clone();
        let created = if conv.created_at.is_empty() {
            chrono::Utc::now().to_rfc3339()
        } else {
            conv.created_at
        };
        let updated = if conv.updated_at.is_empty() {
            created.clone()
        } else {
            conv.updated_at
        };

        vault.insert_conversation(&Conversation {
            id: conv_id.clone(),
            title: if conv.name.is_empty() { "Untitled".into() } else { conv.name },
            provider: "claude".into(),
            created_at: created,
            updated_at: updated,
        })?;

        for msg in conv.chat_messages {
            vault.insert_message(&Message {
                id: 0,
                conversation_id: conv_id.clone(),
                role: map_claude_role(&msg.sender),
                content: msg.text,
                created_at: if msg.created_at.is_empty() {
                    chrono::Utc::now().to_rfc3339()
                } else {
                    msg.created_at
                },
            })?;
            count += 1;
        }
    }

    Ok(count)
}

pub fn import_chatgpt(vault: &Vault, path: &Path) -> Result<usize> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;

    let exports: Vec<ChatGptExport> = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse ChatGPT export {}", path.display()))?;

    let mut count = 0;
    for conv in exports {
        let conv_id = if conv.id.is_empty() {
            format!("chatgpt-{}", uuid::Uuid::new_v4())
        } else {
            conv.id.clone()
        };

        let created = conv.create_time.map(|t| format_timestamp(t)).unwrap_or_else(|| chrono::Utc::now().to_rfc3339());
        let updated = conv.update_time.map(|t| format_timestamp(t)).unwrap_or_else(|| created.clone());

        vault.insert_conversation(&Conversation {
            id: conv_id.clone(),
            title: if conv.title.is_empty() { "Untitled".into() } else { conv.title },
            provider: "chatgpt".into(),
            created_at: created,
            updated_at: updated,
        })?;

        let mut nodes: Vec<_> = conv.mapping.into_iter().collect();
        nodes.sort_by(|a, b| {
            let a_time = a.1.message.as_ref().and_then(|m| m.create_time).unwrap_or(0.0);
            let b_time = b.1.message.as_ref().and_then(|m| m.create_time).unwrap_or(0.0);
            a_time.partial_cmp(&b_time).unwrap_or(std::cmp::Ordering::Equal)
        });

        for (_, node) in nodes {
            if let Some(msg) = node.message {
                let text = msg.content.parts.join("\n");
                if text.trim().is_empty() {
                    continue;
                }
                vault.insert_message(&Message {
                    id: 0,
                    conversation_id: conv_id.clone(),
                    role: map_chatgpt_role(&msg.author.role),
                    content: text,
                    created_at: msg.create_time.map(format_timestamp).unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
                })?;
                count += 1;
            }
        }
    }

    Ok(count)
}

fn map_claude_role(sender: &str) -> String {
    match sender.to_lowercase().as_str() {
        "human" | "user" => "user".into(),
        "assistant" => "assistant".into(),
        _ => sender.into(),
    }
}

fn map_chatgpt_role(role: &str) -> String {
    match role.to_lowercase().as_str() {
        "user" => "user".into(),
        "assistant" => "assistant".into(),
        "system" => "system".into(),
        _ => role.into(),
    }
}

fn format_timestamp(ts: f64) -> String {
    chrono::DateTime::from_timestamp(ts as i64, 0)
        .map(|d| d.to_rfc3339())
        .unwrap_or_else(|| chrono::Utc::now().to_rfc3339())
}

pub fn detect_provider(path: &Path) -> Option<&'static str> {
    if let Ok(content) = fs::read_to_string(path) {
        if content.contains("\"chat_messages\"") {
            return Some("claude");
        }
        if content.contains("\"mapping\"") && content.contains("\"conversation_id\"") {
            return Some("chatgpt");
        }
        if content.contains("\"mapping\"") {
            return Some("chatgpt");
        }
    }
    None
}
