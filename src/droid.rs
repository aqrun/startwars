use super::{Character, Episode, StarWars};
use async_graphql::{Context, Object};

pub struct Droid(pub usize);

#[Object]
impl Droid {
    pub async fn id(&self, ctx: &Context<'_>) -> &str {
        ctx.data_unchecked::<StarWars>().chars[self.0].id
    }

    pub async fn name(&self, ctx: &Context<'_>) -> &str {
        ctx.data_unchecked::<StarWars>().chars[self.0].name
    }

    pub async fn friends(&self, ctx: &Context<'_>) -> Vec<Character> {
        ctx.data_unchecked::<StarWars>().chars[self.0]
            .friends
            .iter()
            .map(|id| Droid(*id).into())
            .collect()
    }

    pub async fn appears_in<'a>(&self, ctx: &'a Context<'_>) -> &'a [Episode] {
        &ctx.data_unchecked::<StarWars>().chars[self.0].appears_in
    }

    pub async fn primary_function<'a>(&self, ctx: &'a Context<'_>) -> &'a Option<&'a str> {
        &ctx.data_unchecked::<StarWars>().chars[self.0].primary_function
    }
}