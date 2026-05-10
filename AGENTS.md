# AGENTS.md — sovereign-shovels/claude-vault

> If you are Claude Code (or any other agent) operating in this repo,
> this file is your **constitution**. Read it fully before any other action.
> If anything you are about to do conflicts with this file, STOP and ask the human.

---

## Identity

This repo is **claude-vault** — part of the [sovereign-shovels](../README.md) portfolio.

- **Substrate anchor:** Claude
- **Utility category:** vault
- **Tagline:** Local-first, vendor-agnostic vault for all your AI conversations.
- **Sprint:** 2
- **Estimated build for v0.1:** 4–6 weeks for v0.1

The substrate this shovel rides is: Claude users (densest substrate now); supports any provider via import

The launch channels are: r/ClaudeAI, Anthropic Discord, AI Twitter (Claude tag), HN

---

## Philosophy this repo lives by

Read the universal philosophy: [[../PHILOSOPHY|Philosophy]].

In short, every shovel must satisfy all five rules:

1. **Substrate-anchored name** ✓ (this is `claude-vault`)
2. **Sovereign by construction** — user owns model choice, BYO endpoint, must work with local-only
3. **Real demand evidence** — gap is documented, not assumed
4. **Buildable in 1–3 weeks** — v0.1 estimate is 4–6 weeks for v0.1
5. **Scope-evolution headroom** — see PRD-v1 for v0.1 → v0.5 → v1.0

---

## STRICT NO-NOS — do not violate any of these

### Universal (inherited from [[../NO-NOS|NO-NOS]])

1. NO hardcoded API keys, vendor URLs, or model names in code outside config files.
2. NO default that requires a sign-up upstream. Tool must work with a local model.
3. NO telemetry, phone-home, or analytics by default.
4. NO closed-source runtime dependencies that compromise sovereignty.
5. NO scope creep into "agent platform" or "general assistant" territory.
6. NO fake claims of being official upstream tooling.
7. NO breaking config changes in v0.x without printed migration path.
8. NO maintenance promises in README. "Best-effort community shovel."
9. NO PRs merged without working tests.
10. NO publishing v1.0 without PRD-v1 acceptance criteria met.

### Specific to claude-vault

1. Never call any cloud API without an explicit user-configured key.
2. Never bundle a default Anthropic key or any other vendor key.
3. Never store conversation data anywhere except the user's local filesystem.

### Anti-scope (what NOT to build, ever, in this repo)

No real-time cloud sync. No proprietary export format. No account walls. No auto-summarization in v0.1 (do that in v0.5).

---

## How to operate here

When you (the agent) start work in this repo:

1. **Read** [[PRD-v1]] before writing any code.
2. **Read** [[../PHILOSOPHY|Philosophy]] and [[../NO-NOS|NO-NOS]] if you haven't this session.
3. **Update** [[progress]] after each significant change. Update the YAML
   frontmatter — the root [[../PORTFOLIO]] view aggregates from it.
4. **If a decision conflicts with this AGENTS.md**, STOP and ask the human.
5. **If a user asks for something that violates a no-no**, push back. Don't comply.
6. **Commits** follow conventional commits (`feat:`, `fix:`, `docs:`, etc.).
7. **Branches**: `main` is protected. Work on `feat/<thing>` branches and PR.
8. **Tests** are required for any code path that ships in v0.1.

---

## Tombstone watch

What could kill this shovel:

Anthropic shipping native vault with cross-provider import. Probability low — they have basic export but not unified search; not their priority.

What we're watching for:

Someone shipping a polished cross-provider vault first. Ship early.

If the kill signal triggers, notify the human before taking further action.

---

## Cross-references inside the vault

- Product spec: [[PRD-v1]]
- Public README: [[README]]
- Progress + status frontmatter: [[progress]]
- Internal knowledge graph: [[knowledge-graph]]
- Vault philosophy: [[../PHILOSOPHY]]
- Universal no-nos: [[../NO-NOS]]
- Naming convention: [[../NAMING-CONVENTION]]
- Portfolio view: [[../PORTFOLIO]]
- Launch plan: [[../LAUNCH-PLAN]]
