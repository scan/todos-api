mod query;

use async_graphql::{Context, EmptyMutation, EmptySubscription, Schema};

use crate::repository::Repository;

pub type AppSchema = Schema<query::Query, EmptyMutation, EmptySubscription>;

pub fn get_repo_from_ctx<'ctx>(ctx: &Context<'ctx>) -> anyhow::Result<&'ctx Repository> {
    ctx.data::<Repository>()
        .map_err(|err| anyhow::Error::msg(err.message))
}

pub fn create_schema_with_context(repo: Repository) -> AppSchema {
    Schema::build(query::Query, EmptyMutation, EmptySubscription)
        .data(repo)
        .finish()
}
