use async_graphql::{Result};
use async_graphql_tide::{endpoint};
use tide::security::{CorsMiddleware, Origin};
use tide::http::headers::HeaderValue;
use crate::utils::G;
use crate::constants::{
    ADDRESS, PORT, GRAPHQL_PATH, GRAPHIQL_PATH,
};
use crate::gql::{graphql, graphiql};

pub async fn run() -> Result<()> {
    let mut app = tide::new();

    app.at(G.get(ADDRESS).unwrap())
        .post(graphql);
    app.at(G.get(PORT).unwrap())
        .get(graphiql);

    let cors = CorsMiddleware::new()
        .allow_methods(
            "GET, POST, OPTIONS".parse::<HeaderValue>().unwrap()
        )
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);

    app.with(cors);
    app.listen(format!(
        "{}:{}",
        G.get(ADDRESS).unwrap(),
        G.get(PORT).unwrap(),
    ))
        .await?;

    Ok(())
}
