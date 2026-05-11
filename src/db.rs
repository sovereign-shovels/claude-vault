use anyhow::Result;
use rusqlite::{params, Connection};
use std::path::Path;

pub struct Vault {
    conn: Connection,
}

#[derive(Debug)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub provider: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug)]
pub struct Message {
    #[allow(dead_code)]
    pub id: i64,
    pub conversation_id: String,
    pub role: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug)]
pub struct SearchResult {
    #[allow(dead_code)]
    pub conversation_id: String,
    pub title: String,
    pub role: String,
    pub content: String,
    pub provider: String,
}

impl Vault {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        let vault = Self { conn };
        vault.init()?;
        Ok(vault)
    }

    fn init(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS conversations (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                provider TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                conversation_id TEXT NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (conversation_id) REFERENCES conversations(id)
            );

            CREATE VIRTUAL TABLE IF NOT EXISTS messages_fts USING fts5(
                content,
                conversation_id UNINDEXED,
                content_rowid=rowid,
                prefix=2
            );

            CREATE TABLE IF NOT EXISTS tags (
                conversation_id TEXT NOT NULL,
                tag TEXT NOT NULL,
                PRIMARY KEY (conversation_id, tag),
                FOREIGN KEY (conversation_id) REFERENCES conversations(id)
            );

            CREATE TRIGGER IF NOT EXISTS messages_ai AFTER INSERT ON messages BEGIN
                INSERT INTO messages_fts(rowid, content, conversation_id)
                VALUES (new.id, new.content, new.conversation_id);
            END;

            CREATE TRIGGER IF NOT EXISTS messages_ad AFTER DELETE ON messages BEGIN
                INSERT INTO messages_fts(messages_fts, rowid, content, conversation_id)
                VALUES ('delete', old.id, old.content, old.conversation_id);
            END;
            "
        )?;
        Ok(())
    }

    pub fn insert_conversation(&self, conv: &Conversation) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO conversations (id, title, provider, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![&conv.id, &conv.title, &conv.provider, &conv.created_at, &conv.updated_at],
        )?;
        Ok(())
    }

    pub fn insert_message(&self, msg: &Message) -> Result<()> {
        self.conn.execute(
            "INSERT INTO messages (conversation_id, role, content, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![&msg.conversation_id, &msg.role, &msg.content, &msg.created_at],
        )?;
        Ok(())
    }

    pub fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        let sql = r#"
            SELECT
                c.id,
                c.title,
                m.role,
                m.content,
                c.provider
            FROM messages_fts fts
            JOIN messages m ON m.id = fts.rowid
            JOIN conversations c ON c.id = m.conversation_id
            WHERE messages_fts MATCH ?1
            ORDER BY rank
            LIMIT 50
        "#;

        let mut stmt = self.conn.prepare(sql)?;
        let results = stmt.query_map([query], |row| {
            Ok(SearchResult {
                conversation_id: row.get(0)?,
                title: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                provider: row.get(4)?,
            })
        })?;

        results.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn list_conversations(&self) -> Result<Vec<Conversation>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, provider, created_at, updated_at
             FROM conversations
             ORDER BY updated_at DESC"
        )?;
        let results = stmt.query_map([], |row| {
            Ok(Conversation {
                id: row.get(0)?,
                title: row.get(1)?,
                provider: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;
        results.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn add_tag(&self, conversation_id: &str, tag: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO tags (conversation_id, tag) VALUES (?1, ?2)",
            params![conversation_id, tag],
        )?;
        Ok(())
    }

    pub fn list_tags(&self, conversation_id: &str) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT tag FROM tags WHERE conversation_id = ?1 ORDER BY tag"
        )?;
        let results = stmt.query_map([conversation_id], |row| row.get(0))?;
        results.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn all_tags(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT tag FROM tags ORDER BY tag"
        )?;
        let results = stmt.query_map([], |row| row.get(0))?;
        results.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn conversation_count(&self) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM conversations", [], |row| row.get(0)
        )?;
        Ok(count)
    }

    pub fn message_count(&self) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM messages", [], |row| row.get(0)
        )?;
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_vault() -> Vault {
        let tmp = std::env::temp_dir().join(format!("claude-vault-test-{}.db", std::process::id()));
        let _ = std::fs::remove_file(&tmp);
        Vault::open(&tmp).unwrap()
    }

    #[test]
    fn test_insert_and_search() {
        let vault = test_vault();

        vault.insert_conversation(&Conversation {
            id: "conv-1".into(),
            title: "Test Chat".into(),
            provider: "claude".into(),
            created_at: "2024-01-01T00:00:00Z".into(),
            updated_at: "2024-01-01T00:00:00Z".into(),
        }).unwrap();

        vault.insert_message(&Message {
            id: 0,
            conversation_id: "conv-1".into(),
            role: "user".into(),
            content: "How do I write a Rust CLI?".into(),
            created_at: "2024-01-01T00:00:00Z".into(),
        }).unwrap();

        vault.insert_message(&Message {
            id: 0,
            conversation_id: "conv-1".into(),
            role: "assistant".into(),
            content: "Use clap for argument parsing.".into(),
            created_at: "2024-01-01T00:01:00Z".into(),
        }).unwrap();

        let results = vault.search("clap").unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].content.contains("clap"));

        let results = vault.search("Rust CLI").unwrap();
        assert_eq!(results.len(), 1);

        let convs = vault.list_conversations().unwrap();
        assert_eq!(convs.len(), 1);
        assert_eq!(convs[0].title, "Test Chat");

        assert_eq!(vault.conversation_count().unwrap(), 1);
        assert_eq!(vault.message_count().unwrap(), 2);
    }

    #[test]
    fn test_tags() {
        let vault = test_vault();

        vault.insert_conversation(&Conversation {
            id: "conv-1".into(),
            title: "Test".into(),
            provider: "claude".into(),
            created_at: "2024-01-01T00:00:00Z".into(),
            updated_at: "2024-01-01T00:00:00Z".into(),
        }).unwrap();

        vault.add_tag("conv-1", "rust").unwrap();
        vault.add_tag("conv-1", "cli").unwrap();

        let tags = vault.list_tags("conv-1").unwrap();
        assert_eq!(tags, vec!["cli", "rust"]);

        let all = vault.all_tags().unwrap();
        assert_eq!(all, vec!["cli", "rust"]);
    }
}
