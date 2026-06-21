# pheno-agents-md — AGENTS.md (Agent Constitution)

**Date:** 2026-06-18
**Status:** ACTIVE
**Substrate:** `pheno-*-lib` (ADR-023)
**MSRV:** see `Cargo.toml`

## Purpose

AI-agent context (AGENTS.md) generation rules + template for the Phenotype fleet. Renders the 60-100 line 'agent constitution' for any repo from a small YAML config.

## Public API

```rust
pheno_agents_md::AgentConfig { build, test, lint, audit, sign, repo_name, extra_dont_touch }
pheno_agents_md::render(&cfg, default_repo_name) -> String
pheno_agents_md::load_config(&Path) -> Result<AgentConfig>
pheno_agents_md::write_agents_md(&cfg, &str, &Path) -> Result<()>
```

## Build & Test

```bash
cargo build --release
cargo test --workspace --all-features
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

## Conventions

- Commits: Conventional Commits (`feat:`, `fix:`, `chore:`, `docs:`, `refactor:`, `test:`, `build:`, `ci:`)
- Branch: `<layer>/<slug>-<YYYY-MM-DD>` or `chore/<req-id>-<slug>-<date>`
- WORKLOG: append 1 row to `WORKLOG.md` per v8 DAG task ID (schema v2.1, 7 cols + `device`)
- PRs: reference task ID in body, e.g. `Refs T15.<n>` (per the T15 v8 plan tracking)
- **Substrate placement** (ADR-023): this is a `pheno-*-lib` — pure reusable Rust library, single concern, single crate.
- **Test coverage gate**: 80% line coverage (ADR-023 Rule 3.1, lib/SDK gate).
- **Quality bar**: spec, README, test matrix, OTLP observability via pheno-tracing (ADR-012), 80% coverage, CI gate.

## Do-Not-Touch Zones

- `<archive>/` (stale work, archived intentionally)
- `<vendor>/`, `<node_modules>/` (third-party)
- `**/.git`, `**/Cargo.lock` (unless explicitly updating deps)
- files marked `# DO NOT EDIT` header

## Authority

- Spec JSON: `/tmp/t15-specs/pheno-agents-md.json`
- Substrate governance: `docs/adr/2026-06-15/ADR-019-substrate-governance.md`
- WORKLOG schema: `pheno-worklog-schema` v2.1 (ADR-015 + ADR-025 + ADR-030)
- llms.txt: see `pheno-llms-txt`
