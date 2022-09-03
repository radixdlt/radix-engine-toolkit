use crate::error::Error;
use crate::models::{Request, Response};

pub fn validate_request<R: Into<Request> + Clone>(request: &R) -> Result<(), Error> {
    let request: Request = request.clone().into();
    match request {
        Request::InformationRequest(_) => Ok(()),
    }
}

pub fn validate_response<R: Into<Response> + Clone>(response: &R) -> Result<(), Error> {
    let response: Response = response.clone().into();
    match response {
        Response::InformationResponse(_) => Ok(()),
    }
}
