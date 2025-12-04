use async_stream::try_stream;
use tonic::{Request, Response, Status, Streaming};

use main::service::ComplexRequest;
use crate::util::complex_response;

use super::ServerStream;

pub async fn handle(
    request: Request<Streaming<ComplexRequest>>,
) -> Result<Response<ServerStream>, Status> {
    let mut inbound = request.into_inner();

    let stream = try_stream! {
        while let Some(req) = inbound.message().await? {
            let num_readings = req.readings.len().max(1);
            let response = complex_response(num_readings);
            yield response;
        }
    };

    Ok(Response::new(Box::pin(stream)))
}
