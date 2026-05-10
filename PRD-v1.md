---
repo: claude-vault
rank: 1
score: 0.86
sprint: 2
substrate_anchor: Claude
build_estimate: "4–6 weeks for v0.1"
status: planned
---

# PRD v1.0 — claude-vault

> **One-liner:** Local-first, vendor-agnostic vault for all your AI conversations.
>
> **Substrate:** Claude users (densest substrate now); supports any provider via import
> **Launch channels:** r/ClaudeAI, Anthropic Discord, AI Twitter (Claude tag), HN
> **Build estimate (v0.1):** 4–6 weeks for v0.1

---

## What problem does this solve

AI conversations are scattered. Claude.ai keeps yours. ChatGPT keeps yours. Gemini keeps yours. Each platform owns the search, the export, and the destruction policy. There is no place where you can search across all of them, tag the useful exchanges, extract the prompts that worked, or keep a record after a vendor decides to wipe history. claude-vault is that place — local-first, sovereign, never asks you to log in.

## Why this is a shovel and not a product

Closes a real, evidenced gap (every major AI subreddit has variants of 'how do I export my chats'). Sovereign by construction. Buildable in weeks, not months. Has clear v0.1 → v1.0 evolution into a second-brain layer.

---

## v0.1 — what ships

Imports Claude.ai exports + GPT exports + a local SQLite vault with FTS5 full-text search. CLI + minimal Tauri UI.

### Acceptance criteria for v0.1

A v0.1 release is publishable to GitHub when ALL of these are true:

- [ ] Core functionality described above works on the primary developer machine.
- [ ] At least one local-only configuration is documented and tested (no cloud required).
- [ ] BYO endpoint / BYO key configuration is documented.
- [ ] README explains: what it is, who it's for, how to install, how to configure, what it doesn't do.
- [ ] LICENSE present (Apache 2.0 unless overridden).
- [ ] No hardcoded keys or vendor URLs anywhere.
- [ ] No telemetry / phone-home.
- [ ] At least one passing test for the main code path.
- [ ] CI green.
- [ ] AGENTS.md compliance reviewed.

## v0.5 — first major evolution

Cross-provider conversation diff. Tagging. Prompt-extraction tooling. Export to markdown/Obsidian.

## v1.0 — fuller scope

Full second-brain integration with KB tools (Obsidian, Logseq). Smart clustering of related conversations.

---

## Architecture sketch

### Stack

Tauri (Rust + TS) for the desktop shell, SQLite + FTS5 for storage, per-provider import adapters in TS.

### Provider abstraction

The shovel MUST expose a provider abstraction even if v0.1 only uses one
provider. Suggested shape:

```
interface Provider {
  name: string;
  endpoint: URL;
  apiKeyEnvVar: string;
  call(input: ProviderInput): Promise<ProviderOutput>;
}
```

The default config in v0.1 must point to a free, local provider where
applicable, and document how to swap in any other.

### Configuration

Configuration order of precedence (highest to lowest):

1. Command-line flags
2. Environment variables (prefix: `CLAUDE_VAULT_*`)
3. User config file (`~/.config/claude-vault/config.toml` on Linux/Mac, equivalent on Windows)
4. Default config (shipped, but never with secrets)

---

## Anti-scope (do NOT build)

No real-time cloud sync. No proprietary export format. No account walls. No auto-summarization in v0.1 (do that in v0.5).

---

## Tombstone risk and mitigation

**Risk:** Anthropic shipping native vault with cross-provider import. Probability low — they have basic export but not unified search; not their priority.

**Mitigation:** Ship fast (v0.1 in 4–6 weeks for v0.1). Build community early
(launch on r/ClaudeAI, Anthropic Discord, AI Twitter (Claude tag), HN). Even if upstream absorbs the feature, accumulated
stars and the community are the audience-build payoff.

**Kill signal:** Someone shipping a polished cross-provider vault first. Ship early.

If the kill signal triggers, the maintainer must announce within one week and
either (a) refocus on a remaining gap, (b) merge gracefully into upstream if
they're receptive, or (c) mark the repo as archived with a clear pointer to the
replacement.

---

## Launch plan

### Pre-launch checklist

- [ ] Repo on GitHub at `github.com/sovereign-shovels/claude-vault`
- [ ] README polished (see template in `_templates/`)
- [ ] At least 3 issues / discussions seeded (real ones, not placeholder)
- [ ] LICENSE, CODE_OF_CONDUCT, CONTRIBUTING present
- [ ] Demo asset (gif, screenshot, or short video — depending on category)
- [ ] First-launch post drafted for primary launch channel

### Day-1 launch

Post to: r/ClaudeAI, Anthropic Discord, AI Twitter (Claude tag), HN

Subject template (adjust per channel):
- Show HN: `Show HN: claude-vault – Local-first, vendor-agnostic vault for all your AI conversations.`
- Reddit: `[OSS] Local-first, vendor-agnostic vault for all your AI conversations.` with full post explaining the gap and the build
- Twitter/X: thread leading with the demo gif

### Week-1 follow-up

- Respond to every issue and comment within 24h.
- Ship at least one bugfix release based on launch feedback.
- Cross-post to secondary channels.

### Month-1 review

- Assess star velocity and community formation.
- If kill signal triggered, follow tombstone protocol above.
- If trajectory is healthy, plan v0.5.

---

## Cross-references

- Constitution: [[AGENTS]]
- Public README: [[README]]
- Progress frontmatter: [[progress]]
- Internal knowledge graph: [[knowledge-graph]]
