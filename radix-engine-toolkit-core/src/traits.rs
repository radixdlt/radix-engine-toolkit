use crate::{error::Error, models::ManifestInstructionsKind};
use scrypto::prelude::{hash, Hash};
use serde::{Deserialize, Serialize};

/// A trait that defines the common interface for a type which can be validated. This validation
/// happens without external context, internal only.
pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
}

pub trait ValidateWithContext<T> {
    fn validate(&self, context: T) -> Result<(), Error>;
}

/// A trait that defines the common interface for a request and response.
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
        crate::memory::toolkit_read_and_deserialize_json_string_from_memory(request_string_pointer)
    }
}

/// A trait for the conversions into a different types with generic external context
pub trait TryIntoWithContext<T, C> {
    type Error;

    fn try_into_with_context(self, context: C) -> Result<T, Self::Error>;
}

pub trait CompilableIntent {
    fn compile(&self) -> Result<Vec<u8>, Error>;

    fn decompile<T>(
        data: &T,
        output_manifest_format: ManifestInstructionsKind,
    ) -> Result<Self, Error>
    where
        Self: Sized,
        T: AsRef<[u8]>;

    fn hash(&self) -> Result<Hash, Error> {
        self.compile().map(|x| hash(x))
    }
}
