use crate::typings::{GqlResult};
use crate::dbs::StarWars;
use crate::models::{Character, Human};
use async_graphql::Error;

pub async fn get_human_by_id(db: StarWars, id: &str) -> GqlResult<Human> {
    if let Some(current_id) = db.human(id) {
        let human = db.chars[current_id].to_human();
        Ok(human)
    } else {
        Err(Error::new("human not exist"))
    }
}

pub async fn get_hero(db: StarWars) -> GqlResult<Character> {
    let hero = db.chars[db.luke].to_human();
    Ok(Character::Human(hero))
}