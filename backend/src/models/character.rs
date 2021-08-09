use chrono::NaiveDateTime;
use async_graphql::{Interface};
use super::{Human, Droid};
use crate::typings::{Episode};

/// graphql 接口
/// 抽象具有特定字段的集合的对象
///
/// 这里是 Interface 接口类型
/// 差不多等同于 java 的 interface
/// 表示 human 和 droid 都继承自 Character
///
/// 前端体现为
///
/// type Human implements Character;
/// type Droid implements Character;
///
/// 接口中的字段数据会映射到具体的类型上
#[derive(Interface)]
#[graphql(
field(name = "id", type = "&str"),
field(name = "name", type = "&str"),
field(name = "friends", type = "Vec<Character>"),
field(name = "appears_in", type = "Vec<Episode>"),
)]
pub enum Character {
    // 人类
    Human(Human),
    // 机器人
    Droid(Droid),
}

/// 角色
#[derive(Queryabel)]
#[table_name="characters"]
pub struct StarWarsChar {
    pub id: i32,
    // 姓名
    pub name: String,
    // 英文姓名
    pub en_name: String,
    pub cha_type: String,
    // 朋友
    pub friends: String,
    // 参与的电影系列
    pub appears_in: String,
    // 籍贯行星
    pub home_planet: Option<String>,
    // 主要功能
    pub primary_function: Option<String>,
    pub created_at: NaiveDateTime,
}

impl StarWarsChar {
    // pub fn to_droid(&self) -> Droid {
    //     Droid {
    //         id: self.id,
    //         name: self.name,
    //         en_name: self.en_name,
    //         friends: self.friends.clone(),
    //         appears_in: self.appears_in.clone(),
    //         primary_function: self.primary_function,
    //     }
    // }

    // pub fn to_human(&self) -> Human {
    //     Human {
    //         id: self.id,
    //         name: self.name,
    //         en_name: self.en_name,
    //         friends: self.friends.clone(),
    //         appears_in: self.appears_in.clone(),
    //         home_planet: self.home_planet,
    //     }
    // }
}

#[derive(Queryabel)]
pub struct Users{
    pub id: i32,
    // 姓名
    pub name: String,
    // 英文姓名
    pub en_name: String,
    pub user_type: String,
    // 朋友
    pub friends: String,
    // 参与的电影系列
    pub appears_in: String,
    // 籍贯行星
    pub home_planet: Option<String>,
    // 主要功能
    pub primary_function: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Insertable)]
#[table_name="user_friends"]
pub struct UserFriends {
    pub id: i32,
    pub friend_id: i32,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub en_name: String,
    pub cha_type: String,
    pub friends: String,
    pub appears_in: String,
    pub home_planet: String,
    pub primary_function: String,
}
