# claude-vault

> Local-first, vendor-agnostic vault for all your AI conversations.

**Status:** v0.1 — planning. Not yet released.

**Sovereignty:** sovereign-by-construction. BYO endpoint, BYO key, BYO model.
A local-only configuration is documented and tested.

This is a community project, **not affiliated with Claude**.
Best-effort community shovel — no SLA, no roadmap commitments.

---

## What this is

Local-first, vendor-agnostic vault for all your AI conversations.

## What this isn't

Not real-time cloud sync. Not a proprietary export format. Not an account-walled service. Not (in v0.1) a summarization or chat interface — just import, store, search.

## Install

> Coming with v0.1 release.

## Configure

You bring the model. By default `claude-vault` tries to use a local provider:

- For LLM endpoints: Ollama at `http://localhost:11434`
- For voice endpoints: configurable, see [docs/configure.md]

To use any other provider (Claude, GPT, Hermes, OpenRouter, Sarvam, etc.):

```toml
# ~/.config/claude-vault/config.toml
[provider]
endpoint = "https://api.your-provider.com/v1"
api_key_env = "YOUR_PROVIDER_KEY"
model = "your-model-name"
```

Anthropic, OpenAI, and Sarvam endpoints all work. Local Ollama, llama.cpp,
LM Studio, and vLLM all work via their OpenAI-compatible endpoints.

## Why this exists

AI conversations are scattered. Claude.ai keeps yours. ChatGPT keeps yours. Gemini keeps yours. Each platform owns the search, the export, and the destruction policy. There is no place where you can search across all of them, tag the useful exchanges, extract the prompts that worked, or keep a record after a vendor decides to wipe history. claude-vault is that place — local-first, sovereign, never asks you to log in.

## What's next

See [PRD-v1.md](./PRD-v1.md) for the full v0.1 → v0.5 → v1.0 plan.

## License

Apache 2.0. See [LICENSE](./LICENSE).

## Part of sovereign-shovels

This repo is part of the [sovereign-shovels](https://github.com/sovereign-shovels)
portfolio of small, focused, sovereign-by-construction AI utilities.

Other shovels: claude-vault, bulbul-studio, saaras-tray, claude-prompts,
ollama-cron, mcp-forge, sarvam-pdf, agent-console, sarvam-meet, obsidian-llm,
llm-diff, claude-bridge, claude-radio, sarvam-cast.
