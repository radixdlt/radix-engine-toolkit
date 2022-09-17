use crate::error::Error;

pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
}
