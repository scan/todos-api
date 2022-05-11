mod auth;
mod filter;
mod graphql;
mod handler;
mod model;
mod repository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init();

    let repo = repository::Repository::from_env().await?;
    repo.migrate().await?;

    let schema = graphql::create_schema_with_context();
    let routes = filter::all(&schema);

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;

    Ok(())
}
