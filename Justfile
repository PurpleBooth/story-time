# This help screen
show-help:
        just --list

# Test it was built ok
test:
        cargo test

# Run a smoke test and see if the app runs
smoke-test: build
        cargo run --bin $((basename "$PWD")) -- -h

# Build release version
build:
        cargo build --release

# Run
run:
        cargo run

# Lint it
lint:
        cargo +nightly fmt --all -- --check
        cargo +nightly clippy --all-features -- -D warnings -Dclippy::all -D clippy::pedantic -D clippy::cargo -A clippy::multiple-crate-versions
        cargo +nightly check
        cargo +nightly audit --ignore RUSTSEC-2020-0071

# Format what can be formatted
fmt:
        cargo +nightly fix --allow-dirty
        cargo +nightly clippy --allow-dirty --fix -Z unstable-options --all-features -- -D warnings -D clippy::all -D clippy::pedantic -D clippy::cargo -D clippy::nursery -A clippy::multiple-crate-versions
        cargo +nightly fmt --all

# Clean the build directory
clean:
        cargo clean
