_list:
    @just --list

# Format project.
[group("lint")]
fmt:
    fd --type=file --hidden --extension=yml --extension=md --extension=js --exec-batch prettier --write
    fd --hidden --extension=toml --exec-batch taplo format
    cargo +nightly fmt

# Check project.
[group("lint")]
check: && clippy
    fd --type=file --hidden --extension=yml --extension=md --extension=js --exec-batch prettier --check
    fd --hidden --extension=toml --exec-batch taplo format --check
    fd --hidden --extension=toml --exec-batch taplo lint

# Run Clippy over workspace.
[group("lint")]
clippy:
    cargo clippy --workspace --all-targets --all-features
