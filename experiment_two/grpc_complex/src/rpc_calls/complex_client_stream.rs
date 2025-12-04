use tonic::{Request, Response, Status, Streaming};
use main::service::{ComplexRequest, ComplexResponse};
use crate::util::complex_response;

pub async fn handle(
    request: Request<Streaming<ComplexRequest>>,
) -> Result<Response<ComplexResponse>, Status> {
    let mut inbound = request.into_inner();
    let mut count: usize = 0;

    // Just count how many messages we got
    while let Some(_req) = inbound.message().await? {
        count += 1;
    }

    // Use 'count' as the reading length in the dummy response
    let response = complex_response(count.max(1));

    Ok(Response::new(response))
}
