use serde::{Serialize, Deserialize};

use async_graphql::{Object, Context};
use crate::dbs::StarWars;
use crate::typings::{Episode};
use super::Character;
use crate::services::droid::get_droid_by_id;

/// 机器人
#[derive(Serialize, Deserialize, Clone)]
pub struct Droid {
    pub id: &'static str,
    // 姓名
    pub name: &'static str,
    // 英文姓名
    pub en_name: &'static str,
    // 朋友
    pub friends: Vec<usize>,
    // 参与的电影系列
    pub appears_in: Vec<Episode>,
    // 主要功能
    pub primary_function: Option<&'static str>,
}

#[Object]
impl Droid {
    pub async fn id(&self) -> &str {
        self.id.as_str()
    }

    pub async fn en_name(&self) -> &str {
        self.en_name.as_str()
    }

    pub async fn name(&self) -> &str {
        self.name.as_str()
    }

    pub async fn friends(&self, ctx: &Context<'_>) -> Vec<Character> {
        let db = ctx.data_unchecked::<StarWars>();
        self.friends
            .iter()
            .map(|id| {
                db.chars[id.into()].to_droid()
            })
            .collect()
    }

    pub async fn appears_in<'a>(&self) -> &'a [Episode] {
        self.appears_in.into()
    }

    pub async fn primary_function<'a>(&self) -> &'a Option<&'a str> {
        self.primary_function.into()
    }
}