_list:
    @just --list

# Format project.
[group("lint")]
fmt:
    fd --type=file --hidden --extension=yml --extension=md --extension=js --exec-batch prettier --write
    fd --hidden --extension=toml --exec-batch taplo format
    cargo +nightly fmt

# Check Rust formatting.
[group("lint")]
fmt-check:
    cargo fmt --all -- --check

# Check project.
[group("lint")]
check: && clippy
    fd --type=file --hidden --extension=yml --extension=md --extension=js --exec-batch prettier --check
    fd --hidden --extension=toml --exec-batch taplo format --check
    fd --hidden --extension=toml --exec-batch taplo lint

# Run Clippy over workspace.
[group("lint")]
clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Initialize the test databases.
init-db:
    nix run .#init-db

# Start Redis for integration tests.
start-redis:
    #!/usr/bin/env bash
    set -euo pipefail

    if ! redis-cli ping; then
        redis-server --daemonize yes
    fi
    redis-cli ping

# Check that all workspace targets compile.
cargo-check:
    cargo check --workspace --bins --examples --tests

# Test the workspace except for the Diesel example.
test:
    cargo test --workspace --all-features --no-fail-fast --exclude=db-diesel -- --nocapture

# Test the Diesel example.
test-diesel:
    #!/usr/bin/env bash
    set -euo pipefail

    cd databases/diesel
    DATABASE_URL=test.db cargo test -p=db-diesel --no-fail-fast -- --nocapture

# Run the build and test sequence locally.
ci:
    just init-db
    just start-redis
    just cargo-check
    just test
    just test-diesel

# Clean Cargo artifacts before uploading the CI cache.
ci-cache-clean:
    cargo-ci-cache-clean
