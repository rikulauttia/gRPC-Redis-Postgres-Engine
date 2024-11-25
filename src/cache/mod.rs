use redis::Client;
use redis::aio::MultiplexedConnection;
use std::env;

pub async fn get_redis_conn() -> MultiplexedConnection {
    dotenv::dotenv().ok();
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = Client::open(redis_url).expect("Invalid REDIS_URL");
    client
        .get_multiplexed_tokio_connection()
        .await
        .expect("Failed to connect to Redis")
}