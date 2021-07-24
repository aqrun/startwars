use crate::{Human, Droid, QueryRoot};
use async_graphql::{Interface, Enum, Schema, EmptyMutation, EmptySubscription};

pub type StarWarsSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

///
/// 星战系列
/// 正传三部曲
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Episode {
    // 新希望
    NewHope,
    // 帝国反击战
    Empire,
    // 绝地归来
    Jedi
}

/// 角色类型
#[derive(Interface)]
#[graphql(
    field(name = "id", type = "&str"),
    field(name = "name", type = "&str"),
    field(name = "friends", type = "Vec<Character>"),
    field(name = "appears_in", type = "&'ctx [Episode]"),
)]
pub enum Character {
    // 人类
    Human(Human),
    // 机器人
    Droid(Droid),
}

/// 角色
pub struct StarWarsChar {
    pub id: &'static str,
    // 姓名
    pub name: &'static str,
    // 英文姓名
    pub en_name: &'static str,
    // 朋友
    pub friends: Vec<usize>,
    // 参与的电影系列
    pub appears_in: Vec<Episode>,
    // 籍贯行星
    pub home_planet: Option<&'static str>,
    // 主要功能
    pub primary_function: Option<&'static str>,
}