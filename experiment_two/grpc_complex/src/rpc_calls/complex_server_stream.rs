use async_stream::try_stream;
use tonic::{Request, Response, Status};

use main::service::{ComplexRequest, ComplexResponse};
use crate::util::complex_response;

use super::ServerStream;

pub async fn handle(
    request: Request<ComplexRequest>,
) -> Result<Response<ServerStream>, Status> {
    let req = request.into_inner();
    let num_readings = req.readings.len().max(1);

    let stream = try_stream! {
        // One dummy complex response per RPC
        let response: ComplexResponse = complex_response(num_readings);
        yield response;
    };

    Ok(Response::new(Box::pin(stream)))
}
