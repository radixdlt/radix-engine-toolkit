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
    "version": "1",
    "network_id": "242",
    "start_epoch_inclusive": "512",
    "end_epoch_exclusive": "528",
    "nonce": "34",
    "notary_public_key": {
      "curve": "EcdsaSecp256k1",
      "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
    },
    "notary_as_signatory": true,
    "cost_unit_limit": "100000000",
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
  "compiled_intent": "4d21022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f50508000021022022011000202000"
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
  "compiled_intent": "4d21022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f50508000021022022011000202000"
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "header": {
    "version": "1",
    "network_id": "242",
    "start_epoch_inclusive": "512",
    "end_epoch_exclusive": "528",
    "nonce": "34",
    "notary_public_key": {
      "curve": "EcdsaSecp256k1",
      "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
    },
    "notary_as_signatory": true,
    "cost_unit_limit": "100000000",
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
      "version": "1",
      "network_id": "242",
      "start_epoch_inclusive": "512",
      "end_epoch_exclusive": "528",
      "nonce": "34",
      "notary_public_key": {
        "curve": "EcdsaSecp256k1",
        "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
      },
      "notary_as_signatory": true,
      "cost_unit_limit": "100000000",
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
      "signature": "01af8f9d8e94529600370aa46a285e54a4fdcb231ba0788646ec403b4eec4639bf6231590164c6514f9df7c78dba5924e13e0c625cf1ce8d76c7996c3fb31af1ef"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "01a9b5ae285055f7195d2d60099c42ea7b2b60dbe36e71ec52dc1665de57f4eba23a53bad977282029e2b5a549cd33de3fbb10983023a73d5fc92d271446539599"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "000e54aafd2b9f28d03875ad264ede827060baac5ac82bd47036e5aeb44365b2e8136d525e084098b3df278fdf9135ab69f1809ddfdad9a71b0a02bdd81bde606c"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
      "signature": "bb0de50890160d05b23c926319727dbcd4ec258ba2c06d8490cbd3df45c6a2c172f6892b105a2a7f073bf09956600a8b8ea132c3a61be522bda48177d7e68706"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
      "signature": "eb13f90a3afb5865767534153d111b3b8c4a111322e2bbc32c56c4d339c3254c50aaf0749d159ceca465db7d6a93be869ca08def07c5a498816d7c8cbf896200"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
      "signature": "37cb4ad6a41f5c00dec197d3d437a4b926853156dfcd6169f2aaebd89db42992149fff66b38f57792aba4c3b16497585a284981ff851abb91292665be8c97d00"
    }
  ]
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "compiled_intent": "4d210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220220110002020002022060001210120074101af8f9d8e94529600370aa46a285e54a4fdcb231ba0788646ec403b4eec4639bf6231590164c6514f9df7c78dba5924e13e0c625cf1ce8d76c7996c3fb31af1ef0001210120074101a9b5ae285055f7195d2d60099c42ea7b2b60dbe36e71ec52dc1665de57f4eba23a53bad977282029e2b5a549cd33de3fbb10983023a73d5fc92d27144653959900012101200741000e54aafd2b9f28d03875ad264ede827060baac5ac82bd47036e5aeb44365b2e8136d525e084098b3df278fdf9135ab69f1809ddfdad9a71b0a02bdd81bde606c01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740bb0de50890160d05b23c926319727dbcd4ec258ba2c06d8490cbd3df45c6a2c172f6892b105a2a7f073bf09956600a8b8ea132c3a61be522bda48177d7e6870601022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740eb13f90a3afb5865767534153d111b3b8c4a111322e2bbc32c56c4d339c3254c50aaf0749d159ceca465db7d6a93be869ca08def07c5a498816d7c8cbf8962000102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b210120074037cb4ad6a41f5c00dec197d3d437a4b926853156dfcd6169f2aaebd89db42992149fff66b38f57792aba4c3b16497585a284981ff851abb91292665be8c97d00"
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
  "compiled_signed_intent": "4d210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220220110002020002022060001210120074101af8f9d8e94529600370aa46a285e54a4fdcb231ba0788646ec403b4eec4639bf6231590164c6514f9df7c78dba5924e13e0c625cf1ce8d76c7996c3fb31af1ef0001210120074101a9b5ae285055f7195d2d60099c42ea7b2b60dbe36e71ec52dc1665de57f4eba23a53bad977282029e2b5a549cd33de3fbb10983023a73d5fc92d27144653959900012101200741000e54aafd2b9f28d03875ad264ede827060baac5ac82bd47036e5aeb44365b2e8136d525e084098b3df278fdf9135ab69f1809ddfdad9a71b0a02bdd81bde606c01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740bb0de50890160d05b23c926319727dbcd4ec258ba2c06d8490cbd3df45c6a2c172f6892b105a2a7f073bf09956600a8b8ea132c3a61be522bda48177d7e6870601022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740eb13f90a3afb5865767534153d111b3b8c4a111322e2bbc32c56c4d339c3254c50aaf0749d159ceca465db7d6a93be869ca08def07c5a498816d7c8cbf8962000102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b210120074037cb4ad6a41f5c00dec197d3d437a4b926853156dfcd6169f2aaebd89db42992149fff66b38f57792aba4c3b16497585a284981ff851abb91292665be8c97d00"
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "intent": {
    "header": {
      "version": "1",
      "network_id": "242",
      "start_epoch_inclusive": "512",
      "end_epoch_exclusive": "528",
      "nonce": "34",
      "notary_public_key": {
        "curve": "EcdsaSecp256k1",
        "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
      },
      "notary_as_signatory": true,
      "cost_unit_limit": "100000000",
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
      "signature": "01af8f9d8e94529600370aa46a285e54a4fdcb231ba0788646ec403b4eec4639bf6231590164c6514f9df7c78dba5924e13e0c625cf1ce8d76c7996c3fb31af1ef"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "01a9b5ae285055f7195d2d60099c42ea7b2b60dbe36e71ec52dc1665de57f4eba23a53bad977282029e2b5a549cd33de3fbb10983023a73d5fc92d271446539599"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "000e54aafd2b9f28d03875ad264ede827060baac5ac82bd47036e5aeb44365b2e8136d525e084098b3df278fdf9135ab69f1809ddfdad9a71b0a02bdd81bde606c"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
      "signature": "bb0de50890160d05b23c926319727dbcd4ec258ba2c06d8490cbd3df45c6a2c172f6892b105a2a7f073bf09956600a8b8ea132c3a61be522bda48177d7e68706"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
      "signature": "eb13f90a3afb5865767534153d111b3b8c4a111322e2bbc32c56c4d339c3254c50aaf0749d159ceca465db7d6a93be869ca08def07c5a498816d7c8cbf896200"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
      "signature": "37cb4ad6a41f5c00dec197d3d437a4b926853156dfcd6169f2aaebd89db42992149fff66b38f57792aba4c3b16497585a284981ff851abb91292665be8c97d00"
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
        "version": "1",
        "network_id": "242",
        "start_epoch_inclusive": "512",
        "end_epoch_exclusive": "528",
        "nonce": "34",
        "notary_public_key": {
          "curve": "EcdsaSecp256k1",
          "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
        },
        "notary_as_signatory": true,
        "cost_unit_limit": "100000000",
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
        "signature": "01af8f9d8e94529600370aa46a285e54a4fdcb231ba0788646ec403b4eec4639bf6231590164c6514f9df7c78dba5924e13e0c625cf1ce8d76c7996c3fb31af1ef"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "01a9b5ae285055f7195d2d60099c42ea7b2b60dbe36e71ec52dc1665de57f4eba23a53bad977282029e2b5a549cd33de3fbb10983023a73d5fc92d271446539599"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "000e54aafd2b9f28d03875ad264ede827060baac5ac82bd47036e5aeb44365b2e8136d525e084098b3df278fdf9135ab69f1809ddfdad9a71b0a02bdd81bde606c"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
        "signature": "bb0de50890160d05b23c926319727dbcd4ec258ba2c06d8490cbd3df45c6a2c172f6892b105a2a7f073bf09956600a8b8ea132c3a61be522bda48177d7e68706"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
        "signature": "eb13f90a3afb5865767534153d111b3b8c4a111322e2bbc32c56c4d339c3254c50aaf0749d159ceca465db7d6a93be869ca08def07c5a498816d7c8cbf896200"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
        "signature": "37cb4ad6a41f5c00dec197d3d437a4b926853156dfcd6169f2aaebd89db42992149fff66b38f57792aba4c3b16497585a284981ff851abb91292665be8c97d00"
      }
    ]
  },
  "notary_signature": {
    "curve": "EcdsaSecp256k1",
    "signature": "00467b443fca6e4c6b966ea8f8b7a9a35f1195450cae7167398988a87415e9121e56cca9627aacae1f4cca4298c19f99456b5cc2514c9f5411d8bad5088f9fdf68"
  }
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "compiled_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220220110002020002022060001210120074101af8f9d8e94529600370aa46a285e54a4fdcb231ba0788646ec403b4eec4639bf6231590164c6514f9df7c78dba5924e13e0c625cf1ce8d76c7996c3fb31af1ef0001210120074101a9b5ae285055f7195d2d60099c42ea7b2b60dbe36e71ec52dc1665de57f4eba23a53bad977282029e2b5a549cd33de3fbb10983023a73d5fc92d27144653959900012101200741000e54aafd2b9f28d03875ad264ede827060baac5ac82bd47036e5aeb44365b2e8136d525e084098b3df278fdf9135ab69f1809ddfdad9a71b0a02bdd81bde606c01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740bb0de50890160d05b23c926319727dbcd4ec258ba2c06d8490cbd3df45c6a2c172f6892b105a2a7f073bf09956600a8b8ea132c3a61be522bda48177d7e6870601022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740eb13f90a3afb5865767534153d111b3b8c4a111322e2bbc32c56c4d339c3254c50aaf0749d159ceca465db7d6a93be869ca08def07c5a498816d7c8cbf8962000102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b210120074037cb4ad6a41f5c00dec197d3d437a4b926853156dfcd6169f2aaebd89db42992149fff66b38f57792aba4c3b16497585a284981ff851abb91292665be8c97d00220001210120074100467b443fca6e4c6b966ea8f8b7a9a35f1195450cae7167398988a87415e9121e56cca9627aacae1f4cca4298c19f99456b5cc2514c9f5411d8bad5088f9fdf68"
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
  "compiled_notarized_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220220110002020002022060001210120074101af8f9d8e94529600370aa46a285e54a4fdcb231ba0788646ec403b4eec4639bf6231590164c6514f9df7c78dba5924e13e0c625cf1ce8d76c7996c3fb31af1ef0001210120074101a9b5ae285055f7195d2d60099c42ea7b2b60dbe36e71ec52dc1665de57f4eba23a53bad977282029e2b5a549cd33de3fbb10983023a73d5fc92d27144653959900012101200741000e54aafd2b9f28d03875ad264ede827060baac5ac82bd47036e5aeb44365b2e8136d525e084098b3df278fdf9135ab69f1809ddfdad9a71b0a02bdd81bde606c01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740bb0de50890160d05b23c926319727dbcd4ec258ba2c06d8490cbd3df45c6a2c172f6892b105a2a7f073bf09956600a8b8ea132c3a61be522bda48177d7e6870601022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740eb13f90a3afb5865767534153d111b3b8c4a111322e2bbc32c56c4d339c3254c50aaf0749d159ceca465db7d6a93be869ca08def07c5a498816d7c8cbf8962000102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b210120074037cb4ad6a41f5c00dec197d3d437a4b926853156dfcd6169f2aaebd89db42992149fff66b38f57792aba4c3b16497585a284981ff851abb91292665be8c97d00220001210120074100467b443fca6e4c6b966ea8f8b7a9a35f1195450cae7167398988a87415e9121e56cca9627aacae1f4cca4298c19f99456b5cc2514c9f5411d8bad5088f9fdf68"
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
        "version": "1",
        "network_id": "242",
        "start_epoch_inclusive": "512",
        "end_epoch_exclusive": "528",
        "nonce": "34",
        "notary_public_key": {
          "curve": "EcdsaSecp256k1",
          "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
        },
        "notary_as_signatory": true,
        "cost_unit_limit": "100000000",
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
        "signature": "01af8f9d8e94529600370aa46a285e54a4fdcb231ba0788646ec403b4eec4639bf6231590164c6514f9df7c78dba5924e13e0c625cf1ce8d76c7996c3fb31af1ef"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "01a9b5ae285055f7195d2d60099c42ea7b2b60dbe36e71ec52dc1665de57f4eba23a53bad977282029e2b5a549cd33de3fbb10983023a73d5fc92d271446539599"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "000e54aafd2b9f28d03875ad264ede827060baac5ac82bd47036e5aeb44365b2e8136d525e084098b3df278fdf9135ab69f1809ddfdad9a71b0a02bdd81bde606c"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
        "signature": "bb0de50890160d05b23c926319727dbcd4ec258ba2c06d8490cbd3df45c6a2c172f6892b105a2a7f073bf09956600a8b8ea132c3a61be522bda48177d7e68706"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
        "signature": "eb13f90a3afb5865767534153d111b3b8c4a111322e2bbc32c56c4d339c3254c50aaf0749d159ceca465db7d6a93be869ca08def07c5a498816d7c8cbf896200"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
        "signature": "37cb4ad6a41f5c00dec197d3d437a4b926853156dfcd6169f2aaebd89db42992149fff66b38f57792aba4c3b16497585a284981ff851abb91292665be8c97d00"
      }
    ]
  },
  "notary_signature": {
    "curve": "EcdsaSecp256k1",
    "signature": "00467b443fca6e4c6b966ea8f8b7a9a35f1195450cae7167398988a87415e9121e56cca9627aacae1f4cca4298c19f99456b5cc2514c9f5411d8bad5088f9fdf68"
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
  "compiled_unknown_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220220110002020002022060001210120074101af8f9d8e94529600370aa46a285e54a4fdcb231ba0788646ec403b4eec4639bf6231590164c6514f9df7c78dba5924e13e0c625cf1ce8d76c7996c3fb31af1ef0001210120074101a9b5ae285055f7195d2d60099c42ea7b2b60dbe36e71ec52dc1665de57f4eba23a53bad977282029e2b5a549cd33de3fbb10983023a73d5fc92d27144653959900012101200741000e54aafd2b9f28d03875ad264ede827060baac5ac82bd47036e5aeb44365b2e8136d525e084098b3df278fdf9135ab69f1809ddfdad9a71b0a02bdd81bde606c01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740bb0de50890160d05b23c926319727dbcd4ec258ba2c06d8490cbd3df45c6a2c172f6892b105a2a7f073bf09956600a8b8ea132c3a61be522bda48177d7e6870601022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740eb13f90a3afb5865767534153d111b3b8c4a111322e2bbc32c56c4d339c3254c50aaf0749d159ceca465db7d6a93be869ca08def07c5a498816d7c8cbf8962000102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b210120074037cb4ad6a41f5c00dec197d3d437a4b926853156dfcd6169f2aaebd89db42992149fff66b38f57792aba4c3b16497585a284981ff851abb91292665be8c97d00220001210120074100467b443fca6e4c6b966ea8f8b7a9a35f1195450cae7167398988a87415e9121e56cca9627aacae1f4cca4298c19f99456b5cc2514c9f5411d8bad5088f9fdf68"
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
          "version": "1",
          "network_id": "242",
          "start_epoch_inclusive": "512",
          "end_epoch_exclusive": "528",
          "nonce": "34",
          "notary_public_key": {
            "curve": "EcdsaSecp256k1",
            "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
          },
          "notary_as_signatory": true,
          "cost_unit_limit": "100000000",
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
          "signature": "01af8f9d8e94529600370aa46a285e54a4fdcb231ba0788646ec403b4eec4639bf6231590164c6514f9df7c78dba5924e13e0c625cf1ce8d76c7996c3fb31af1ef"
        },
        {
          "curve": "EcdsaSecp256k1",
          "signature": "01a9b5ae285055f7195d2d60099c42ea7b2b60dbe36e71ec52dc1665de57f4eba23a53bad977282029e2b5a549cd33de3fbb10983023a73d5fc92d271446539599"
        },
        {
          "curve": "EcdsaSecp256k1",
          "signature": "000e54aafd2b9f28d03875ad264ede827060baac5ac82bd47036e5aeb44365b2e8136d525e084098b3df278fdf9135ab69f1809ddfdad9a71b0a02bdd81bde606c"
        },
        {
          "curve": "EddsaEd25519",
          "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
          "signature": "bb0de50890160d05b23c926319727dbcd4ec258ba2c06d8490cbd3df45c6a2c172f6892b105a2a7f073bf09956600a8b8ea132c3a61be522bda48177d7e68706"
        },
        {
          "curve": "EddsaEd25519",
          "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
          "signature": "eb13f90a3afb5865767534153d111b3b8c4a111322e2bbc32c56c4d339c3254c50aaf0749d159ceca465db7d6a93be869ca08def07c5a498816d7c8cbf896200"
        },
        {
          "curve": "EddsaEd25519",
          "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
          "signature": "37cb4ad6a41f5c00dec197d3d437a4b926853156dfcd6169f2aaebd89db42992149fff66b38f57792aba4c3b16497585a284981ff851abb91292665be8c97d00"
        }
      ]
    },
    "notary_signature": {
      "curve": "EcdsaSecp256k1",
      "signature": "00467b443fca6e4c6b966ea8f8b7a9a35f1195450cae7167398988a87415e9121e56cca9627aacae1f4cca4298c19f99456b5cc2514c9f5411d8bad5088f9fdf68"
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
  "entity_type": "GlobalFungibleResource",
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
  "compiled_notarized_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220220110002020002022060001210120074101af8f9d8e94529600370aa46a285e54a4fdcb231ba0788646ec403b4eec4639bf6231590164c6514f9df7c78dba5924e13e0c625cf1ce8d76c7996c3fb31af1ef0001210120074101a9b5ae285055f7195d2d60099c42ea7b2b60dbe36e71ec52dc1665de57f4eba23a53bad977282029e2b5a549cd33de3fbb10983023a73d5fc92d27144653959900012101200741000e54aafd2b9f28d03875ad264ede827060baac5ac82bd47036e5aeb44365b2e8136d525e084098b3df278fdf9135ab69f1809ddfdad9a71b0a02bdd81bde606c01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740bb0de50890160d05b23c926319727dbcd4ec258ba2c06d8490cbd3df45c6a2c172f6892b105a2a7f073bf09956600a8b8ea132c3a61be522bda48177d7e6870601022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740eb13f90a3afb5865767534153d111b3b8c4a111322e2bbc32c56c4d339c3254c50aaf0749d159ceca465db7d6a93be869ca08def07c5a498816d7c8cbf8962000102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b210120074037cb4ad6a41f5c00dec197d3d437a4b926853156dfcd6169f2aaebd89db42992149fff66b38f57792aba4c3b16497585a284981ff851abb91292665be8c97d00220001210120074101af8f9d8e94529600370aa46a285e54a4fdcb231ba0788646ec403b4eec4639bf6231590164c6514f9df7c78dba5924e13e0c625cf1ce8d76c7996c3fb31af1ef",
  "validation_config": {
    "network_id": "242",
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
  "error": "SignatureValidationError(InvalidNotarySignature)"
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
  "epoch_manager_system_address": "epochmanager_rdx1sexxxxxxxxxxephmgrxxxxxxxxx009352500589xxxxxxxxxephmgr",
  "clock_system_address": "clock_rdx1skxxxxxxxxxxclckxxxxxxxxxxx002253583992xxxxxxxxxclckxx"
}
```
</details>

## Hash

| Function Name     | `hash` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_hash` |
| Functionality     | Hashes some payload through the hashing algorithm used in Scrypto and the Radix</br>Engine.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "payload": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220220110002020002022060001210120074101af8f9d8e94529600370aa46a285e54a4fdcb231ba0788646ec403b4eec4639bf6231590164c6514f9df7c78dba5924e13e0c625cf1ce8d76c7996c3fb31af1ef0001210120074101a9b5ae285055f7195d2d60099c42ea7b2b60dbe36e71ec52dc1665de57f4eba23a53bad977282029e2b5a549cd33de3fbb10983023a73d5fc92d27144653959900012101200741000e54aafd2b9f28d03875ad264ede827060baac5ac82bd47036e5aeb44365b2e8136d525e084098b3df278fdf9135ab69f1809ddfdad9a71b0a02bdd81bde606c01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740bb0de50890160d05b23c926319727dbcd4ec258ba2c06d8490cbd3df45c6a2c172f6892b105a2a7f073bf09956600a8b8ea132c3a61be522bda48177d7e6870601022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740eb13f90a3afb5865767534153d111b3b8c4a111322e2bbc32c56c4d339c3254c50aaf0749d159ceca465db7d6a93be869ca08def07c5a498816d7c8cbf8962000102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b210120074037cb4ad6a41f5c00dec197d3d437a4b926853156dfcd6169f2aaebd89db42992149fff66b38f57792aba4c3b16497585a284981ff851abb91292665be8c97d00220001210120074101af8f9d8e94529600370aa46a285e54a4fdcb231ba0788646ec403b4eec4639bf6231590164c6514f9df7c78dba5924e13e0c625cf1ce8d76c7996c3fb31af1ef"
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "value": "7f626ef483cac790852a8e79e22a0577b66059d1e30da317f0a6c460bc98cdb9"
}
```
</details>
