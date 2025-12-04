use tonic::{Request, Response, Status};
use main::service::{ComplexRequest, ComplexResponse};
use crate::util::complex_response;

pub async fn handle(
    request: Request<ComplexRequest>,
) -> Result<Response<ComplexResponse>, Status> {
    let req = request.into_inner();

    // Use length of incoming readings to decide how many dummy readings to send back.
    let num_readings = req.readings.len().max(1);

    let response = complex_response(num_readings);

    Ok(Response::new(response))
}
