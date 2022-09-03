use super::*;

macro_rules! define_request_response_enum{
    (
     $(#[$meta:meta])*
     $vis:vis enum $enum_ident:ident {
        $(
            $(#[$variant_metadata:meta])*
            $variant_ident:ident($internal_type:ty)
        ),*$(,)*
    }
    ) => {
        $(#[$meta])*
        $vis enum $enum_ident {
            $(
                $(#[$variant_metadata])*
                $variant_ident($internal_type),
            )*
        }

        $(
            impl From<$internal_type> for $enum_ident {
                fn from(request: $internal_type) -> Self {
                    Self::$variant_ident(request)
                }
            }

            impl TryInto<$internal_type> for $enum_ident {
                type Error = crate::error::Error;

                fn try_into(self) -> Result<$internal_type, Self::Error> {
                    if let Self::$variant_ident(request) = self {
                        Ok(request)
                    } else {
                        Err(Self::Error::RequestResponseConversionError(format!("Failed to convert request to: {}", stringify!($enum_ident)).into()))
                    }
                }
            }
        )*
    }
}

define_request_response_enum! {
    pub enum Request {
        InformationRequest(InformationRequest),
    }
}

define_request_response_enum! {
    pub enum Response {
        InformationResponse(InformationResponse),
    }
}
