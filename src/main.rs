mod db;
mod cache;
mod grpc;

use grpc::handlers::MyMessagingService;
use grpc::service::messaging_service_server::MessagingServiceServer;
use tokio::sync::OnceCell;
use tonic::transport::Server;

static DB_POOL: OnceCell<sqlx::Pool<sqlx::Postgres>> = OnceCell::const_new();
static REDIS_CONN: OnceCell<redis::aio::MultiplexedConnection> = OnceCell::const_new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let db_pool = db::db::get_db_pool().await;
    let redis_conn = cache::get_redis_conn().await;

    DB_POOL.set(db_pool.clone()).unwrap();
    REDIS_CONN.set(redis_conn.clone()).unwrap();

    let addr = "[::1]:50051".parse().unwrap();
    let messaging_service = MyMessagingService::new(db_pool, redis_conn);

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(MessagingServiceServer::new(messaging_service))
        .serve(addr)
        .await?;

    Ok(())
}