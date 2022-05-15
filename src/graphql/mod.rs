mod mutation;
mod query;

use async_graphql::{Context, EmptySubscription, Schema};

use crate::repository::Repository;

pub type AppSchema = Schema<query::Query, mutation::Mutation, EmptySubscription>;

pub fn get_repo_from_ctx<'ctx>(ctx: &Context<'ctx>) -> anyhow::Result<&'ctx Repository> {
    ctx.data::<Repository>()
        .map_err(|err| anyhow::Error::msg(err.message))
}

pub fn create_schema_with_context(repo: Repository) -> AppSchema {
    Schema::build(query::Query, mutation::Mutation, EmptySubscription)
        .data(repo)
        .finish()
}
