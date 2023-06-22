# 0.10.0 (WIP)

* All builds of the Radix Engine Toolkit now come with a `test-pack.tar.gz` file containing many test vectors to use when updating the toolkit wrappers to be compatible with the toolkit. More specifically, it includes the following:
    * The JSON Schema of all of the models used by the toolkit.
    * Test vectors of serialized models to include as part of the toolkit wrappers to ensure correct serialization/deserialization of models.
    * Test vectors for the input and output of functions to include as part of toolkit wrappers to ensure that the wrappers are able to invoke methods on the toolkit and receive expected responses.
* All across the Radix Engine Toolkit all types which are serialized as discriminated unions now have a discriminator called `kind` instead of `type` and a value field of `value`. There are some exceptions to this where the value is not a single value but multiple such as `SignatureWithPublicKey`.
* The Radix Engine Toolkit is now made up of modules. As an example, there is an `intent` module that has functions such as `intent_hash`, `intent_compile`, `intent_decompile`, and `intent_statically_validate`. 
    * All functions now follow the following naming scheme: `${module_name}_${function_name}`, thus, you can expect that all of the function names from the previous version of the toolkit has been renamed to adhere to this.
* The `network_id` is now being serialized as a `String` all across the toolkit in a consistent manner.

## Functions

### `build` Module

* Added a new `build` module which has functions related to the Radix Engine Toolkit build being used. 

#### `build_information` Function

* The `information` function has been renamed to `build_information` to be consistent with module naming scheme adopted with this version of the toolkit.
* The `build_information` function no longer returns the `last_commit_hash`, instead, it returns the `scrypto_dependency` version that is currently in use. 

```jsonc
{
    // The version of the Radix Engine Toolkit
    "version": "0.10.0",
    // The version of Scrypto used is returned, not a commit hash of the RET
    "scrypto_dependency": {
        "kind": "Branch", // RET is depending on a specific branch of Scrypto
        "value": "feature/fee-locks-in-execution-trace" // This is the branch is depends on
    }
}
```

### `instructions` Module

#### `instructions_hash` Function

* Added a new function to the `instructions` module that returns the hash of some given instructions: `instructions_hash`.

#### `instructions_convert` Function

* The `convert_manifest` function has been renamed to `instructions_convert`:
    * To be consistent with module naming scheme adopted with this version of the toolkit.
    * Because the function did not need to take a full manifest (instructions + blobs), it realistically only needed to take the instructions as input to convert them to another form.
* The `instructions_output_kind` field of the `instructions_convert` input has been renamed to `instructions_kind`.
* The `manifest` field of the `instructions_convert` input has been:
    * Renamed to `instructions`.
    * Only requires the instructions and not a complete `TransactionManifest`.

#### `instructions_compile` Function

* Added a new function to the `instructions` module that compiles the instructions by SBOR encoding them: `instructions_compile`.

#### `instructions_decompile` Function

* Added a new function to the `instructions` module that decompiles a byte array into instructions by SBOR decoding the byte array into instructions: `instructions_decompile`.

#### `instructions_statically_validate` Function

* Added a new function to the `instructions` module that validates the instructions statically: `instructions_statically_validate`.

#### `instructions_extract_addresses` Function

* The `extract_addresses_from_manifest` function has been renamed to `instructions_extract_addresses`:
    * To be consistent with the module naming scheme adopted with this version of the toolkit.
    * Because the function did not need to take a full manifest (instructions + blobs), it realistically only needed to take the instructions to extract the addresses from there.
* The `manifest` field of the `instructions_extract_addresses` input has been:
    * Renamed to `instructions`.
    * Only requires the instructions and not a complete `TransactionManifest`.
* The `instructions_extract_addresses` output has changed, please refer to the JSON Schema and Function examples provided in the test pack for more information. 

### `manifest` Module

#### `manifest_hash` Function

* Added a new function to the `manifest` module that returns the hash of some given manifest: `manifest_hash`.

#### `manifest_compile` Function

* Added a new function to the `manifest` module that compiles the manifest by SBOR encoding it: `manifest_compile`.

#### `manifest_decompile` Function

* Added a new function to the `manifest` module that decompiles a byte array into manifest by SBOR decoding the byte array into manifest: `manifest_decompile`.

#### `manifest_statically_validate` Function

* Added a new function to the `manifest` module that validates the manifest statically: `manifest_statically_validate`.


### `intent` Module

#### `intent_hash` Function

* The `hash_transaction_intent` function has been renamed to `intent_hash` to be consistent with module naming scheme adopted with this version of the toolkit.
* The `hash_transaction_intent` output is no longer a complex object of keys and values, it's now just a hex-encoded string of the hash.

#### `intent_compile` Function

* The `compile_transaction_intent` function has been renamed to `intent_compile` to be consistent with module naming scheme adopted with this version of the toolkit.
* The `compile_transaction_intent` output is no longer a complex object of keys and values, it's now just a hex-encoded string of the compiled intent.

#### `intent_decompile` Function

* The `decompile_transaction_intent` function has been renamed to `intent_decompile` to be consistent with module naming scheme adopted with this version of the toolkit.
* The `instructions_output_kind` field of the `intent_decompile` input has been renamed to `instructions_kind`.

#### `intent_statically_validate` Function

* Added a new function to the `intent` module that validates the intent statically: `intent_statically_validate`.

### `signed_intent` Module

#### `signed_intent_hash` Function

* The `hash_signed_transaction_intent` function has been renamed to `signed_intent_hash` to be consistent with module naming scheme adopted with this version of the toolkit.
* The `hash_signed_transaction_intent` output is no longer a complex object of keys and values, it's now just a hex-encoded string of the hash.

#### `signed_intent_compile` Function

* The `compile_signed_transaction_intent` function has been renamed to `signed_intent_compile` to be consistent with module naming scheme adopted with this version of the toolkit.
* The `compile_signed_transaction_intent` output is no longer a complex object of keys and values, it's now just a hex-encoded string of the compiled signed intent.

#### `signed_intent_decompile` Function

* The `decompile_signed_transaction_intent` function has been renamed to `signed_intent_decompile` to be consistent with module naming scheme adopted with this version of the toolkit.
* The `instructions_output_kind` field of the `signed_intent_decompile` input has been renamed to `instructions_kind`.

#### `signed_intent_statically_validate` Function

* Added a new function to the `signed_intent` module that validates the signed_intent statically: `signed_intent_statically_validate`.

### `notarized_transaction` Module

#### `notarized_transaction_hash` Function

* The `hash_notarized_transaction` function has been renamed to `notarized_transaction_hash` to be consistent with module naming scheme adopted with this version of the toolkit.
* The `hash_notarized_transaction` output is no longer a complex object of keys and values, it's now just a hex-encoded string of the hash.

#### `notarized_transaction_compile` Function

* The `compile_notarized_transaction` function has been renamed to `notarized_transaction_compile` to be consistent with module naming scheme adopted with this version of the toolkit.
* The `compile_notarized_transaction` output is no longer a complex object of keys and values, it's now just a hex-encoded string of the compiled signed intent.

#### `notarized_transaction_decompile` Function

* The `decompile_notarized_transaction` function has been renamed to `notarized_transaction_decompile` to be consistent with module naming scheme adopted with this version of the toolkit.
* The `instructions_output_kind` field of the `notarized_transaction_decompile` input has been renamed to `instructions_kind`.

#### `notarized_transaction_statically_validate` Function

* The `statically_validate_transaction` function has been renamed to `notarized_transaction_statically_validate` to be consistent with module naming scheme adopted with this version of the toolkit.
* The `compiled_notarized_intent` field of the `notarized_transaction_statically_validate` input has been renamed to `notarized_transaction` and is no longer a compiled notarized transaction.
* The `validation_config` struct has a new added field for message validation `message_validation`.




<!-- 


--------


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
* The value field has been renamed from `public_key` to `value`. -->