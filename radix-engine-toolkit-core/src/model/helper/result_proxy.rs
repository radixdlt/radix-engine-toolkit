use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "variant")]
pub enum ResultProxy<O, E> {
    Ok { field: O },
    Err { field: E },
}

impl<O, E> From<ResultProxy<O, E>> for Result<O, E> {
    fn from(result: ResultProxy<O, E>) -> Self {
        match result {
            ResultProxy::Ok { field } => Result::Ok(field),
            ResultProxy::Err { field } => Result::Err(field),
        }
    }
}

impl<O, E> From<Result<O, E>> for ResultProxy<O, E> {
    fn from(result: Result<O, E>) -> Self {
        match result {
            Result::Ok(field) => ResultProxy::Ok { field },
            Result::Err(field) => ResultProxy::Err { field },
        }
    }
}
