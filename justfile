# The vote module lives in js/board; the Rust crates ship copies for their asset! macros.
sync-js:
    cp js/board/seggwat-board.js rust/seggwat-topcoat/assets/
    cp js/board/seggwat-board.js rust/seggwat-dioxus/assets/

check:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo clippy -p seggwat --all-targets -- -D warnings
    cargo test --workspace --all-features
    cargo check -p seggwat-dioxus --target wasm32-unknown-unknown
