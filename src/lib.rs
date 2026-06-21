//! pheno-agents-md core lib.
//!
//! Generates an `AGENTS.md` file (the "agent constitution") for a repo, by
//! rendering the canonical template (§77.1 of `FLEET_100TASK_DAG_V4.md`) with
//! optional overrides from a `pheno-agents-md.yaml` config.
//!
//! See: <https://agents.md> for the spec.

#![warn(missing_docs)]

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AgentConfig {
    /// Build command (e.g. `just build`, `cargo build`).
    pub build: Option<String>,
    /// Test command.
    pub test: Option<String>,
    /// Lint command.
    pub lint: Option<String>,
    /// Audit command (cargo-deny / npm audit / pip-audit).
    pub audit: Option<String>,
    /// Sign command (cosign / sigstore).
    pub sign: Option<String>,
    /// Repo name. Defaults to the directory name.
    pub repo_name: Option<String>,
    /// Extra do-not-touch zones beyond the defaults.
    pub extra_dont_touch: Vec<String>,
}

const TEMPLATE: &str = r#"# {repo_name} — AGENTS.md (Agent Constitution)

## Build & test
- Build:  `{build}`
- Test:   `{test}`
- Lint:   `{lint}`
- Audit:  `{audit}`
- Sign:   `{sign}`

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
{extra_dont_touch_list}

## Ownership
- See `CODEOWNERS` (GitHub) — agents should not self-approve PRs
- Last 5 contributors: `git shortlog -sn | head -5`

## References
- DAG: `FLEET_100TASK_DAG_V4.md` (or V5/V6/... — pick the current one)
- Worklog schema: `pheno-worklog-schema` (lib)
- llms.txt: see `pheno-llms-txt`
"#;

/// Render the AGENTS.md content for the given config.
pub fn render(config: &AgentConfig, default_repo_name: &str) -> String {
    let repo_name = config
        .repo_name
        .clone()
        .unwrap_or_else(|| default_repo_name.to_string());
    let build = config.build.clone().unwrap_or_else(|| "cargo build".into());
    let test = config.test.clone().unwrap_or_else(|| "cargo test".into());
    let lint = config.lint.clone().unwrap_or_else(|| "cargo clippy -- -D warnings".into());
    let audit = config.audit.clone().unwrap_or_else(|| "cargo deny check".into());
    let sign = config.sign.clone().unwrap_or_else(|| "cosign sign".into());
    let extra = if config.extra_dont_touch.is_empty() {
        String::new()
    } else {
        let list = config
            .extra_dont_touch
            .iter()
            .map(|z| format!("- `{z}` (custom)"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("\nCustom zones:\n{list}")
    };

    TEMPLATE
        .replace("{repo_name}", &repo_name)
        .replace("{build}", &build)
        .replace("{test}", &test)
        .replace("{lint}", &lint)
        .replace("{audit}", &audit)
        .replace("{sign}", &sign)
        .replace("{extra_dont_touch_list}", &extra)
}

/// Load a `pheno-agents-md.yaml` from disk (or return defaults if missing).
pub fn load_config(path: &Path) -> Result<AgentConfig> {
    if !path.exists() {
        return Ok(AgentConfig::default());
    }
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("read config at {}", path.display()))?;
    let cfg: AgentConfig = serde_yaml::from_str(&raw)
        .with_context(|| format!("parse YAML at {}", path.display()))?;
    Ok(cfg)
}

/// Render and write AGENTS.md to `dest`.
pub fn write_agents_md(config: &AgentConfig, repo_name: &str, dest: &Path) -> Result<()> {
    let rendered = render(config, repo_name);
    std::fs::write(dest, rendered)
        .with_context(|| format!("write AGENTS.md to {}", dest.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_with_defaults() {
        let cfg = AgentConfig::default();
        let s = render(&cfg, "myrepo");
        assert!(s.contains("# myrepo"));
        assert!(s.contains("Build:  `cargo build`"));
        assert!(s.contains("Conventional Commits"));
    }

    #[test]
    fn renders_with_overrides() {
        let cfg = AgentConfig {
            build: Some("just build".into()),
            test: Some("just test".into()),
            repo_name: Some("thegent".into()),
            extra_dont_touch: vec!["apps/byteport/backend/api/.archive/".into()],
            ..Default::default()
        };
        let s = render(&cfg, "fallback");
        assert!(s.contains("# thegent"));
        assert!(s.contains("Build:  `just build`"));
        assert!(s.contains("apps/byteport/backend/api/.archive/"));
    }

    #[test]
    fn empty_config_renders() {
        let s = render(&AgentConfig::default(), "x");
        assert!(s.contains("# x"));
        assert!(!s.contains("Custom zones:"));
    }

    // --- V20 SOTA: YAML config edge case tests ----------------------------

    #[test]
    fn load_config_empty_yaml_returns_defaults() {
        // Edge case 1: empty file -> all defaults
        let tmp = tempfile::tempdir().unwrap();
        let cfg_path = tmp.path().join("pheno-agents-md.yaml");
        std::fs::write(&cfg_path, "").unwrap();
        let cfg = load_config(&cfg_path).unwrap();
        assert!(cfg.build.is_none());
        assert!(cfg.test.is_none());
        assert!(cfg.repo_name.is_none());
        assert!(cfg.extra_dont_touch.is_empty());
        // Render still works with all defaults
        let s = render(&cfg, "empty-yaml");
        assert!(s.contains("# empty-yaml"));
        assert!(s.contains("Build:  `cargo build`"));
    }

    #[test]
    fn load_config_minimal_yaml_missing_fields() {
        // Edge case 2: YAML with only some fields -> missing ones use defaults
        let tmp = tempfile::tempdir().unwrap();
        let cfg_path = tmp.path().join("pheno-agents-md.yaml");
        std::fs::write(&cfg_path, "repo_name: minimal\n").unwrap();
        let cfg = load_config(&cfg_path).unwrap();
        assert_eq!(cfg.repo_name.as_deref(), Some("minimal"));
        // build, test, lint etc. are all None -> render falls back to defaults
        assert!(cfg.build.is_none());
        assert!(cfg.test.is_none());
        assert!(cfg.lint.is_none());
        assert!(cfg.extra_dont_touch.is_empty());
        let s = render(&cfg, "fallback");
        assert!(s.contains("# minimal"));
        // Default commands appear (from render fallbacks)
        assert!(s.contains("Test:   `cargo test`"));
        assert!(s.contains("Lint:   `cargo clippy -- -D warnings`"));
    }

    #[test]
    fn load_config_yaml_with_unicode() {
        // Edge case 3: non-ASCII (CJK + emoji) round-trips intact
        let tmp = tempfile::tempdir().unwrap();
        let cfg_path = tmp.path().join("pheno-agents-md.yaml");
        let yaml = "repo_name: \"リポ_名_🚀\"\nbuild: \"cargo 构建 --release 🚀\"\n";
        std::fs::write(&cfg_path, yaml).unwrap();
        let cfg = load_config(&cfg_path).unwrap();
        assert_eq!(cfg.repo_name.as_deref(), Some("リポ_名_🚀"));
        assert_eq!(cfg.build.as_deref(), Some("cargo 构建 --release 🚀"));
        let s = render(&cfg, "ignored");
        assert!(s.contains("リポ_名_🚀"));
        assert!(s.contains("cargo 构建 --release 🚀"));
    }

    #[test]
    fn load_config_yaml_with_very_long_strings() {
        // Edge case 4: a 50,000-character build command must not break parsing
        let tmp = tempfile::tempdir().unwrap();
        let cfg_path = tmp.path().join("pheno-agents-md.yaml");
        let huge = "x".repeat(50_000);
        let yaml = format!("build: \"{huge}\"\ntest: \"{huge}\"\n");
        std::fs::write(&cfg_path, yaml).unwrap();
        let cfg = load_config(&cfg_path).unwrap();
        assert_eq!(cfg.build.as_ref().map(String::len), Some(50_000));
        assert_eq!(cfg.test.as_ref().map(String::len), Some(50_000));
        let s = render(&cfg, "long");
        // The huge commands appear in the rendered output, and the header is intact
        assert!(s.contains("# long"));
        assert!(s.contains(&"x".repeat(1000)));
    }

    #[test]
    fn load_config_yaml_with_nested_extra_dont_touch() {
        // Edge case 5: a list of many `extra_dont_touch` entries renders all zones
        let tmp = tempfile::tempdir().unwrap();
        let cfg_path = tmp.path().join("pheno-agents-md.yaml");
        let yaml = "\
repo_name: nested
extra_dont_touch:
  - apps/byteport/backend/.archive/**
  - apps/byteport/frontend/dist/**
  - vendor/**
  - .secrets/**
  - \"path with spaces/中文/**
\"
";
        std::fs::write(&cfg_path, yaml).unwrap();
        let cfg = load_config(&cfg_path).unwrap();
        assert_eq!(cfg.extra_dont_touch.len(), 5);
        assert!(cfg.extra_dont_touch.contains(&"vendor/**".to_string()));
        assert!(cfg.extra_dont_touch.iter().any(|z| z.contains("中文")));
        let s = render(&cfg, "ignored");
        // All 5 custom zones appear in the rendered output under "Custom zones:"
        assert!(s.contains("Custom zones:"));
        for zone in &cfg.extra_dont_touch {
            assert!(
                s.contains(zone),
                "rendered output missing extra_dont_touch zone {zone:?}"
            );
        }
    }
}
