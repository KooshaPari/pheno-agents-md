# pheno-agents-md

> Generate [`AGENTS.md`](https://agents.md) files for Phenotype repos (the "agent constitution").

This is the canonical implementation of the **`pheno-agents-md`** AI-DD crutch
described in `FLEET_100TASK_DAG_V4.md` §70.3 + §77.1. Every focus repo in the
Phenotype fleet adopts it as part of V11 L16 AX (Agent eXperience).

## What it does

Generates an `AGENTS.md` file by rendering the canonical template (build/test/
lint/audit/sign commands, conventions, do-not-touch zones, ownership) with
optional overrides from a `pheno-agents-md.yaml` config.

## Install

```bash
cargo install pheno-agents-md
```

## Usage

```bash
# 1. Drop a config in your repo root (optional):
cat > pheno-agents-md.yaml <<'YAML'
build: just build
test:  just test
lint:  just lint
audit: just audit
sign:  just sign
repo_name: thegent
extra_dont_touch:
  - apps/byteport/backend/api/.archive/
YAML

# 2. Run:
pheno-agents-md
# → Wrote AGENTS.md

# 3. Or with all flags:
pheno-agents-md --out AGENTS.md --config pheno-agents-md.yaml --repo myrepo
```

## What gets generated

A 60–100 line `AGENTS.md` with sections:

- **Build & test** — `just build`, `just test`, `just lint`, `just audit`, `just sign`
- **Conventions** — Conventional Commits, branch naming, WORKLOG rules, PR conventions
- **Do-not-touch zones** — `<archive>/`, `<vendor>/`, `**/Cargo.lock`, `# DO NOT EDIT` files, plus your custom zones
- **Ownership** — `CODEOWNERS`, last 5 contributors
- **References** — DAG, worklog schema, llms.txt

## Reference template (excerpt)

```markdown
# <repo> — AGENTS.md (Agent Constitution)

## Build & test
- Build:  `just build`
- Test:   `just test`
- Lint:   `just lint`
- Audit:  `just audit`
- Sign:   `cosign sign`

## Conventions
- Commits: Conventional Commits (feat/fix/docs/style/refactor/perf/test/chore)
- Branch:  `<layer>/<slug>-<YYYY-MM-DD>` (e.g. `l1/l1-triage-2026-06-11`)
- WORKLOG: append 1 row to `WORKLOG.md` per V4 DAG task ID
- PRs:     reference V4 task ID in body, e.g. `Refs V4-1.2.3`

## Do-not-touch zones
- `<archive>/` (stale work, archived intentionally)
- `<vendor>/`, `<node_modules>/` (third-party)
- `**/.git`, `**/Cargo.lock` (unless explicitly updating deps)
- files marked `# DO NOT EDIT` header
```

## Spec

- The template is defined in [`src/lib.rs`](src/lib.rs) constant `TEMPLATE`.
- The config schema is `AgentConfig` in [`src/lib.rs`](src/lib.rs).
- The DAG reference is `FLEET_100TASK_DAG_V4.md` §77.1.

## Eat your own dogfood

This repo uses itself. See [`AGENTS.md`](AGENTS.md).

## License

MIT OR Apache-2.0
