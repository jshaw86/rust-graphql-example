cargo build

cargo install sqlx-cli
DATABASE_URL="postgres://@localhost/todos" cargo sqlx migrate run

DATABASE_URL="postgres://@localhost/todos" cargo run
