use redis::aio::ConnectionManager;
use redis::Client;
use std::env;

pub async fn get_redis_conn() -> ConnectionManager {
    dotenv::dotenv().ok();
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = Client::open(redis_url).expect("Invalid REDIS_URL");
    client
        .get_tokio_connection_manager()
        .await
        .expect("Failed to connect to Redis")
}
