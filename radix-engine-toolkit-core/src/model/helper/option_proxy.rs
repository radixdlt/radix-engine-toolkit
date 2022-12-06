use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "variant")]
pub enum OptionProxy<T> {
    Some { field: T },
    None,
}

impl<T> From<Option<T>> for OptionProxy<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Option::Some(field) => Self::Some { field },
            Option::None => Self::None,
        }
    }
}

impl<T> From<OptionProxy<T>> for Option<T> {
    fn from(option: OptionProxy<T>) -> Self {
        match option {
            OptionProxy::Some { field } => Self::Some(field),
            OptionProxy::None => Self::None,
        }
    }
}
