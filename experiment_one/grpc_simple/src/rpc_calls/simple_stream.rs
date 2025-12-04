use std::time::{SystemTime, UNIX_EPOCH};

use async_stream::try_stream;
use tonic::{Request, Response, Status, Streaming};

use main::service::{SimpleRequest, SimpleResponse};

use super::ServerStream;

fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

pub async fn handle(
    request: Request<Streaming<SimpleRequest>>,
) -> Result<Response<ServerStream>, Status> {
    let mut inbound = request.into_inner();

    let stream = try_stream! {
        while let Some(req) = inbound.message().await? {
            let response = SimpleResponse {
                message: req.message,
                timestamp: now_unix(),
            };
            yield response;
        }
    };

    Ok(Response::new(Box::pin(stream)))
}
