use async_graphql::{Context, Object};

pub struct Query;

#[Object]
impl Query {
    pub async fn version<'ctx>(&self, _ctx: &Context<'ctx>) -> &str {
        "0.0.1"
    }
}
