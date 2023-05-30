# Requests Examples

This document contains examples and descriptions of the different requests and responses which
the Radix Engine Toolkit may provide. As long as all of the CI test pass, then you may treat this
document as the canonical truth for the format of the different requests and as valid examples of
the payload and responses of these requests.


## Information

| Function Name     | `information` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_information` |
| Functionality     | The function provides information information on the currently in-use radix engine</br>toolkit such as the version of the radix engine toolkit. In most cases, this is the first</br>function written when integrating new clients; so, this function is often times seen as the</br>"Hello World" example of the radix engine toolkit.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "package_version": "0.9.0",
  "last_commit_hash": "This is just an example. We don't have a commit hash here"
}
```
</details>

## Convert Manifest

| Function Name     | `convert_manifest` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_convertManifest` |
| Functionality     | Clients have a need to be able to read, parse, understand, and interrogate transaction</br>manifests to get more information on what a transactions might be doing. Transaction manifests</br>have so far existed in one format: as strings. While the string format is very human readable, it</br>is not easily readable by machines as a lexer and parser are needed to make sense of them; thus,</br>it is for clients to programmatically make sense of transactions. As such, there is a need for</br>another transaction manifest format (to supplement, NOT replace) which machines can easily make</br>sense of without the need to implement a lexer and parser.</br></br>Therefore, this library introduces a Parsed format for transaction manifests which clients can</br>use when wanting to read and interrogate their transaction manifests in code. The transaction</br>manifest Parsed format has a 1:1 mapping to the string format of transaction manifests, meaning</br>that anything which can be done in the string format of transaction manifests, can be done in the</br>Parsed format as well.</br></br>This function allows the client the convert their manifest between the two supported manifest</br>types: string and Parsed.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "network_id": "242",
  "instructions_output_kind": "Parsed",
  "manifest": {
    "instructions": {
      "type": "Parsed",
      "value": [
        {
          "instruction": "DROP_ALL_PROOFS"
        }
      ]
    },
    "blobs": []
  }
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "instructions": {
    "type": "Parsed",
    "value": [
      {
        "instruction": "DROP_ALL_PROOFS"
      }
    ]
  },
  "blobs": []
}
```
</details>

## Extract Addresses From Manifest

| Function Name     | `extract_addresses_from_manifest` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_extractAddressesFromManifest` |
| Functionality     | Analyzes the manifest returning back all of the addresses involved in the manifest</br>alongside some useful information on whether the accounts were withdrawn from, deposited into, or</br>just used in the manifest in general.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "network_id": "242",
  "manifest": {
    "instructions": {
      "type": "String",
      "value": "DROP_ALL_PROOFS;\n"
    },
    "blobs": []
  }
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "package_addresses": [],
  "component_addresses": [],
  "resource_addresses": [],
  "account_addresses": [],
  "accounts_requiring_auth": [],
  "accounts_withdrawn_from": [],
  "accounts_deposited_into": [],
  "identity_addresses": [],
  "identities_requiring_auth": []
}
```
</details>

## Compile Transaction Intent

| Function Name     | `compile_transaction_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_compileTransactionIntent` |
| Functionality     | Takes a transaction intent and compiles it by SBOR encoding it and returning it back</br>to the caller. This is mainly useful when creating a transaction.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "header": {
    "network_id": "242",
    "start_epoch_inclusive": "512",
    "end_epoch_exclusive": "528",
    "nonce": "34",
    "notary_public_key": {
      "curve": "EcdsaSecp256k1",
      "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
    },
    "notary_is_signatory": true,
    "tip_percentage": "0"
  },
  "manifest": {
    "instructions": {
      "type": "Parsed",
      "value": [
        {
          "instruction": "DROP_ALL_PROOFS"
        }
      ]
    },
    "blobs": []
  }
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "compiled_intent": "4d2104210707f20a00020000000000000a1002000000000000092200000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21010108000020220150002020002100"
}
```
</details>

## Decompile Transaction Intent

| Function Name     | `decompile_transaction_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_decompileTransactionIntent` |
| Functionality     | This function does the opposite of the compile_transaction_intent function. It takes</br>in a compiled transaction intent and decompiles it into its human-readable / machine-readable</br>format.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "instructions_output_kind": "Parsed",
  "compiled_intent": "4d2104210707f20a00020000000000000a1002000000000000092200000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21010108000020220150002020002100"
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "header": {
    "network_id": "242",
    "start_epoch_inclusive": "512",
    "end_epoch_exclusive": "528",
    "nonce": "34",
    "notary_public_key": {
      "curve": "EcdsaSecp256k1",
      "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
    },
    "notary_is_signatory": true,
    "tip_percentage": "0"
  },
  "manifest": {
    "instructions": {
      "type": "Parsed",
      "value": [
        {
          "instruction": "DROP_ALL_PROOFS"
        }
      ]
    },
    "blobs": []
  }
}
```
</details>

## Compile Signed Transaction Intent

| Function Name     | `compile_signed_transaction_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_compileSignedTransactionIntent` |
| Functionality     | This function takes in a raw transaction intent as well as its signatures and compiles</br>it. This is useful when a notary wishes to notarize a signed transaction intent.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "intent": {
    "header": {
      "network_id": "242",
      "start_epoch_inclusive": "512",
      "end_epoch_exclusive": "528",
      "nonce": "34",
      "notary_public_key": {
        "curve": "EcdsaSecp256k1",
        "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
      },
      "notary_is_signatory": true,
      "tip_percentage": "0"
    },
    "manifest": {
      "instructions": {
        "type": "Parsed",
        "value": [
          {
            "instruction": "DROP_ALL_PROOFS"
          }
        ]
      },
      "blobs": []
    }
  },
  "intent_signatures": [
    {
      "curve": "EcdsaSecp256k1",
      "signature": "00fc8f9ad3d9a3376714e0b941add423a7ea3717787b9fff1d56856a33abaec2a9646ccb17348591dac6b0dc8fb34c6214e6ac3a93d9634d6978a55e7c4a20857a"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "0160572255d7202af33b63161d97e2952a095c9d223833c4944654225c7c9048297a9f0b8175dd1a3a9ee251390349bd42474e12b35d821d63ef4a1f3555212d2f"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "00bab73855b939c69de92c6599d8145f5e17ef8e10cae8746ce857e8a513f6674e5f7a9fcc493ff47c572acc71b2e23849ae77d563a536462b447d85b949897b02"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
      "signature": "43c6aed201f894ca326106fe13e2f6d633c48dfd8610c53d07fb82cf771050a7b9461a9ed062eacf104be737d422b91d5896625deb09aee564cdd70f56f2ee02"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
      "signature": "8d234c5e6d4a354f8232834501bef42cdf847e9aa5c4995505be4d67982d8deb27a8f26306561062bf10833e0ed4313656ceed03a03d85a47224245cf471080f"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
      "signature": "fb2f59c11fc7a720b76521e461f8ae77803abea6c1acc767c2bde8a1f8141f75a1cf6b10ed8dd4a4f663cd6ce0571a3029cdd2021cc2f426ba356cef75c41c00"
    }
  ]
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "compiled_intent": "4d21022104210707f20a00020000000000000a1002000000000000092200000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa210101080000202201500020200021002022060001210120074100fc8f9ad3d9a3376714e0b941add423a7ea3717787b9fff1d56856a33abaec2a9646ccb17348591dac6b0dc8fb34c6214e6ac3a93d9634d6978a55e7c4a20857a000121012007410160572255d7202af33b63161d97e2952a095c9d223833c4944654225c7c9048297a9f0b8175dd1a3a9ee251390349bd42474e12b35d821d63ef4a1f3555212d2f0001210120074100bab73855b939c69de92c6599d8145f5e17ef8e10cae8746ce857e8a513f6674e5f7a9fcc493ff47c572acc71b2e23849ae77d563a536462b447d85b949897b0201022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29210120074043c6aed201f894ca326106fe13e2f6d633c48dfd8610c53d07fb82cf771050a7b9461a9ed062eacf104be737d422b91d5896625deb09aee564cdd70f56f2ee0201022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe267421012007408d234c5e6d4a354f8232834501bef42cdf847e9aa5c4995505be4d67982d8deb27a8f26306561062bf10833e0ed4313656ceed03a03d85a47224245cf471080f0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740fb2f59c11fc7a720b76521e461f8ae77803abea6c1acc767c2bde8a1f8141f75a1cf6b10ed8dd4a4f663cd6ce0571a3029cdd2021cc2f426ba356cef75c41c00"
}
```
</details>

## Decompile Signed Transaction Intent

| Function Name     | `decompile_signed_transaction_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_decompileSignedTransactionIntent` |
| Functionality     | This function does the opposite of the compile_signed_transaction_intent function.</br>This function takes in a compiled signed transaction intent and decompiles it into its</br>transaction intent and signatures.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "instructions_output_kind": "Parsed",
  "compiled_signed_intent": "4d21022104210707f20a00020000000000000a1002000000000000092200000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa210101080000202201500020200021002022060001210120074100fc8f9ad3d9a3376714e0b941add423a7ea3717787b9fff1d56856a33abaec2a9646ccb17348591dac6b0dc8fb34c6214e6ac3a93d9634d6978a55e7c4a20857a000121012007410160572255d7202af33b63161d97e2952a095c9d223833c4944654225c7c9048297a9f0b8175dd1a3a9ee251390349bd42474e12b35d821d63ef4a1f3555212d2f0001210120074100bab73855b939c69de92c6599d8145f5e17ef8e10cae8746ce857e8a513f6674e5f7a9fcc493ff47c572acc71b2e23849ae77d563a536462b447d85b949897b0201022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29210120074043c6aed201f894ca326106fe13e2f6d633c48dfd8610c53d07fb82cf771050a7b9461a9ed062eacf104be737d422b91d5896625deb09aee564cdd70f56f2ee0201022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe267421012007408d234c5e6d4a354f8232834501bef42cdf847e9aa5c4995505be4d67982d8deb27a8f26306561062bf10833e0ed4313656ceed03a03d85a47224245cf471080f0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740fb2f59c11fc7a720b76521e461f8ae77803abea6c1acc767c2bde8a1f8141f75a1cf6b10ed8dd4a4f663cd6ce0571a3029cdd2021cc2f426ba356cef75c41c00"
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "intent": {
    "header": {
      "network_id": "242",
      "start_epoch_inclusive": "512",
      "end_epoch_exclusive": "528",
      "nonce": "34",
      "notary_public_key": {
        "curve": "EcdsaSecp256k1",
        "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
      },
      "notary_is_signatory": true,
      "tip_percentage": "0"
    },
    "manifest": {
      "instructions": {
        "type": "Parsed",
        "value": [
          {
            "instruction": "DROP_ALL_PROOFS"
          }
        ]
      },
      "blobs": []
    }
  },
  "intent_signatures": [
    {
      "curve": "EcdsaSecp256k1",
      "signature": "00fc8f9ad3d9a3376714e0b941add423a7ea3717787b9fff1d56856a33abaec2a9646ccb17348591dac6b0dc8fb34c6214e6ac3a93d9634d6978a55e7c4a20857a"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "0160572255d7202af33b63161d97e2952a095c9d223833c4944654225c7c9048297a9f0b8175dd1a3a9ee251390349bd42474e12b35d821d63ef4a1f3555212d2f"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "00bab73855b939c69de92c6599d8145f5e17ef8e10cae8746ce857e8a513f6674e5f7a9fcc493ff47c572acc71b2e23849ae77d563a536462b447d85b949897b02"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
      "signature": "43c6aed201f894ca326106fe13e2f6d633c48dfd8610c53d07fb82cf771050a7b9461a9ed062eacf104be737d422b91d5896625deb09aee564cdd70f56f2ee02"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
      "signature": "8d234c5e6d4a354f8232834501bef42cdf847e9aa5c4995505be4d67982d8deb27a8f26306561062bf10833e0ed4313656ceed03a03d85a47224245cf471080f"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
      "signature": "fb2f59c11fc7a720b76521e461f8ae77803abea6c1acc767c2bde8a1f8141f75a1cf6b10ed8dd4a4f663cd6ce0571a3029cdd2021cc2f426ba356cef75c41c00"
    }
  ]
}
```
</details>

## Compile Notarized Transaction

| Function Name     | `compile_notarized_transaction` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_compileNotarizedTransaction` |
| Functionality     | This function takes in a raw signed transaction intent as well as the notary signature</br>and compiles it. This is useful when we wish to submit a transaction to the Gateway</br>API   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "signed_intent": {
    "intent": {
      "header": {
        "network_id": "242",
        "start_epoch_inclusive": "512",
        "end_epoch_exclusive": "528",
        "nonce": "34",
        "notary_public_key": {
          "curve": "EcdsaSecp256k1",
          "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
        },
        "notary_is_signatory": true,
        "tip_percentage": "0"
      },
      "manifest": {
        "instructions": {
          "type": "Parsed",
          "value": [
            {
              "instruction": "DROP_ALL_PROOFS"
            }
          ]
        },
        "blobs": []
      }
    },
    "intent_signatures": [
      {
        "curve": "EcdsaSecp256k1",
        "signature": "00fc8f9ad3d9a3376714e0b941add423a7ea3717787b9fff1d56856a33abaec2a9646ccb17348591dac6b0dc8fb34c6214e6ac3a93d9634d6978a55e7c4a20857a"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "0160572255d7202af33b63161d97e2952a095c9d223833c4944654225c7c9048297a9f0b8175dd1a3a9ee251390349bd42474e12b35d821d63ef4a1f3555212d2f"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "00bab73855b939c69de92c6599d8145f5e17ef8e10cae8746ce857e8a513f6674e5f7a9fcc493ff47c572acc71b2e23849ae77d563a536462b447d85b949897b02"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
        "signature": "43c6aed201f894ca326106fe13e2f6d633c48dfd8610c53d07fb82cf771050a7b9461a9ed062eacf104be737d422b91d5896625deb09aee564cdd70f56f2ee02"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
        "signature": "8d234c5e6d4a354f8232834501bef42cdf847e9aa5c4995505be4d67982d8deb27a8f26306561062bf10833e0ed4313656ceed03a03d85a47224245cf471080f"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
        "signature": "fb2f59c11fc7a720b76521e461f8ae77803abea6c1acc767c2bde8a1f8141f75a1cf6b10ed8dd4a4f663cd6ce0571a3029cdd2021cc2f426ba356cef75c41c00"
      }
    ]
  },
  "notary_signature": {
    "curve": "EcdsaSecp256k1",
    "signature": "003586f438502f8789127d694367c9abfb7d2cd846e66d0e7f2d36d69214ac4c7e3f827696c4e5b70d1d638984fb04d4faf87383c4891e5d3fefb66a86ad948b61"
  }
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "compiled_intent": "4d210221022104210707f20a00020000000000000a1002000000000000092200000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa210101080000202201500020200021002022060001210120074100fc8f9ad3d9a3376714e0b941add423a7ea3717787b9fff1d56856a33abaec2a9646ccb17348591dac6b0dc8fb34c6214e6ac3a93d9634d6978a55e7c4a20857a000121012007410160572255d7202af33b63161d97e2952a095c9d223833c4944654225c7c9048297a9f0b8175dd1a3a9ee251390349bd42474e12b35d821d63ef4a1f3555212d2f0001210120074100bab73855b939c69de92c6599d8145f5e17ef8e10cae8746ce857e8a513f6674e5f7a9fcc493ff47c572acc71b2e23849ae77d563a536462b447d85b949897b0201022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29210120074043c6aed201f894ca326106fe13e2f6d633c48dfd8610c53d07fb82cf771050a7b9461a9ed062eacf104be737d422b91d5896625deb09aee564cdd70f56f2ee0201022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe267421012007408d234c5e6d4a354f8232834501bef42cdf847e9aa5c4995505be4d67982d8deb27a8f26306561062bf10833e0ed4313656ceed03a03d85a47224245cf471080f0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740fb2f59c11fc7a720b76521e461f8ae77803abea6c1acc767c2bde8a1f8141f75a1cf6b10ed8dd4a4f663cd6ce0571a3029cdd2021cc2f426ba356cef75c41c002200012101200741003586f438502f8789127d694367c9abfb7d2cd846e66d0e7f2d36d69214ac4c7e3f827696c4e5b70d1d638984fb04d4faf87383c4891e5d3fefb66a86ad948b61"
}
```
</details>

## Decompile Notarized Transaction

| Function Name     | `decompile_notarized_transaction` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_decompileNotarizedTransaction` |
| Functionality     | This function does the opposite of the compile_notarized_intent()_intent function.</br>This function takes in a compiled notarized transaction intent and decompiles it into its signed</br>transaction intent and notary signature.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "instructions_output_kind": "Parsed",
  "compiled_notarized_intent": "4d210221022104210707f20a00020000000000000a1002000000000000092200000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa210101080000202201500020200021002022060001210120074100fc8f9ad3d9a3376714e0b941add423a7ea3717787b9fff1d56856a33abaec2a9646ccb17348591dac6b0dc8fb34c6214e6ac3a93d9634d6978a55e7c4a20857a000121012007410160572255d7202af33b63161d97e2952a095c9d223833c4944654225c7c9048297a9f0b8175dd1a3a9ee251390349bd42474e12b35d821d63ef4a1f3555212d2f0001210120074100bab73855b939c69de92c6599d8145f5e17ef8e10cae8746ce857e8a513f6674e5f7a9fcc493ff47c572acc71b2e23849ae77d563a536462b447d85b949897b0201022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29210120074043c6aed201f894ca326106fe13e2f6d633c48dfd8610c53d07fb82cf771050a7b9461a9ed062eacf104be737d422b91d5896625deb09aee564cdd70f56f2ee0201022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe267421012007408d234c5e6d4a354f8232834501bef42cdf847e9aa5c4995505be4d67982d8deb27a8f26306561062bf10833e0ed4313656ceed03a03d85a47224245cf471080f0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740fb2f59c11fc7a720b76521e461f8ae77803abea6c1acc767c2bde8a1f8141f75a1cf6b10ed8dd4a4f663cd6ce0571a3029cdd2021cc2f426ba356cef75c41c002200012101200741003586f438502f8789127d694367c9abfb7d2cd846e66d0e7f2d36d69214ac4c7e3f827696c4e5b70d1d638984fb04d4faf87383c4891e5d3fefb66a86ad948b61"
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "signed_intent": {
    "intent": {
      "header": {
        "network_id": "242",
        "start_epoch_inclusive": "512",
        "end_epoch_exclusive": "528",
        "nonce": "34",
        "notary_public_key": {
          "curve": "EcdsaSecp256k1",
          "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
        },
        "notary_is_signatory": true,
        "tip_percentage": "0"
      },
      "manifest": {
        "instructions": {
          "type": "Parsed",
          "value": [
            {
              "instruction": "DROP_ALL_PROOFS"
            }
          ]
        },
        "blobs": []
      }
    },
    "intent_signatures": [
      {
        "curve": "EcdsaSecp256k1",
        "signature": "00fc8f9ad3d9a3376714e0b941add423a7ea3717787b9fff1d56856a33abaec2a9646ccb17348591dac6b0dc8fb34c6214e6ac3a93d9634d6978a55e7c4a20857a"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "0160572255d7202af33b63161d97e2952a095c9d223833c4944654225c7c9048297a9f0b8175dd1a3a9ee251390349bd42474e12b35d821d63ef4a1f3555212d2f"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "00bab73855b939c69de92c6599d8145f5e17ef8e10cae8746ce857e8a513f6674e5f7a9fcc493ff47c572acc71b2e23849ae77d563a536462b447d85b949897b02"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
        "signature": "43c6aed201f894ca326106fe13e2f6d633c48dfd8610c53d07fb82cf771050a7b9461a9ed062eacf104be737d422b91d5896625deb09aee564cdd70f56f2ee02"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
        "signature": "8d234c5e6d4a354f8232834501bef42cdf847e9aa5c4995505be4d67982d8deb27a8f26306561062bf10833e0ed4313656ceed03a03d85a47224245cf471080f"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
        "signature": "fb2f59c11fc7a720b76521e461f8ae77803abea6c1acc767c2bde8a1f8141f75a1cf6b10ed8dd4a4f663cd6ce0571a3029cdd2021cc2f426ba356cef75c41c00"
      }
    ]
  },
  "notary_signature": {
    "curve": "EcdsaSecp256k1",
    "signature": "003586f438502f8789127d694367c9abfb7d2cd846e66d0e7f2d36d69214ac4c7e3f827696c4e5b70d1d638984fb04d4faf87383c4891e5d3fefb66a86ad948b61"
  }
}
```
</details>

## Decompile Unknown Intent

| Function Name     | `decompile_unknown_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_decompileUnknownIntent` |
| Functionality     | There are certain cases where we might have some blob which we suspect is a</br>transaction intent but we have no way of verifying whether that is true or not. Looking at the</br>type id byte of the blob does not help either as it's a generic Struct type which is not too</br>telling. For this specific use case, this library provides this function which attempts to</br>decompile a transaction intent of an unknown type.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "instructions_output_kind": "Parsed",
  "compiled_unknown_intent": "4d210221022104210707f20a00020000000000000a1002000000000000092200000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa210101080000202201500020200021002022060001210120074100fc8f9ad3d9a3376714e0b941add423a7ea3717787b9fff1d56856a33abaec2a9646ccb17348591dac6b0dc8fb34c6214e6ac3a93d9634d6978a55e7c4a20857a000121012007410160572255d7202af33b63161d97e2952a095c9d223833c4944654225c7c9048297a9f0b8175dd1a3a9ee251390349bd42474e12b35d821d63ef4a1f3555212d2f0001210120074100bab73855b939c69de92c6599d8145f5e17ef8e10cae8746ce857e8a513f6674e5f7a9fcc493ff47c572acc71b2e23849ae77d563a536462b447d85b949897b0201022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29210120074043c6aed201f894ca326106fe13e2f6d633c48dfd8610c53d07fb82cf771050a7b9461a9ed062eacf104be737d422b91d5896625deb09aee564cdd70f56f2ee0201022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe267421012007408d234c5e6d4a354f8232834501bef42cdf847e9aa5c4995505be4d67982d8deb27a8f26306561062bf10833e0ed4313656ceed03a03d85a47224245cf471080f0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740fb2f59c11fc7a720b76521e461f8ae77803abea6c1acc767c2bde8a1f8141f75a1cf6b10ed8dd4a4f663cd6ce0571a3029cdd2021cc2f426ba356cef75c41c002200012101200741003586f438502f8789127d694367c9abfb7d2cd846e66d0e7f2d36d69214ac4c7e3f827696c4e5b70d1d638984fb04d4faf87383c4891e5d3fefb66a86ad948b61"
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "type": "NotarizedTransactionIntent",
  "value": {
    "signed_intent": {
      "intent": {
        "header": {
          "network_id": "242",
          "start_epoch_inclusive": "512",
          "end_epoch_exclusive": "528",
          "nonce": "34",
          "notary_public_key": {
            "curve": "EcdsaSecp256k1",
            "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
          },
          "notary_is_signatory": true,
          "tip_percentage": "0"
        },
        "manifest": {
          "instructions": {
            "type": "Parsed",
            "value": [
              {
                "instruction": "DROP_ALL_PROOFS"
              }
            ]
          },
          "blobs": []
        }
      },
      "intent_signatures": [
        {
          "curve": "EcdsaSecp256k1",
          "signature": "00fc8f9ad3d9a3376714e0b941add423a7ea3717787b9fff1d56856a33abaec2a9646ccb17348591dac6b0dc8fb34c6214e6ac3a93d9634d6978a55e7c4a20857a"
        },
        {
          "curve": "EcdsaSecp256k1",
          "signature": "0160572255d7202af33b63161d97e2952a095c9d223833c4944654225c7c9048297a9f0b8175dd1a3a9ee251390349bd42474e12b35d821d63ef4a1f3555212d2f"
        },
        {
          "curve": "EcdsaSecp256k1",
          "signature": "00bab73855b939c69de92c6599d8145f5e17ef8e10cae8746ce857e8a513f6674e5f7a9fcc493ff47c572acc71b2e23849ae77d563a536462b447d85b949897b02"
        },
        {
          "curve": "EddsaEd25519",
          "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
          "signature": "43c6aed201f894ca326106fe13e2f6d633c48dfd8610c53d07fb82cf771050a7b9461a9ed062eacf104be737d422b91d5896625deb09aee564cdd70f56f2ee02"
        },
        {
          "curve": "EddsaEd25519",
          "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
          "signature": "8d234c5e6d4a354f8232834501bef42cdf847e9aa5c4995505be4d67982d8deb27a8f26306561062bf10833e0ed4313656ceed03a03d85a47224245cf471080f"
        },
        {
          "curve": "EddsaEd25519",
          "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
          "signature": "fb2f59c11fc7a720b76521e461f8ae77803abea6c1acc767c2bde8a1f8141f75a1cf6b10ed8dd4a4f663cd6ce0571a3029cdd2021cc2f426ba356cef75c41c00"
        }
      ]
    },
    "notary_signature": {
      "curve": "EcdsaSecp256k1",
      "signature": "003586f438502f8789127d694367c9abfb7d2cd846e66d0e7f2d36d69214ac4c7e3f827696c4e5b70d1d638984fb04d4faf87383c4891e5d3fefb66a86ad948b61"
    }
  }
}
```
</details>

## Encode Address

| Function Name     | `encode_address` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_encodeAddress` |
| Functionality     | This function can be used when we have a byte array which we wish to do Bech32m</br>encoding on. In this case, the HRP to use will be determined through the entity byte of the</br>passed address hex string.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "address_bytes": "0d906318c6318c6c4d63f8cc6318c6318cf7bf553d3ca51686318c6318c6",
  "network_id": "242"
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "address": "package_sim1pkgxxxxxxxxxcntrlrxxxxxxxxx000648572295xxxxxxxxxxc5z0l"
}
```
</details>

## Decode Address

| Function Name     | `decode_address` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_decodeAddress` |
| Functionality     | This function can be used to decode a Bech32m encoded address string into its</br>equivalent hrp and data. In addition to that, this function provides other useful information on</br>the address such as the network id and name that it is used for, and the entity type of the</br>address.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "address": "resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3"
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "network_id": "242",
  "network_name": "simulator",
  "entity_type": "GlobalFungibleResourceManager",
  "data": "5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6"
}
```
</details>

## Sbor Encode

| Function Name     | `sbor_encode` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_sborEncode` |
| Functionality     | This function takes in a ScryptoSborValue and encodes it in SBOR.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "type": "ScryptoSbor",
  "value": {
    "kind": "Tuple",
    "fields": [
      {
        "kind": "Decimal",
        "value": "10"
      },
      {
        "kind": "PreciseDecimal",
        "value": "10"
      },
      {
        "kind": "String",
        "value": "Hello World!"
      }
    ]
  }
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "encoded_value": "5c2103a00000e8890423c78a000000000000000000000000000000000000000000000000b000000000000000000a36257aef45394e46ef8b8a90c37f1c2716f3000000000000000000000000000000000000000000000000000000000000000000000000000c0c48656c6c6f20576f726c6421"
}
```
</details>

## Sbor Decode

| Function Name     | `sbor_decode` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_sborDecode` |
| Functionality     | This function takes in a hex string and attempts to decode it into a</br>ScryptoSborValue.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "encoded_value": "5c2104a0000064a7b3b6e00d000000000000000000000000000000000000000000000000b00000000000000000011f6abf64ed386eed97a7daf4f93fe9034f1800000000000000000000000000000000000000000000000000000000000000000000000000c0010000000000000001c002020203",
  "network_id": "242",
  "schema": null
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "type": "ScryptoSbor",
  "value": {
    "kind": "Tuple",
    "fields": [
      {
        "kind": "Decimal",
        "value": "1"
      },
      {
        "kind": "PreciseDecimal",
        "value": "1"
      },
      {
        "kind": "NonFungibleLocalId",
        "value": "#1#"
      },
      {
        "kind": "NonFungibleLocalId",
        "value": "[0203]"
      }
    ]
  }
}
```
</details>

## Derive Virtual Account Address

| Function Name     | `derive_virtual_account_address` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_deriveVirtualAccountAddress` |
| Functionality     | Derives the virtual account component address given a public key and a network id.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "network_id": "242",
  "public_key": {
    "curve": "EcdsaSecp256k1",
    "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
  }
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "virtual_account_address": "account_sim168gge5mvjmkc7q4suyt3yddgk0c7yd5z6g662z4yc548cumw8nztch"
}
```
</details>

## Derive Virtual Identity Address

| Function Name     | `derive_virtual_identity_address` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_deriveVirtualIdentityAddress` |
| Functionality     | Derives the virtual identity component address given a public key and a network id.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "network_id": "242",
  "public_key": {
    "curve": "EcdsaSecp256k1",
    "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
  }
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "virtual_identity_address": "identity_sim16tgge5mvjmkc7q4suyt3yddgk0c7yd5z6g662z4yc548cumwdmnyar"
}
```
</details>

## Derive Babylon Address From Olympia Address

| Function Name     | `derive_babylon_address_from_olympia_address` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_deriveBabylonAddressFromOlympiaAddress` |
| Functionality     | Derives the Babylon account address associated with the given Olympia account</br>address   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "network_id": "1",
  "olympia_account_address": "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842"
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "babylon_account_address": "account_rdx168e8u653alt59xm8ple6khu6cgce9cfx9mlza6wxf7qs3wwdh0pwrf",
  "public_key": {
    "curve": "EcdsaSecp256k1",
    "public_key": "026f08db98ef1d0231eb15580da9123db8e25aa1747c8c32e5fd2ec47b8db73d5c"
  }
}
```
</details>

## Derive Olympia Address From Public Key

| Function Name     | `derive_olympia_address_from_public_key` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_deriveOlympiaAddressFromPublicKey` |
| Functionality     | Given an ECDSA Secp256k1 Public Key and Olympia network, this function derives the</br>Olympia account address associated with the public key on that network.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "network": "Mainnet",
  "public_key": {
    "curve": "EcdsaSecp256k1",
    "public_key": "026f08db98ef1d0231eb15580da9123db8e25aa1747c8c32e5fd2ec47b8db73d5c"
  }
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "olympia_account_address": "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842"
}
```
</details>

## Statically Validate Transaction

| Function Name     | `statically_validate_transaction` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_staticallyValidateTransaction` |
| Functionality     | Performs static validation on the given notarized transaction.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "compiled_notarized_intent": "4d210221022104210707f20a00020000000000000a1002000000000000092200000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa210101080000202201500020200021002022060001210120074100fc8f9ad3d9a3376714e0b941add423a7ea3717787b9fff1d56856a33abaec2a9646ccb17348591dac6b0dc8fb34c6214e6ac3a93d9634d6978a55e7c4a20857a000121012007410160572255d7202af33b63161d97e2952a095c9d223833c4944654225c7c9048297a9f0b8175dd1a3a9ee251390349bd42474e12b35d821d63ef4a1f3555212d2f0001210120074100bab73855b939c69de92c6599d8145f5e17ef8e10cae8746ce857e8a513f6674e5f7a9fcc493ff47c572acc71b2e23849ae77d563a536462b447d85b949897b0201022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29210120074043c6aed201f894ca326106fe13e2f6d633c48dfd8610c53d07fb82cf771050a7b9461a9ed062eacf104be737d422b91d5896625deb09aee564cdd70f56f2ee0201022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe267421012007408d234c5e6d4a354f8232834501bef42cdf847e9aa5c4995505be4d67982d8deb27a8f26306561062bf10833e0ed4313656ceed03a03d85a47224245cf471080f0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740fb2f59c11fc7a720b76521e461f8ae77803abea6c1acc767c2bde8a1f8141f75a1cf6b10ed8dd4a4f663cd6ce0571a3029cdd2021cc2f426ba356cef75c41c00220001210120074100fc8f9ad3d9a3376714e0b941add423a7ea3717787b9fff1d56856a33abaec2a9646ccb17348591dac6b0dc8fb34c6214e6ac3a93d9634d6978a55e7c4a20857a",
  "validation_config": {
    "network_id": "242",
    "max_notarized_payload_size": "1048576",
    "min_cost_unit_limit": "1000000",
    "max_cost_unit_limit": "100000000",
    "min_tip_percentage": "0",
    "max_tip_percentage": "65535",
    "max_epoch_range": "100"
  }
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "validity": "Invalid",
  "error": "PrepareError(DecodeError(UnexpectedValueKind { expected: 34, actual: 33 }))"
}
```
</details>

## Known Entity Addresses

| Function Name     | `known_entity_addresses` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_knownEntityAddresses` |
| Functionality     | Given a network id, this function derives the Bech32m-encoded addresses of the set of</br>known addresses.</br></br>        As an example, this function allows users to derive the XRD resource address, faucet</br>component address, or account package address on any network (given that they know its network</br>id).   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "network_id": "1"
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "account_package_address": "package_rdx1pkgxxxxxxxxxaccntxxxxxxxxxx000929625493xxxxxxxxxaccntx",
  "xrd_resource_address": "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd",
  "system_token_resource_address": "resource_rdx1nfxxxxxxxxxxsystxnxxxxxxxxx002683325037xxxxxxxxxsystxn",
  "ecdsa_secp256k1_token_resource_address": "resource_rdx1nfxxxxxxxxxxsecpsgxxxxxxxxx004638826440xxxxxxxxxsecpsg",
  "eddsa_ed25519_token_resource_address": "resource_rdx1nfxxxxxxxxxxed25sgxxxxxxxxx002236757237xxxxxxxxxed25sg",
  "package_token_resource_address": "resource_rdx1nfxxxxxxxxxxpkcllrxxxxxxxxx003652646977xxxxxxxxxpkcllr",
  "consensus_manager_component_address": "consensusmanager_rdx1scxxxxxxxxxxcnsmgrxxxxxxxxx000999665565xxxxxxxxxcnsmgr"
}
```
</details>

## Hash

| Function Name     | `hash` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_hash` |
| Functionality     | Hashes some payload through the hashing algorithm used in Scrypto and the Radix Engine.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "payload": "4d210221022104210707f20a00020000000000000a1002000000000000092200000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa210101080000202201500020200021002022060001210120074100fc8f9ad3d9a3376714e0b941add423a7ea3717787b9fff1d56856a33abaec2a9646ccb17348591dac6b0dc8fb34c6214e6ac3a93d9634d6978a55e7c4a20857a000121012007410160572255d7202af33b63161d97e2952a095c9d223833c4944654225c7c9048297a9f0b8175dd1a3a9ee251390349bd42474e12b35d821d63ef4a1f3555212d2f0001210120074100bab73855b939c69de92c6599d8145f5e17ef8e10cae8746ce857e8a513f6674e5f7a9fcc493ff47c572acc71b2e23849ae77d563a536462b447d85b949897b0201022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29210120074043c6aed201f894ca326106fe13e2f6d633c48dfd8610c53d07fb82cf771050a7b9461a9ed062eacf104be737d422b91d5896625deb09aee564cdd70f56f2ee0201022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe267421012007408d234c5e6d4a354f8232834501bef42cdf847e9aa5c4995505be4d67982d8deb27a8f26306561062bf10833e0ed4313656ceed03a03d85a47224245cf471080f0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740fb2f59c11fc7a720b76521e461f8ae77803abea6c1acc767c2bde8a1f8141f75a1cf6b10ed8dd4a4f663cd6ce0571a3029cdd2021cc2f426ba356cef75c41c00220001210120074100fc8f9ad3d9a3376714e0b941add423a7ea3717787b9fff1d56856a33abaec2a9646ccb17348591dac6b0dc8fb34c6214e6ac3a93d9634d6978a55e7c4a20857a"
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "value": "c1a830342883c2da6d280a6651d59d3a4adb5dd203784b3d907c4cf928389d23"
}
```
</details>
