use database::{create_pool, run_migrations};
use sqlx::query;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = create_pool(&database_url).await?;
    run_migrations(&pool).await?;

    query!("INSERT INTO example (name) VALUES ($1)", "Hello, world!")
        .execute(&pool)
        .await?;

    let results = query!("SELECT name FROM example").fetch_all(&pool).await?;

    for row in results {
        println!("Result: {:?}", row.name);
    }

    Ok(())
}
