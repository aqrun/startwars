use async_graphql::{Object, Context, FieldResult};
use crate::typings::{GqlResult, Episode};
use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use crate::dbs::StarWars;
use crate::services;
use crate::models::{Droid, Human, Character};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hero(
        &self,
        ctx: &Context<'_>,
        #[graphql(
            desc = "无值返回所有系列的角色，有值返回指定的系列角色"
        )]
        episode: Option<Episode>,
    ) -> GqlResult<Character> {
        let db = ctx.data_unchecked::<StarWars>();

        if let Some(episode) = episode {
            if episode == Episode::Empire {
                services::human::get_hero(db).await
            } else {
                services::droid::get_hero(db).await
            }
        } else {
            services::human::get_hero(db).await
        }
    }

    async fn human(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "人类ID")]
        id: String,
    ) -> GqlResult<Character> {
        let db = ctx.data_unchecked::<StarWars>();
        services::human::get_human_by_id(db, &id).await
    }

    // async fn humans(
    //     &self,
    //     ctx: &Context<'_>,
    //     after: Option<String>,
    //     before: Option<String>,
    //     first: Option<i32>,
    //     last: Option<i32>,
    // ) -> FieldResult<Connection<usize, Human, EmptyFields, EmptyFields>> {
    //     let humans = ctx
    //         .data_unchecked::<StarWars>()
    //         .humans()
    //         .iter()
    //         .copied()
    //         .collect::<Vec<_>>();
    //     query_characters(after, before, first, last, &humans)
    //         .await
    //         .map(|conn| conn.map_node(Human))
    // }
}