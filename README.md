cargo +nightly build
RUSTFLAGS="-Z macro-backtrace" cargo +nightly build

cargo install --version=0.1.0-beta.1 sqlx-cli
cargo sqlx migrate run

DATABASE_URL="mysql://root:@localhost/todos" cargo +nightly run
