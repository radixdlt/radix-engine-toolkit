use crate::models::{value::ValueKind, RENodeKind};
use serde::{Deserialize, Serialize};

/// Represents an error encountered by the operations of the crate.
///
/// This is the main error type used throughout this crate to represent errors of all kinds. This
/// error enum is not meant consist of other error types (as values of its variants) for three
/// mains reasons:
///
/// 1. When errors are nested a few levels deep, their serialized representation looks very
/// unintuitive and is not very easy to understand.
/// 2. Modeling of non-nested errors is easier in other languages (such as TypeScript) which will be
/// consuming and making use of this API.
/// 3. Some errors can not be nested since they do not implement the `Serialize` trait and therefore
/// Serde can not serialize them.
///
/// Therefore, it is fine for variants of this enum to have values as long as these values are not
/// error enums which need to be serialized.
///
/// Regarding the issue with certain errors not being serializable. An easy (but perhaps temporary)
/// way of dealing with this is to represent these errors as `String`s. As an example, a
/// `DecimalParseError` can not be represented as the following variant:
/// `DecimalParseError(DecimalParseError)` since the `DecimalParseError` type does not implement
/// `Serialize`. But, a variant can be created which holds a `String` value and a
/// `From<DecimalParseError>` trait can be implemented for this error type to allow for
/// `DecimalParseError` errors to be represented through this type.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "error", content = "value")]
pub enum Error {
    // ===============
    // General Errors
    // ===============
    /// An error emitted if the encoding or decoding of Bech32 addresses fails.
    AddressError(String),

    /// An error emitted when attempting to get information on an address but its format could not
    /// be established. Typically, this means that the entity byte is invalid or unknown.
    UnrecognizedAddressFormat,

    /// An error emitted when the decoding of SBOR fails.
    SborDecodeError(String),

    /// An error emitted when the decoding of SBOR fails.
    SborEncodeError(String),

    // =====================
    // Serde Related Errors
    // =====================
    /// An error emitted when deserialization of a JSON payload fails.
    DeserializationError(String),

    /// An error emitted when the contents of the request string pointer could not be loaded.
    InvalidRequestString(String),

    // ========================
    // Value Conversion Errors
    // ========================
    /// An error emitted during the conversion from radix_transaction::ast::Value => crate::Value. This
    /// error typically means that the contents of some value were unexpected. An example of this is
    /// a package address being found inside of a [`radix_transaction::manifest::ast::Value::Decimal`].
    UnexpectedContents {
        kind_being_parsed: ValueKind,
        allowed_children_kinds: Vec<ValueKind>,
        found_child_kind: ValueKind,
    },

    /// An error emitted during the conversion of [`crate::models::RENode`] into the AST's native
    /// type. This error signals that unexpected contents were found when parsing the contents of
    /// RENode. For example, for the case where we are parsing a [`crate::models::RENode::Bucket`],
    /// we expect that it consists of a [`radix_transaction::manifest::ast::Value::String`] or a
    /// [`radix_transaction::manifest::ast::Value::U32`]. If when parsing this `RENode`, we
    /// encounter anything else that is not the types we expect, then this error is emitted.
    UnexpectedReNodeContents {
        kind_being_parsed: RENodeKind,
        allowed_children_kinds: Vec<ValueKind>,
        found_child_kind: ValueKind,
    },

    /// An error emitted when validating the type of `Value` objects.
    InvalidType {
        expected_types: Vec<ValueKind>,
        actual_type: ValueKind,
    },

    /// An error emitted when encountering an unknown SBOR type id during value conversion.
    UnknownTypeId { type_id: u8 },

    /// An error emitted when the parsing of a value from string fails.
    ParseError { kind: ValueKind, message: String },

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

    /// An error emitted when attempting to decompile a transaction intent but the format is not
    /// known to the library.
    UnrecognizedCompiledIntentFormat,

    /// An error emitted when attempting to validate a transaction fails.
    TransactionValidationError(String),

    /// An error emitted when the extraction of the package ABI fails.
    ExtractAbiError(String),

    /// An error emitted when there is a network mismatch between addresses and the header network
    NetworkMismatchError { expected: u8, found: u8 },
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
                        message: format!("{:?}", error)
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
    scrypto::prelude::ParseBlobError => Blob,
    scrypto::prelude::ParseExpressionError => Expression,
    scrypto::prelude::ParseEcdsaSecp256k1PublicKeyError => EcdsaSecp256k1PublicKey,
    scrypto::prelude::ParseEcdsaSecp256k1SignatureError => EcdsaSecp256k1Signature,
    scrypto::prelude::ParseEddsaEd25519PublicKeyError => EddsaEd25519PublicKey,
    scrypto::prelude::ParseEddsaEd25519SignatureError => EddsaEd25519Signature,
}

impl_from_error! {
    scrypto::radix_engine_interface::address::AddressError => AddressError,
    sbor::DecodeError => SborDecodeError,
    sbor::EncodeError => SborEncodeError,

    serde_json::Error => DeserializationError,
    std::str::Utf8Error => InvalidRequestString,

    radix_transaction::manifest::CompileError => TransactionCompileError,
    radix_transaction::manifest::DecompileError => TransactionDecompileError,
    radix_transaction::manifest::generator::GeneratorError => GeneratorError,
    radix_transaction::errors::TransactionValidationError => TransactionValidationError,
    radix_transaction::errors::SignatureValidationError => TransactionValidationError,
}
