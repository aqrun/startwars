use async_std::task;
use async_graphql::{Result};

use startwars1::run;

fn main() -> Result<()> {
    task::block_on(run())
}
