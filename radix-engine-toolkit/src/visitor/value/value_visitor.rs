use crate::{Value, ValueKind};

macro_rules! define_value_visitor {
    (
        $(#[$meta:meta])*
        $vis: vis trait $trait_ident: ident {
            $($method_ident: ident),*
        }
    ) => {
        $(#[$meta])*
        $vis trait $trait_ident {
            $(
                fn $method_ident(&mut self, _value: &mut $crate::Value) -> $crate::Result<()> {
                    Ok(())
                }
            )*
        }
    };
}

macro_rules! visit {
    ($visitors: expr, $method: ident, $value: expr) => {
        $visitors
            .iter_mut()
            .map(|visitor| visitor.$method($value))
            .collect::<$crate::Result<Vec<_>>>()
    };
}

define_value_visitor! {
    /// A trait which defines a [`crate::Value`] visitor which operates on unstructured values, this
    /// choice is made to allow the visitor to work with aliasing an dealiasing operations which
    /// involves the visitor changing the value variant.
    pub trait ValueVisitor {
        visit_bool,

        visit_u8,
        visit_u16,
        visit_u32,
        visit_u64,
        visit_u128,

        visit_i8,
        visit_i16,
        visit_i32,
        visit_i64,
        visit_i128,

        visit_string,

        visit_enum,
        visit_some,
        visit_none,
        visit_ok,
        visit_err,

        visit_array,
        visit_map,
        visit_tuple,

        visit_decimal,
        visit_precise_decimal,

        visit_own,

        visit_component_address,
        visit_resource_address,
        visit_package_address,

        visit_hash,
        visit_ecdsa_secp256k1_public_key,
        visit_ecdsa_secp256k1_signature,
        visit_eddsa_ed25519_public_key,
        visit_eddsa_ed25519_signature,

        visit_bucket,
        visit_proof,

        visit_non_fungible_global_id,
        visit_non_fungible_local_id,

        visit_expression,
        visit_blob,
        visit_bytes
    }
}

pub fn traverse_value(
    value: &mut crate::Value,
    visitors: &mut [&mut dyn ValueVisitor],
) -> crate::Result<()> {
    // Visit the top level value parts
    match value.kind() {
        ValueKind::Bool => visit!(visitors, visit_bool, value)?,

        ValueKind::U8 => visit!(visitors, visit_u8, value)?,
        ValueKind::U16 => visit!(visitors, visit_u16, value)?,
        ValueKind::U32 => visit!(visitors, visit_u32, value)?,
        ValueKind::U64 => visit!(visitors, visit_u64, value)?,
        ValueKind::U128 => visit!(visitors, visit_u128, value)?,

        ValueKind::I8 => visit!(visitors, visit_i8, value)?,
        ValueKind::I16 => visit!(visitors, visit_i16, value)?,
        ValueKind::I32 => visit!(visitors, visit_i32, value)?,
        ValueKind::I64 => visit!(visitors, visit_i64, value)?,
        ValueKind::I128 => visit!(visitors, visit_i128, value)?,

        ValueKind::String => visit!(visitors, visit_string, value)?,

        ValueKind::Enum => visit!(visitors, visit_enum, value)?,

        ValueKind::Some => visit!(visitors, visit_some, value)?,
        ValueKind::None => visit!(visitors, visit_none, value)?,
        ValueKind::Ok => visit!(visitors, visit_ok, value)?,
        ValueKind::Err => visit!(visitors, visit_err, value)?,

        ValueKind::Map => visit!(visitors, visit_map, value)?,
        ValueKind::Array => visit!(visitors, visit_array, value)?,
        ValueKind::Tuple => visit!(visitors, visit_tuple, value)?,

        ValueKind::Decimal => visit!(visitors, visit_decimal, value)?,
        ValueKind::PreciseDecimal => visit!(visitors, visit_precise_decimal, value)?,

        ValueKind::Own => visit!(visitors, visit_own, value)?,

        ValueKind::ComponentAddress => visit!(visitors, visit_component_address, value)?,
        ValueKind::ResourceAddress => visit!(visitors, visit_resource_address, value)?,
        ValueKind::PackageAddress => visit!(visitors, visit_package_address, value)?,

        ValueKind::Hash => visit!(visitors, visit_hash, value)?,

        ValueKind::EcdsaSecp256k1PublicKey => {
            visit!(visitors, visit_ecdsa_secp256k1_public_key, value)?
        }
        ValueKind::EcdsaSecp256k1Signature => {
            visit!(visitors, visit_ecdsa_secp256k1_signature, value)?
        }
        ValueKind::EddsaEd25519PublicKey => {
            visit!(visitors, visit_eddsa_ed25519_public_key, value)?
        }
        ValueKind::EddsaEd25519Signature => visit!(visitors, visit_eddsa_ed25519_signature, value)?,

        ValueKind::Bucket => visit!(visitors, visit_bucket, value)?,
        ValueKind::Proof => visit!(visitors, visit_proof, value)?,

        ValueKind::NonFungibleLocalId => visit!(visitors, visit_non_fungible_local_id, value)?,
        ValueKind::NonFungibleGlobalId => visit!(visitors, visit_non_fungible_global_id, value)?,

        ValueKind::Expression => visit!(visitors, visit_expression, value)?,
        ValueKind::Blob => visit!(visitors, visit_blob, value)?,
        ValueKind::Bytes => visit!(visitors, visit_bytes, value)?,
    };

    // Attempt to continue traversal on the value children (contained nested values). For future
    // reference, any variant that has a `Value` inside of it should go here.
    match value {
        Value::Map {
            entries: values, ..
        } => {
            values
                .iter_mut()
                .flat_map(|(x, y)| [x, y])
                .map(|value| traverse_value(value, visitors))
                .collect::<crate::Result<Vec<_>>>()?;
        }
        Value::Enum {
            fields: Some(values),
            ..
        }
        | Value::Array {
            elements: values, ..
        }
        | Value::Tuple {
            elements: values, ..
        } => {
            values
                .iter_mut()
                .map(|value| traverse_value(value, visitors))
                .collect::<crate::Result<Vec<_>>>()?;
        }
        Value::Some { value } | Value::Ok { value } | Value::Err { value } => {
            traverse_value(value, visitors)?;
        }
        Value::Bool { .. }
        | Value::U8 { .. }
        | Value::U16 { .. }
        | Value::U32 { .. }
        | Value::U64 { .. }
        | Value::U128 { .. }
        | Value::I8 { .. }
        | Value::I16 { .. }
        | Value::I32 { .. }
        | Value::I64 { .. }
        | Value::I128 { .. }
        | Value::String { .. }
        | Value::Enum { fields: None, .. }
        | Value::None { .. }
        | Value::Decimal { .. }
        | Value::PreciseDecimal { .. }
        | Value::Own { .. }
        | Value::ComponentAddress { .. }
        | Value::ResourceAddress { .. }
        | Value::PackageAddress { .. }
        | Value::Hash { .. }
        | Value::EcdsaSecp256k1PublicKey { .. }
        | Value::EcdsaSecp256k1Signature { .. }
        | Value::EddsaEd25519PublicKey { .. }
        | Value::EddsaEd25519Signature { .. }
        | Value::Bucket { .. }
        | Value::Proof { .. }
        | Value::NonFungibleLocalId { .. }
        | Value::NonFungibleGlobalId { .. }
        | Value::Expression { .. }
        | Value::Blob { .. }
        | Value::Bytes { .. } => { /* No OP. Doesn't contain a Value */ }
    };

    Ok(())
}
