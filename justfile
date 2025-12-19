set shell := ["bash", "-euc"]

check:
    cargo fmt --check --all
    cargo clippy --bins --all-features -- -Dwarnings

check-strict:
    cargo clippy --bins --all-features -- -D clippy::pedantic -D clippy::nursery
