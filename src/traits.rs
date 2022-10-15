use crate::error::Error;
use serde::{Deserialize, Serialize};

pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
}

pub trait Request<'a, Response>
where
    Self: Deserialize<'a> + Validate,
    Response: Serialize + Validate,
{
    fn validate_request(&self) -> Result<(), Error> {
        self.validate()
    }
    fn validate_response(response: &Response) -> Result<(), Error> {
        response.validate()
    }

    fn handle_request(self) -> Result<Response, Error>;
    fn fulfill_request(self) -> Result<Response, Error> {
        self.validate_request()?;
        let response: Response = self.handle_request()?;
        Self::validate_response(&response)?;
        Ok(response)
    }

    /// Creates a new request from a character pointer
    ///
    /// # Safety
    ///
    /// This function makes use of pointers which is an unsafe feature.
    unsafe fn new_from_pointer(
        request_string_pointer: crate::memory::Pointer,
    ) -> Result<Self, Error> {
        crate::memory::toolkit_read_and_deserialize_string_from_memory(request_string_pointer)
    }
}
