set dotenv-load := true

test:
    cargo test --all-targets --all-features

build:
    cargo build --all-targets --all-features

lint:
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings

deny:
    cargo deny check

demo:
    cargo run -- --repo demo --out /tmp/demo-AGENTS.md
    cat /tmp/demo-AGENTS.md

ci: lint build test deny

# Measure code coverage (SSOT: see grade.sh for the canonical command)
coverage:
    cargo llvm-cov --workspace --fail-under-lines 85
