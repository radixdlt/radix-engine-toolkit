# 0.10.0 (6 July 2023)

* The instruction model of the `radix-engine-toolkit` crate no longer performs any aliasing of instructions. All aliasing is to be handled by transaction manifest builders.

# 0.10.0-damson.1 (26-June-2023)

No interface changes were made with this internal release, just internal changes to the toolkit as a result of bumping the version of the Scrypto Dependency.

# 0.10.0 (26-June-2023)

* All builds of the Radix Engine Toolkit now come with a `test-kit.tar.gz` file containing many test vectors to use when updating the toolkit wrappers to be compatible with the toolkit. More specifically, it includes the following:
    * The JSON Schema of all of the models used by the toolkit.
    * Test vectors of serialized models to include as part of the toolkit wrappers to ensure correct serialization/deserialization of models.
    * Test vectors for the input and output of functions to include as part of toolkit wrappers to ensure that the wrappers are able to invoke methods on the toolkit and receive expected responses.
* All across the Radix Engine Toolkit all types which are serialized as discriminated unions now have a discriminator called `kind` instead of `type` and a value field of `value`. There are some exceptions to this where the value is not a single value but multiple such as `SignatureWithPublicKey`.
* The Radix Engine Toolkit is now made up of modules. As an example, there is an `intent` module that has functions such as `intent_hash`, `intent_compile`, `intent_decompile`, and `intent_statically_validate`. 
    * All functions now follow the following naming scheme: `${module_name}_${function_name}`, thus, you can expect that all of the function names from the previous version of the toolkit has been renamed to adhere to this.
* The `network_id` is now being serialized as a `String` all across the toolkit in a consistent manner.

## Functions

* The `decompile_unknown_intent` function has been removed.
* The `encode_address` and `decode_address` functions have been **temporarily** removed.

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

* Added a new `instructions` module of functions that can be performed on manifest instructions.

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
* The `instructions_extract_addresses` output has changed, please refer to the JSON Schema and Function examples provided in the test kit for more information. 

### `manifest` Module

* Added a new `manifest` module of functions that can be performed on transaction manifests.

#### `manifest_hash` Function

* Added a new function to the `manifest` module that returns the hash of some given manifest: `manifest_hash`.

#### `manifest_compile` Function

* Added a new function to the `manifest` module that compiles the manifest by SBOR encoding it: `manifest_compile`.

#### `manifest_decompile` Function

* Added a new function to the `manifest` module that decompiles a byte array into manifest by SBOR decoding the byte array into manifest: `manifest_decompile`.

#### `manifest_statically_validate` Function

* Added a new function to the `manifest` module that validates the manifest statically: `manifest_statically_validate`.


### `intent` Module

* Added a new `intent` module of functions that can be performed on transaction intents.

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

* Added a new `signed_intent` module of functions that can be performed on signed transaction intents.

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

* Added a new `notarized_transaction` module of functions that can be performed on notarized transaction intents.

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

### `derive` Module

* Added a new `derive` module for all of the derivation operations that can be done by the toolkit.

#### `derive_virtual_account_address_from_public_key` Function

* The `derive_virtual_account_address` function has been renamed to `derive_virtual_account_address_from_public_key`: 
    * To be consistent with module naming scheme adopted with this version of the toolkit.
    * To have a more descriptive name of what it does exactly.
* The `derive_virtual_account_address_from_public_key` output is no longer a complex object of keys and values, it's now just a string of the virtual account address.

#### `derive_virtual_identity_address_from_public_key` Function

* The `derive_virtual_identity_address` function has been renamed to `derive_virtual_identity_address_from_public_key`: 
    * To be consistent with module naming scheme adopted with this version of the toolkit.
    * To have a more descriptive name of what it does exactly.
* The `derive_virtual_identity_address_from_public_key` output is no longer a complex object of keys and values, it's now just a string of the virtual identity address.

#### `derive_virtual_signature_non_fungible_global_id_from_public_key` Function

* Added a new function to the `derive` module that derives the non-fungible global id of the virtual signature badge given a public key: `derive_virtual_signature_non_fungible_global_id_from_public_key`.

#### `derive_virtual_account_address_from_olympia_account_address` Function

* The `derive_babylon_address_from_olympia_address` function has been renamed to `derive_virtual_account_address_from_olympia_account_address`: 
    * To be consistent with module naming scheme adopted with this version of the toolkit.
    * To have a more descriptive name of what it does exactly.
* The `derive_virtual_account_address_from_olympia_account_address` output no longer includes the public key associated with the Olympia account, just a string of the Babylon account address. The public key can be derived from an olympia account address through the `derive_public_key_from_olympia_account_address` function.

#### `derive_public_key_from_olympia_account_address` Function

* Added a new function to the `derive` module that derives the public key given an Olympia account address: `derive_public_key_from_olympia_account_address`.
* The output of this function is a string and not a `PublicKey` because Olympia only supports Ecdsa Secp256k1 and there is no need to have a discriminated union when we know what the type is.

#### `derive_resource_address_from_olympia_resource_address` Function

* Added a new function to the `derive` module that derives the address of a resource from the Olympia network on the Babylon network: `derive_resource_address_from_olympia_resource_address`.

#### `derive_olympia_account_address_from_public_key` Function

* The `network` field of the `derive_olympia_account_address_from_public_key` input has been renamed to `olympia_network`.
* The `derive_olympia_account_address_from_public_key` output is no longer a complex object of keys and values, it's now just a string of the address.

#### `derive_node_address_from_public_key` Function

* Added a new function to the `derive` module that derives the node address from a public key.

### `execution` Module

* Added a new `execution` module for everything related to transaction execution and analysis.

#### `execution_analyze` Function

* The `analyze_transaction_execution` function has been renamed to `execution_analyze` to be consistent with module naming scheme adopted with this version of the toolkit.
* The `transaction_receipt` field of the `execution_analyze` input has been renamed to `preview_receipt`.
* The `manifest` field of the `execution_analyze` input has been:
    * Renamed to `instructions`.
    * Only requires the instructions and not a complete `TransactionManifest`.
* The `execution_analyze` output has completely changed, please refer to the JSON Schema and function examples in the test kit for more information.

### `utils` Module

* Added a new `utils` module for miscellaneous functions needed by clients.

#### `utils_known_address`

* The `known_entity_addresses` function has been renamed to `utils_known_address` to be consistent with module naming scheme adopted with this version of the toolkit.
* The `utils_known_address` input is no longer a complex object of keys and values, it's just a string of the network id.
* The `utils_known_address` output has completely changed, please refer to the JSON Schema and function examples in the test kit for more information.

### `manifest_sbor` Module

* Added a `manifest_sbor` module for all manifest SBOR related operations.

#### `manifest_sbor_decode_to_string` Function

* Added a new function to the `manifest_sbor` module for decoding an manifest SBOR payload to a string either with or without a schema, additionally, this function can output the manifest representation, programmatic JSON, model JSON, and natural JSON representation of the payload: `manifest_sbor_decode_to_string`.

### `scrypto_sbor` Module

* Added a `scrypto_sbor` module for all scrypto SBOR related operations.

#### `scrypto_sbor_decode_to_string` Function

* Added a new function to the `scrypto_sbor` module for decoding an scrypto SBOR payload to a string either with or without a schema, additionally, this function can output the programmatic JSON, model JSON, and natural JSON representation of the payload: `scrypto_sbor_decode_to_string`.

## Models

### Transaction

* The `TransactionIntent` model has an additional `message` field for messages. The structure of the `message` field can be found in the JSON Schema provided in the test-kit.

### Cryptographic

* The `PublicKeyHash` discriminator has been renamed to `kind`.
* The `PublicKeyHash` variants have been shortened to be `Secp256k1` and `Ed25519` respectively.
* The `PublicKey` discriminator has been renamed to `kind`.
* The `PublicKey` variants have been shortened to be `Secp256k1` and `Ed25519` respectively.
* The `SignatureWithPublicKey` discriminator has been renamed to `kind`.
* The `SignatureWithPublicKey` variants have been shortened to be `Secp256k1` and `Ed25519` respectively.
* The `Signature` discriminator has been renamed to `kind`.
* The `Signature` variants have been shortened to be `Secp256k1` and `Ed25519` respectively.

### Instruction

* The `Instruction` discriminator has been renamed to `kind`.
* The `Instruction` variant names are no longer SCREAMING_SNAKE_CASE and are now in PascalCase.

### Value Models

* The `ManifestAstValue` model has been renamed to `ManifestValue`.
* All type aliases have been removed from the `ManifestValue` model, namely, `Some`, `None`, `Err`, `Ok`.
* Enums in the `ManifestValue` model only accept u8 discriminators. Clients that wish to use named discriminators are free to define that mapping in the wrapper. 
