
pub type GqlResult<T> = std::result::Result<T, async_graphql::Error>;

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