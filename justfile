_list:
    @just --list

# Format project.
[group("lint")]
fmt:
    cargo +nightly fmt
    fd --type=file --hidden --extension=yml --extension=md --extension=js --exec-batch npx -y prettier --write

# Check project.
[group("lint")]
check: clippy
    fd --type=file --hidden --extension=yml --extension=md --extension=js --exec-batch npx -y prettier --check

# Run Clippy over workspace.
[group("lint")]
clippy:
    cargo clippy --workspace --all-targets --all-features
