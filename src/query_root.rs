use async_graphql::{Object, Context, FieldResult};
use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use super::{Episode, Human, Droid, StarWars, Character};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hero(
        &self,
        ctx: &Context<'_>,
        #[graphql(
            desc = "无值返回所有系列的角色，有值返回指定的系列角色"
        )]
        episode: Episode,
    ) -> Character {
        if episode == Episode::Empire {
            Human(ctx.data_unchecked::<StarWars>().luke).into()
        } else {
            Droid(ctx.data_unchecked::<StarWars>().artoo).into()
        }
    }

    async fn human(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "人类ID")]
        id: String,
    ) -> Option<Human> {
        ctx.data_unchecked::<StarWars>().human(&id).map(Human)
    }

    async fn humans(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> FieldResult<Connection<usize, Human, EmptyFields, EmptyFields>> {
        let humans = ctx
            .data_unchecked::<StarWars>()
            .humans()
            .iter()
            .copied()
            .collect::<Vec<_>>();
        query_characters(after, before, first, last, &humans)
            .await
            .map(|conn| conn.map_node(Human))
    }
}

async fn query_characters(
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    characters: &[usize],
) -> FieldResult<Connection<usize, usize, EmptyFields, EmptyFields>> {
    query(
        after,
        before,
        first,
        last,
        |after, before, first, last| async move {
            let mut start = 0usize;
            let mut end = characters.len();

            // 如果有 after 参数 更新 start
            if let Some(after) = after {
                if after >= characters.len() {
                    return Ok(Connection::new(false, false));
                }
                start = after + 1;
            }

            // 如果有 before 参数 更新 end
            if let Some(before) = before {
                if before == 0 {
                    return Ok(Connection::new(false, false));
                }
                end = before;
            }

            let mut slice = &characters[start..end];

            // 如果有 first 参数
            if let Some(first) = first {
                // 更新切片为 first 参数指定大小的切片 最大是整个切片 从前向后取
                slice = &slice[..first.min(slice.len())];
                end -= first.min(slice.len());
            } else if let Some(last) = last {
                // 如果指定了 last 参数
                // 更新切片 从后向前取 last 个数据
                slice = &slice[slice.len() - last.min(slice.len())..];
                start = end - last.min(slice.len());
            }

            let mut connection = Connection::new(
                start > 0,
                end < characters.len()
            );
            connection.append(
                slice
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| Edge::new(start + idx, *item)),
            );
            Ok(connection)
        },
    )
        .await
}