<div align="center">
  <h1><code>Radix Engine Toolkit</code></h1>
  <p>
    <strong>A Python wrapper around the Radix Engine Toolkit that provides Radix Ledger primitives to Python</strong>
  </p>

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
</div>

The (Python) Radix Engine Toolkit is a wrapper around the [Radix Engine Toolkit](https://github.com/radixdlt/radix-engine-toolkit/) library which exposes the Radix Engine and Scrypto primitives to Python. These primitives include: manifests, transactions, transaction construction and building, access rules, metadata, the SBOR codec, derivations, and many others. The purpose this and the other wrappers around the toolkit is to provide developers with the freedom of constructing transactions and interacting with the ledger in their language of their choice instead of being limited to using Rust <!-- Even though I really think you should learn and use Rust!. -->.

This library uses [UniFFI](https://github.com/mozilla/uniffi-rs) for interoperability between the core Rust Radix Engine Toolkit and Radix Engine Toolkit wrappers such as this Python wrapper. Thus, the entire codebase of this library is automatically generated and the Python code does not live in a repo by itself. Instead, this library is published directly to PyPi with each push that's made to the [Radix Engine Toolkit](https://github.com/radixdlt/radix-engine-toolkit/) repo. If you would like to submit an issue or open a PR then head to: https://github.com/radixdlt/radix-engine-toolkit/

# Example Usage

```py
from radix_engine_toolkit import *
from typing import Tuple
import secrets


def new_account(network_id: int) -> Tuple[PrivateKey, PublicKey, Address]:
    """
    Creates a new random Ed25519 private key and then derives the public key and
    the account address associated with it
    """
    private_key_bytes: bytes = secrets.randbits(256).to_bytes(32)
    private_key: PrivateKey = PrivateKey.new_ed25519(list(private_key_bytes))
    public_key: PublicKey = private_key.public_key()
    account: Address = derive_virtual_account_address_from_public_key(
        public_key, network_id
    )
    return (private_key, public_key, account)


def random_nonce() -> int:
    """
    Generates a random secure random number between 0 and 0xFFFFFFFF (u32::MAX)
    """
    return secrets.randbelow(0xFFFFFFFF)


# A constant of the id of the network
NETWORK_ID: int = 0x02

(private_key1, public_key1, account1) = new_account(NETWORK_ID)
(private_key2, public_key2, account2) = new_account(NETWORK_ID)

print(f"Address of account 1: {account1.as_str()}")
print(f"Address of account 1: {account2.as_str()}")

# Constructing a transaction that gets 10_000 XRD from the faucet and sends
# half to account1 and half to account2
address_book: KnownAddresses = known_addresses(NETWORK_ID)
faucet_address: Address = address_book.component_addresses.faucet
xrd_address: Address = address_book.resource_addresses.xrd

manifest: TransactionManifest = (
    ManifestBuilder()
    .faucet_lock_fee()
    .faucet_free_xrd()
    .take_from_worktop(xrd_address, Decimal("5000"), ManifestBuilderBucket("bucket1"))
    .take_from_worktop(xrd_address, Decimal("5000"), ManifestBuilderBucket("bucket2"))
    .account_deposit(account1, ManifestBuilderBucket("bucket1"))
    .account_deposit(account2, ManifestBuilderBucket("bucket2"))
    .build(NETWORK_ID)
)
header: TransactionHeader = TransactionHeader(
    network_id=NETWORK_ID,
    # For the start and end epochs you will need to query the Gateway/Core APIs
    # for the current epoch and use it here. Or use some other epoch. What I'm
    # trying to say is, there is no way for the Radix Engine Toolkit to provide
    # you with the epoch information.
    start_epoch_inclusive=200,
    end_epoch_exclusive=210,
    nonce=random_nonce(),
    notary_public_key=public_key2,
    notary_is_signatory=True,
    tip_percentage=0,
)

transaction: NotarizedTransaction = (
    TransactionBuilder()
    .header(header)
    .manifest(manifest)
    .sign_with_private_key(private_key1)
    .notarize_with_private_key(private_key2)
)

# Ensure that the transaction is statically valid - if the validation fails an
# exception will be raised.
transaction.statically_validate(ValidationConfig.default(NETWORK_ID))
print(f"Transaction Hash: {transaction.intent_hash().as_str()}")

# Note: at this point, we have created the created the transaction and validated
# it statically ensuring that no simple errors might be there. The RET does not
# submit transactions on your behalf to the network, it's your responsibility to
# do that. You can do that through either the gateway or core API.
print(
    f"Transaction Payload to submit to the network: {bytearray(transaction.compile()).hex()}"
)
```

# License

The Python Radix Engine Toolkit code is released under the [Apache 2.0 license](./LICENSE). Binaries are licensed under the [Radix Generic EULA](https://www.radixdlt.com/terms/genericEULA).