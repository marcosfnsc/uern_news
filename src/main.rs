use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

#[tokio::main]
async fn main() {
    let poll = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename("posts.sqlite")
            .create_if_missing(true),
    )
    .await
    .unwrap();
}
