# claude-vault

> Local-first, vendor-agnostic vault for all your AI conversations.

**Status:** v0.1 — ready to use.

**Sovereignty:** sovereign-by-construction. No cloud, no login, no telemetry.

This is a community project, **not affiliated with Anthropic**.
Best-effort community shovel — no SLA, no roadmap commitments.

---

## Architecture

```
┌─────────────────┐     ┌──────────────┐     ┌─────────────────┐
│  Claude export  │────▶│              │     │   FTS5 index    │
│   (JSON/ZIP)    │     │   claude-    │────▶│  (full-text     │
├─────────────────┤     │   vault      │     │    search)      │
│  ChatGPT export │────▶│  (SQLite)    │     ├─────────────────┤
│   (JSON/ZIP)    │     │              │     │    Tags table   │
└─────────────────┘     └──────────────┘     └─────────────────┘
                                │
                                ▼
                        ┌──────────────┐
                        │  CLI query   │
                        │ search/list/ │
                        │  tag/stats   │
                        └──────────────┘
```

## What this is

AI conversations are scattered. Claude.ai keeps yours. ChatGPT keeps yours. Gemini keeps yours. Each platform owns the search, the export, and the destruction policy.

claude-vault is a local SQLite vault with FTS5 full-text search. Import your exports, search across all of them, tag the useful ones. Your data, on your machine, forever.

## What this isn't

- No real-time cloud sync
- No proprietary export format
- No auto-summarization in v0.1 (v0.5)
- Not a chat client — it's a vault for conversations you've already had

See [PRD-v1.md](./PRD-v1.md) for the full anti-scope definition.

---

## Install

### From source

**Prerequisites:**
- [Rust](https://rustup.rs/) 1.75+

```bash
git clone https://github.com/sovereign-shovels/claude-vault.git
cd claude-vault

# Build
cargo build --release

# The binary is at target/release/claude-vault
```

---

## Usage

### Import conversations

```bash
# Claude.ai export (JSON)
claude-vault import conversations.json --provider claude

# ChatGPT export (JSON)
claude-vault import conversations.json --provider chatgpt

# Auto-detect provider
claude-vault import conversations.json
```

### Search

```bash
# Full-text search across all messages
claude-vault search "how do I use tokio"

# Results show conversation title, speaker role, and message preview
```

### List conversations

```bash
claude-vault list
```

### Tag conversations

```bash
# Tag a conversation
claude-vault tag <conversation-id> rust

# List tags for a conversation
claude-vault tags <conversation-id>

# List all tags in vault
claude-vault tags
```

### Stats

```bash
claude-vault stats
```

**Demo output:**
```
$ claude-vault import test-claude-export.json --provider claude
Imported 2 messages from claude export.
Vault now has 1 conversations, 2 messages.

$ claude-vault search "test message"
Found 1 result(s):

[claude] Test Conversation (user):
  Hello, this is a test message.

$ claude-vault stats
Vault: ~/Library/Application Support/claude-vault/vault.db
Conversations: 1
Messages: 2
Unique tags: 0
```

---

## Configure

```toml
# ~/.config/claude-vault/config.toml
[vault]
vault_path = "/path/to/your/vault.db"
```

Or via environment variable:

```bash
export CLAUDE_VAULT_PATH="/path/to/your/vault.db"
```

Default vault location:
- macOS: `~/Library/Application Support/claude-vault/vault.db`
- Linux: `~/.local/share/claude-vault/vault.db`
- Windows: `%APPDATA%\claude-vault\vault.db`

---

## Exporting from providers

### Claude.ai
1. Go to claude.ai → Settings → Account → Export Data
2. Download the ZIP, extract the JSON
3. `claude-vault import claude_export.json`

### ChatGPT
1. Go to chat.openai.com → Settings → Data controls → Export
2. Download the ZIP, extract `conversations.json`
3. `claude-vault import conversations.json --provider chatgpt`

---

## Why this exists

Every major AI subreddit has variants of "how do I export my chats?" There is no cross-provider vault. Until now.

See [PRD-v1.md](./PRD-v1.md) for the full problem statement and rationale.

## What's next

- **v0.5:** Cross-provider conversation diff, tagging UI, prompt extraction, export to markdown/Obsidian
- **v1.0:** Full second-brain integration with Obsidian/Logseq, smart clustering

See [PRD-v1.md](./PRD-v1.md) for the full roadmap.

---

## License

Apache 2.0. See [LICENSE](./LICENSE).

## Part of sovereign-shovels

This repo is part of the [sovereign-shovels](https://github.com/sovereign-shovels) portfolio of small, focused, sovereign-by-construction AI utilities.

Other shovels: claude-vault, bulbul-studio, saaras-tray, claude-prompts, ollama-cron, mcp-forge, sarvam-pdf, agent-console, sarvam-meet, obsidian-llm, llm-diff, claude-bridge, claude-radio, sarvam-cast.
