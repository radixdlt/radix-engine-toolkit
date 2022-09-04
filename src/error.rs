use crate::models::value::ValueKind;
use serde::Serialize;

/// Represents an error encountered by the operations of the crate.
///
/// This is the main error type used throughout this crate to represent errors of all kinds. This error enum is not
/// meant consist of other error types (as values of its variants) for three mains reasons:
///
/// 1. When errors are nested a few levels deep, their serialized representation looks very unintuitive and is not very
/// easy to understand.
/// 2. Modeling of non-nested errors is easier in other languages (such as TypeScript) which will be consuming and
/// making use of this API.
/// 3. Some errors can not be nested since they do not implement the `Serialize` trait and therefore Serde can not
/// serialize them.
///
/// Therefore, it is fine for variants of this enum to have values as long as these values are not error enums which
/// need to be serialized.
///
/// Regarding the issue with certain errors not being serializable. An easy (but perhaps temporary) way of dealing with
/// this is to represent these errors as `String`s. As an example, a `DecimalParseError` can not be represented as the
/// following variant: `DecimalParseError(DecimalParseError)` since the `DecimalParseError` type does not implement
/// `Serialize`. But, a variant can be created which holds a `String` value and a `From<DecimalParseError>` trait can be
/// implemented for this error type to allow for `DecimalParseError` errors to be represented through this type.
#[derive(Serialize, Debug)]
#[serde(tag = "error", content = "value")]
pub enum Error {
    // ===============
    // General Errors
    // ===============
    /// An error emitted if the encoding or decoding of Bech32 addresses fails.
    AddressError(String),

    /// An error emitted when the decoding of SBOR fails.
    DecodeError(String),

    // =====================
    // Serde Related Errors
    // =====================
    /// An error emitted when deserialization of the JSON payload fails.
    DeserializationError(String),

    /// An error emitted when the contents of the request string pointer could not be loaded.
    InvalidRequestString(String),

    // ========================
    // Value Conversion Errors
    // ========================
    /// An error emitted during the conversion from transaction::ast::Value => crate::Value. This error typically means
    /// that the contents of some value were unexpected. An example of this is a package address being found inside of
    /// a `transaction::ast::Value::Decimal`.
    UnexpectedContents {
        kind: ValueKind,
        expected: Vec<ValueKind>,
        found: ValueKind,
    },

    /// An error emitted when validating the type of `Value` objects.
    InvalidType {
        expected_type: ValueKind,
        actual_type: ValueKind,
    },

    /// An error emitted when the parsing of a value from string fails.
    ParseError { kind: ValueKind, error: String },

    // ==============================
    // Instruction Conversion Errors
    // ==============================
    /// An error emitted when an error is encountered during transaction compilation.
    TransactionCompileError(String),

    /// An error when an error encountered during transaction manifest decompilation.
    TransactionDecompileError(String),

    /// An error emitted when a transaction version is specified but the library has no support for
    /// this transaction version.
    UnsupportedTransactionVersion(u8),

    /// An error emitted during the conversion of ast::Instructions to a `TransactionManifest`
    GeneratorError(String),

    // ===========================
    // Internal Operations Errors
    // ===========================
    /// An error emitted when the conversion to a specific request or response type fails
    RequestResponseConversionError(String),
}

macro_rules! impl_from_error {
    ($($error_type: ty => $variant_ident: ident,)*) => {
        $(
            impl From<$error_type> for Error {
                fn from(error: $error_type) -> Self {
                    Self::$variant_ident(format!("{:?}", error))
                }
            }
        )*
    };
}

macro_rules! impl_from_parse_error {
    ($($error_type: ty => $kind: ident,)*) => {
        $(
            impl From<$error_type> for Error {
                fn from(error: $error_type) -> Self {
                    Self::ParseError {
                        kind: ValueKind::$kind,
                        error: format!("{:?}", error)
                    }
                }
            }
        )*
    };
}

impl_from_parse_error! {
    scrypto::prelude::ParseDecimalError => Decimal,
    scrypto::prelude::ParsePreciseDecimalError => PreciseDecimal,
    scrypto::prelude::ParseHashError => Hash,
    scrypto::prelude::ParseNonFungibleIdError => NonFungibleId,
    scrypto::prelude::ParseNonFungibleAddressError => NonFungibleAddress,
}

impl_from_error! {
    scrypto::address::AddressError => AddressError,
    sbor::DecodeError => DecodeError,

    serde_json::Error => DeserializationError,
    std::str::Utf8Error => InvalidRequestString,

    transaction::manifest::CompileError => TransactionCompileError,
    transaction::manifest::DecompileError => TransactionDecompileError,
    transaction::manifest::generator::GeneratorError => GeneratorError,
}
