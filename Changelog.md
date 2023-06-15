# 0.10.0 (WIP)

All across the Radix Engine Toolkit all types which are serialized as discriminated unions now have a discriminator called `kind` and a value field of `value`. There are some exceptions to this where the value is not a single value but multiple such as `SignatureWithPublicKey`.

## Value Models

### Manifest AST Value

* This value model has been renamed to `ManifestValue`.
* All aliases have been removed: `Some`, `None`, `Ok`, `Err`, `NonFungibleGlobalId`, and `Bytes`.
* Enums no longer accept string or u8 discriminators, only u8.

## Functions

* Added a new function for deriving the virtual signature badge non-fungible global id from a public key: `derive_virtual_signature_non_fungible_global_id_from_public_key`.
* Added a new function for deriving the public key associated with an Olympia account address: `derive_public_key_from_olympia_account_address`.
* Added a new function for deriving the node address for a given Ecdsa Secp256k1 public key: `derive_node_address_from_public_key`. Note that the term "node" here does not refer to RE (Radix Engine) nodes, but refers to Radix Nodes.

### `information`

* The `information` function has been renamed to `build_information`.
* The `last_commit_hash` field has been removed.
* Added a new `scrypto_dependency` field which has the current version of the Scrypto dependency used. This information is obtained directly from the Cargo.toml manifest file, thus it is guaranteed to always be accurate.

### `derive_virtual_account_address`

* The `derive_virtual_account_address` function has been renamed to `derive_virtual_account_address_from_public_key`.
* The return type of the `derive_virtual_account_address_from_public_key` is now just a `String` instead of a JSON object.

### `derive_virtual_identity_address`

* The `derive_virtual_identity_address` function has been renamed to `derive_virtual_identity_address_from_public_key`.
* The return type of the `derive_virtual_identity_address_from_public_key` is now just a `String` instead of a JSON object.

### `derive_babylon_address_from_olympia_address`

* The `derive_babylon_address_from_olympia_address` function has been renamed to `derive_virtual_account_address_from_olympia_account_address`.
* This function used to return both the public key and the virtual account address, this is no longer the case. This function now only returns the Babylon virtual account address as a `String`. The public key can be derived from an Olympia address through the `derive_public_key_from_olympia_account_address` function.

## Types

### `PublicKey`

* The discriminator field has been renamed from `curve` to `kind`.
* The value field has been renamed from `public_key` to `value`.