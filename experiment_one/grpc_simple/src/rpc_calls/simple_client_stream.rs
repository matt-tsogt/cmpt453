use std::time::{SystemTime, UNIX_EPOCH};
use tonic::{Request, Response, Status, Streaming};

use main::service::{SimpleRequest, SimpleResponse};

fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

pub async fn handle(
    request: Request<Streaming<SimpleRequest>>,
) -> Result<Response<SimpleResponse>, Status> {
    let mut inbound = request.into_inner();
    let mut count = 0;

    while let Some(_req) = inbound.message().await? {
        count += 1;
    }

    let response = SimpleResponse {
        message: format!("count={}", count),
        timestamp: now_unix(),
    };

    Ok(Response::new(response))
}
