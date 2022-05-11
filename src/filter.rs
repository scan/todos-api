use async_graphql_warp::{graphql_subscription, GraphQLResponse};
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};

use crate::{auth, graphql::AppSchema, handler};

pub fn all(schema: &AppSchema) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let cors = warp::cors()
        .allow_credentials(true)
        .allow_origins(vec!["http://localhost:3000"])
        .allow_headers(vec!["Content-Type", "User-Agent", "Referer", "Origin"])
        .allow_methods(vec!["POST", "GET"])
        .build();

    graphql_subscription(schema.clone())
        .or(graphql_post(schema))
        .or(health())
        .or(not_found())
        .with(cors)
}

fn health() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("health")
        .and(warp::get())
        .and_then(handler::health)
}

fn not_found() -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    warp::any().and_then(handler::not_found)
}

fn graphql_post(
    schema: &AppSchema,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::header::optional::<String>("authorization")
    .and(warp::post())
    .and(warp::header::exact_ignore_case(
      "content-type",
      "application/json",
    ))
    .and(async_graphql_warp::graphql(schema.clone()))
    .and_then(
      |token: Option<String>,
       (schema, mut request): (AppSchema, async_graphql::Request)| async move {
        if let Some(bearer) = token.and_then(|s| auth::parse_bearer_token(&s)) {
          request = request.data(bearer);
        }

        let resp = schema.execute(request).await;

        Ok::<_, Infallible>(GraphQLResponse::from(resp))
      },
    )
}
