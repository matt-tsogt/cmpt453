use std::time::{SystemTime, UNIX_EPOCH};

use async_stream::try_stream;
use tonic::{Request, Response, Status};

use main::service::{SimpleRequest, SimpleResponse};

use super::ServerStream;

fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

pub async fn handle(
    request: Request<SimpleRequest>,
) -> Result<Response<ServerStream>, Status> {
    let msg = request.into_inner().message;

    let stream = try_stream! {
        let response = SimpleResponse {
            message: msg,
            timestamp: now_unix(),
        };
        yield response;
    };

    Ok(Response::new(Box::pin(stream)))
}
