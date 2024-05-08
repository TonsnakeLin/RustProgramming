use mini_redis::Result;
use my_redis::redis_get_set;

#[tokio::main]
async fn main() -> Result<()>{
    redis_get_set().await?;
    Ok(())
}
