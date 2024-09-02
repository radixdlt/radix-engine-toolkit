# Changelog

This changelog captures the changes made to the toolkit between releases.

## Cuttlefish Protocol Update 

### `radix-engine-toolkit` Crate

* The `virtual_account_address_from_public_key` function has been renamed `preallocated_account_address_from_public_key`.
* The `virtual_identity_address_from_public_key` function has been renamed `preallocated_identity_address_from_public_key`.
* The `sbor_decode_to_native_event` function has been renamed to `scrypto_sbor_decode_to_native_event`.
* Moved the `instruction`, `manifest`, `intent`, `signed_intent`, and `notarized_transaction` functions to a `transaction_v1` module so that they're namespaced.
* Renamed all `compile` and `decompile` functions in `instruction`, `manifest`, `intent`, `signed_intent`, and `notarized_transaction` functions to `to_payload_bytes` and `from_payload_bytes` respectively.
* Added a new `transaction_v2` namespace with the modules `instructions`, `manifest`, `intent_core`, `subintent`, `signed_transaction_intent`, `transaction_intent`, `notarized_transaction` with functions for transactions v2.
* Removed the `modify` manifest function since it was unused by the toolkit's main clients.