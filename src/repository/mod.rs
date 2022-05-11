use sqlx::{migrate::Migrator, sqlite::SqlitePoolOptions, Pool, Sqlite};

mod note;

static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Clone)]
pub struct Repository(pub(crate) Pool<Sqlite>);

impl Repository {
    pub async fn from_env() -> anyhow::Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(std::env::var("DATABASE_URL")?.as_str())
            .await?;

        Ok(Repository(pool))
    }

    pub async fn migrate(&self) -> anyhow::Result<()> {
        MIGRATOR.run(&self.0).await?;

        Ok(())
    }
}
