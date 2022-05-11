mod query;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub type AppSchema = Schema<query::Query, EmptyMutation, EmptySubscription>;

pub fn create_schema_with_context() -> AppSchema {
    Schema::build(query::Query, EmptyMutation, EmptySubscription).finish()
}
