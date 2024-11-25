use super::service::messaging_service_server::MessagingService;
use super::service::{MessageRequest, MessageResponse};
use tonic::{Request, Response, Status};
// Remove unused imports
use redis::aio::MultiplexedConnection;
use sqlx::Pool;
use sqlx::Postgres;

#[derive(Clone)]
pub struct MyMessagingService {
    db_pool: Pool<Postgres>,
    redis_conn: MultiplexedConnection,
}

impl MyMessagingService {
    pub fn new(db_pool: Pool<Postgres>, redis_conn: MultiplexedConnection) -> Self {
        Self { db_pool, redis_conn }
    }
}

#[tonic::async_trait]
impl MessagingService for MyMessagingService {
    async fn send_message(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        let req = request.into_inner();
        let mut conn = self.redis_conn.clone();

        // Cache the message in Redis
        let _: () = redis::cmd("SET")
            .arg(format!("message:{}", req.user_id))
            .arg(&req.message)
            .query_async(&mut conn)
            .await
            .map_err(|e| Status::internal(format!("Redis error: {}", e)))?;

        // Optionally, store the message in PostgreSQL (omitted for brevity)

        Ok(Response::new(MessageResponse {
            status: "Message sent".into(),
        }))
    }
}