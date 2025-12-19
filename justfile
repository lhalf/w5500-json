set shell := ["bash", "-euc"]

check:
    cargo fmt --check --all
    cargo clippy --bins --all-features -- -Dwarnings

test:
    cargo test --lib --target x86_64-unknown-linux-gnu

check-strict:
    cargo clippy --bins --all-features -- -D clippy::pedantic -D clippy::nursery
