_list:
    @just --list

# Format project.
[group("lint")]
fmt:
    cargo +nightly fmt
    fd --type=file --hidden --extension=yml --extension=md --extension=js --exec-batch npx -y prettier --write

# Run Clippy over workspace.
[group("lint")]
clippy:
    cargo clippy --workspace --all-targets --all-features
