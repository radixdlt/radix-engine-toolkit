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
        request_string_pointer: *const std::os::raw::c_char,
    ) -> Result<Self, Error> {
        let string: &str = std::ffi::CStr::from_ptr(request_string_pointer).to_str()?;
        Ok(serde_json::from_str(string)?)
    }
}
