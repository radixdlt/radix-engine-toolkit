use serde::{Deserialize, Serialize};

// ===============================
// ValueKind Type and Conversions
// ===============================

macro_rules! define_value_kind{
    (
     $(#[$meta:meta])*
     $vis:vis enum $enum_ident:ident {
        $(
            $(#[$variant_metadata:meta])*
            $variant_ident:ident
        ),*$(,)*
    }
    ) => {
        $(#[$meta])*
        $vis enum $enum_ident {
            $(
                $(#[$variant_metadata])*
                $variant_ident,
            )*
        }

        impl Into<transaction::manifest::ast::Type> for $enum_ident {
            fn into(self) -> transaction::manifest::ast::Type {
                match self {
                    $(
                        Self::$variant_ident => transaction::manifest::ast::Type::$variant_ident,
                    )*
                }
            }
        }

        impl From<transaction::manifest::ast::Type> for $enum_ident {
            fn from(value: transaction::manifest::ast::Type) -> $enum_ident {
                match value {
                    $(
                        transaction::manifest::ast::Type::$variant_ident => Self::$variant_ident,
                    )*
                }
            }
        }
    }
}

define_value_kind! {
    #[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum ValueKind {
        Unit,
        Bool,

        I8,
        I16,
        I32,
        I64,
        I128,

        U8,
        U16,
        U32,
        U64,
        U128,

        String,

        Struct,
        Enum,

        Option,
        Array,
        Tuple,
        Result,

        List,
        Set,
        Map,

        Decimal,
        PreciseDecimal,

        PackageAddress,
        ComponentAddress,
        ResourceAddress,

        Hash,

        Bucket,
        Proof,

        NonFungibleId,
        NonFungibleAddress,

        Bytes,
    }
}
