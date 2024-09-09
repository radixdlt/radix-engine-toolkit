# Changelog

This changelog captures the changes made to the toolkit between releases.

## Cuttlefish Protocol Update 

### `radix-engine-toolkit` Crate 

This section is primarily relevant for internal integrators such as Sargon. The `radix-engine-toolkit` is not consumed by external integrators at the moment.

* The `virtual_account_address_from_public_key` function has been renamed `preallocated_account_address_from_public_key`.
* The `virtual_identity_address_from_public_key` function has been renamed `preallocated_identity_address_from_public_key`.
* The `sbor_decode_to_native_event` function has been renamed to `scrypto_sbor_decode_to_native_event`.
* Moved the `instruction`, `manifest`, `intent`, `signed_intent`, and `notarized_transaction` functions to a `transaction_v1` module so that they're namespaced.
* Renamed all `compile` and `decompile` functions in `instruction`, `manifest`, `intent`, `signed_intent`, and `notarized_transaction` functions to `to_payload_bytes` and `from_payload_bytes` respectively.
* Added a new `transaction_v2` namespace with the modules `instructions`, `manifest`, `intent_core`, `subintent`, `signed_transaction_intent`, `transaction_intent`, `notarized_transaction` with functions for transactions v2.
* Removed the `modify` manifest function since it was unused by the toolkit's main clients.
* Renamed the `summary` function to `statically_analyze`.
* Renamed the `execution_summary` function to `dynamically_analyze`.
* Renamed `ManifestSummary` to `StaticAnalysis`.
* Renamed `ExecutionSummary` to `DynamicAnalysis`.
* Added two new fields to the `ManifestSummary`: `account_withdraws` and `account_deposits`.
* Changed the `statically_analyze` to return an `Option` 

### `radix-engine-toolkit-uniffi` Crate (The UniFFI toolkit published in the various programming languages)

The changes here apply to the UniFFI toolkit which is the toolkit published in the various programming languages (C#, Python, Go, Kotlin, and Swift)

* The `RadixEngineToolkitError::ManifestModificationError` error variant has been removed with the removal of the modification functionality.
* A number of renames have taken place to accommodate for having v1 and v2 transactions:
    * `Instruction` => `InstructionV1`.
    * `Instructions` => `InstructionsV1`.
    * `TransactionHeader` => `TransactionHeaderV1`.
    * `TransactionManifest` => `TransactionManifestV1`.
    * `Intent` => `IntentV1`.
    * `SignedIntent` => `SignedIntentV1`.
    * `Message` => `MessageV1`.
    * `PlainTextMessage` => `PlainTextMessageV1`.
    * `EncryptedMessage` => `EncryptedMessageV1`.
    * `Signature` => `SignatureV1`.
    * `SignatureWithPublicKey` => `SignatureWithPublicKeyV1`.
    * `ManifestBuilder` => `ManifestV1Builder`.
* The `TransactionManifestV1` `summary` method has been renamed to `static_analysis`.
* The `TransactionManifestV1` `summary` method has been renamed to `static_analysis`.
* The `TransactionManifestV1` `static_analysis` method could now throw errors if the static analysis due to errors in the manifest.
* The `TransactionManifestV1` `execution_summary` method has been renamed to `dynamic_analysis`.
* Removed the `modify` method from the `TransactionManifestV1` type as well as all of the associated types.
* The `ExecutionSummary` type has been renamed to `DynamicAnalysis`.
* The `ManifestSummary` type has been renamed to `StaticAnalysis`.
* The `sbor_decode_to_native_event` function has been renamed to `scrypto_sbor_decode_to_native_event`.
* The `derive_virtual_account_address_from_public_key` function has been renamed to `derive_preallocated_account_address_from_public_key`.
* The `derive_virtual_identity_address_from_public_key` function has been renamed to `derive_preallocated_identity_address_from_public_key`.
* The `derive_virtual_identity_address_from_public_key` function has been renamed to `derive_signature_badge_non_fungible_global_id_from_public_key`.
* The `derive_virtual_signature_non_fungible_global_id_from_public_key` function has been renamed to `derive_signature_badge_non_fungible_global_id_from_public_key`.
* The `derive_virtual_account_address_from_olympia_account_address` function has been renamed to `derive_preallocated_account_address_from_olympia_account_address`.
* Renamed a number of the `Address` methods:
    * `virtual_account_address_from_public_key` => `preallocated_account_address_from_public_key`
    * `virtual_identity_address_from_public_key` => `preallocated_identity_address_from_public_key`
    * `virtual_account_address_from_olympia_address` => `preallocated_account_address_from_olympia_address`
    * `is_global_virtual` => `is_global_preallocated`