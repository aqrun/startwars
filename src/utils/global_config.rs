use dotenv::{dotenv, var};
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::constants::{
    ADDRESS, PORT, GRAPHQL_PATH, GRAPHIQL_PATH,
    DEFAULT_ADDRESS, DEFAULT_PORT,
    DEFAULT_GRAPHQL_PATH, DEFAULT_GRAPHIQL_PATH,
};

lazy_static! {
    pub static ref G: HashMap<&'static str, String> = {
        dotenv().ok();

        let mut map = HashMap::new();

        map.insert(ADDRESS, var(ADDRESS).unwrap_or(DEFAULT_ADDRESS.into()));
        map.insert(PORT, var(PORT).unwrap_or(DEFAULT_PORT.into()));
        map.insert(GRAPHQL_PATH, var(GRAPHQL_PATH).unwrap_or(DEFAULT_GRAPHQL_PATH.into()));
        map.insert(GRAPHIQL_PATH, var(GRAPHIQL_PATH).unwrap_or(DEFAULT_GRAPHIQL_PATH.into()));

        map
    };
}
