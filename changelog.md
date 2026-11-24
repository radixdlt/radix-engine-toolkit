# Changelog

This changelog captures the changes made to the toolkit between releases.

## Unreleased

### `radix-engine-toolkit` Crate

* Added the `EntitySecurify` manifest classification. Its detailed output lists each securified account or identity with the rule set and timed recovery delay of the access controller created for it. Besides securifying and creating the access controllers, it only permits paying the fee: it doesn't permit setting metadata, including the entities' `owner_keys`, or withdrawing XRD and contributing to the recovery fee vaults of the access controllers it creates. Each is done in a transaction of its own, a top-up in a manifest of the `AccessControllerRecoveryFee` class. Each access controller must be created with exactly one owner badge, so every entity securified is in the output with the rules that control it.
* Added the `SecurifiedEntityCreation` manifest classification, for a manifest that creates accounts or identities with `create` and puts each one's owner badge into an access controller it creates, so they're securified from the start. Its detailed output lists each entity created, its address derived from its owner badge, with the rule set and timed recovery delay of its access controller. Like `EntitySecurify`, it only permits paying the fee besides that, and each access controller must be created with exactly one of the owner badges. Entities created with `create_advanced`, whose owner role isn't an owner badge, aren't permitted.
* Added five access controller manifest classifications. Besides their access controller calls, they only permit paying the fee and the subintent instructions of an enclosed subintent that yields nothing back, so a manifest of any of them withdraws nothing, contributes nothing and sets no metadata. A manifest is of at most one of them, and every access controller it calls must fit that one:
  * `AccessControllerRecovery`: on each access controller, one recovery is proposed and then quick confirmed with the same proposer, rule set and timed recovery delay, so the output lists exactly the rules enacted.
  * `AccessControllerTimedRecovery`: on each access controller, one recovery is proposed by the recovery role and not confirmed, optionally locking the primary role.
  * `AccessControllerConfirmRecovery`: on each access controller, a recovery already proposed is quick or timed confirmed. The output lists the rules the confirmation names, which the engine checks against the proposal.
  * `AccessControllerCancelRecovery`: on each access controller, the timed recovery is stopped and/or recovery proposals are cancelled, optionally unlocking the primary role.
  * `AccessControllerPrimaryRoleLock`: on each access controller, the primary role is locked or unlocked.
* Added the `AccessControllerRecoveryFee` manifest classification. The manifest either tops up the recovery fee vaults of access controllers with XRD withdrawn from accounts, depositing nothing, or withdraws XRD from them and deposits it into accounts, withdrawing nothing from accounts. Besides that, it only permits paying the fee. Its detailed output, from dynamic analysis, lists the XRD moved into or out of each access controller and account.
* A subintent of an access controller class, or of `AccessControllerRecoveryFee`, may not pay the fee, and may only yield to its parent as its last instructions. In a subintent a fee lock either fails or is a contingent one, a payment the output doesn't show, and yielding earlier would let the parent run its own instructions in between. Transaction manifests of these classes may still pay the fee, unless they start with `ASSERT_WORKTOP_IS_EMPTY`: that's what `as_enclosed` turns an enclosed subintent into, so they're held to the same rules.
* The timed recovery delay in the outputs is the one named in the proposal. The engine doesn't apply it when a recovery is confirmed: an access controller keeps the delay it was created with.
* Fixed `as_enclosed` treating a subintent as enclosed when its first instruction asserts that specific resources are on the worktop. It now returns `None` unless that assertion is an `ASSERT_WORKTOP_IS_EMPTY`.

### `radix-engine-toolkit-uniffi` Crate

* Added the `EntitySecurify`, `SecurifiedEntityCreation`, `AccessControllerRecovery`, `AccessControllerTimedRecovery`, `AccessControllerConfirmRecovery`, `AccessControllerCancelRecovery`, `AccessControllerPrimaryRoleLock` and `AccessControllerRecoveryFee` variants to `ManifestClassification` and `DetailedManifestClassification`, with their output records.

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
* All of the transaction v2 and v1 methods for static validation no longer take a validation config and now take in a network definition or id to validate against.

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
* Renamed all `compile` and `decompile` functions in `instruction`, `manifest`, `intent`, `signed_intent`, and `notarized_transaction` functions to `to_payload_bytes` and `from_payload_bytes` respectively.
* Renamed the `Message` functions `public_key_fingerprint_from_vec` and `public_key_fingerprint_to_vec` to `public_key_fingerprint_v1_from_vec` and `public_key_fingerprint_v1_to_vec`.
* Renamed `PublicKeyFingerprint` to `PublicKeyFingerprintV1`.
* All of the transaction v2 and v1 methods for static validation no longer take a validation config and now take in a network definition or id to validate against.
* Renamed the transaction builder and all of the associated types to include the version of the transactions that they're building. It is now clear that they build v1 or v2 transactions. As an example, the `TransactionBuilder` has been renamed to `TransactionV1Builder`.