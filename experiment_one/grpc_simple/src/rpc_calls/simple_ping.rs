use tonic::{Request, Response, Status};
use crate::rpc_calls::util::now_unix;
use main::service::{SimpleRequest, SimpleResponse};


pub async fn handle(request: Request<SimpleRequest>) -> Result<Response<SimpleResponse>, Status> {
    let req = request.into_inner();

    let response = SimpleResponse {
        message: req.message,
        timestamp: now_unix(),
    };

    Ok(Response::new(response))
}
