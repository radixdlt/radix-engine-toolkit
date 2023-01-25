# Requests Examples

This document contains examples and descriptions of the different requests and responses which the Radix Engine Toolkit may provide. As long as all of the CI test pass, then you may treat this document as the canonical truth for the format of the different requests and as valid examples of the payload and responses of these requests.


## Information

| Function Name     | `information` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_information` |
| Functionality     | The function provides information information on the currently in-use radix engine toolkit such as the version of the radix engine toolkit. In most cases, this is the first function written when integrating new clients; so, this function is often times seen as the "Hello World" example of the radix engine toolkit. |
| Request Type      | `InformationRequest` |
| Response Type     | `InformationResponse` |

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
  "package_version": "0.7.0",
  "git_hash": "This is just an example. We don't have a commit hash here :)"
}
```
</details>

## Convert Manifest

| Function Name     | `convert_manifest` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_convertManifest` |
| Functionality     | Clients have a need to be able to read, parse, understand, and interrogate transaction manifests to get more information on what a transactions might be doing. Transaction manifests have so far existed in one format: as strings. While the string format is very human readable, it is not easily readable by machines as a lexer and parser are needed to make sense of them; thus, it is for clients to programmatically make sense of transactions. As such, there is a need for another transaction manifest format (to supplement, NOT replace) which machines can easily make sense of without the need to implement a lexer and parser.</br></br>Therefore, this library introduces a Parsed format for transaction manifests which clients can use when wanting to read and interrogate their transaction manifests in code. The transaction manifest Parsed format has a 1:1 mapping to the string format of transaction manifests, meaning that anything which can be done in the string format of transaction manifests, can be done in the Parsed format as well.</br></br>This function allows the client the convert their manifest between the two supported manifest types: string and Parsed. |
| Request Type      | `ConvertManifestRequest` |
| Response Type     | `ConvertManifestResponse` |

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
          "instruction": "CALL_METHOD",
          "component_address": {
            "type": "ComponentAddress",
            "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
          },
          "method_name": {
            "type": "String",
            "value": "withdraw_by_amount"
          },
          "arguments": [
            {
              "type": "Decimal",
              "value": "5"
            },
            {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            }
          ]
        },
        {
          "instruction": "TAKE_FROM_WORKTOP_BY_AMOUNT",
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          },
          "amount": {
            "type": "Decimal",
            "value": "2"
          },
          "into_bucket": {
            "type": "Bucket",
            "variant": "String",
            "identifier": "bucket1"
          }
        },
        {
          "instruction": "CALL_METHOD",
          "component_address": {
            "type": "ComponentAddress",
            "address": "component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum"
          },
          "method_name": {
            "type": "String",
            "value": "buy_gumball"
          },
          "arguments": [
            {
              "type": "Bucket",
              "variant": "String",
              "identifier": "bucket1"
            }
          ]
        },
        {
          "instruction": "ASSERT_WORKTOP_CONTAINS_BY_AMOUNT",
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          },
          "amount": {
            "type": "Decimal",
            "value": "3"
          }
        },
        {
          "instruction": "ASSERT_WORKTOP_CONTAINS",
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6"
          }
        },
        {
          "instruction": "TAKE_FROM_WORKTOP",
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          },
          "into_bucket": {
            "type": "Bucket",
            "variant": "String",
            "identifier": "bucket2"
          }
        },
        {
          "instruction": "CREATE_PROOF_FROM_BUCKET",
          "bucket": {
            "type": "Bucket",
            "variant": "String",
            "identifier": "bucket2"
          },
          "into_proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof1"
          }
        },
        {
          "instruction": "CLONE_PROOF",
          "proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof1"
          },
          "into_proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof2"
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof1"
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof2"
          }
        },
        {
          "instruction": "CALL_METHOD",
          "component_address": {
            "type": "ComponentAddress",
            "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
          },
          "method_name": {
            "type": "String",
            "value": "create_proof_by_amount"
          },
          "arguments": [
            {
              "type": "Decimal",
              "value": "5"
            },
            {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            }
          ]
        },
        {
          "instruction": "POP_FROM_AUTH_ZONE",
          "into_proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof3"
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof3"
          }
        },
        {
          "instruction": "RETURN_TO_WORKTOP",
          "bucket": {
            "type": "Bucket",
            "variant": "String",
            "identifier": "bucket2"
          }
        },
        {
          "instruction": "TAKE_FROM_WORKTOP_BY_IDS",
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          },
          "ids": [
            {
              "type": "NonFungibleId",
              "variant": "Number",
              "value": "1"
            }
          ],
          "into_bucket": {
            "type": "Bucket",
            "variant": "String",
            "identifier": "bucket3"
          }
        },
        {
          "instruction": "DROP_ALL_PROOFS"
        },
        {
          "instruction": "CALL_METHOD",
          "component_address": {
            "type": "ComponentAddress",
            "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
          },
          "method_name": {
            "type": "String",
            "value": "deposit_batch"
          },
          "arguments": [
            {
              "type": "Expression",
              "value": "ENTIRE_WORKTOP"
            }
          ]
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
        "instruction": "CALL_METHOD",
        "component_address": {
          "type": "ComponentAddress",
          "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
        },
        "method_name": {
          "type": "String",
          "value": "withdraw_by_amount"
        },
        "arguments": [
          {
            "type": "Decimal",
            "value": "5"
          },
          {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          }
        ]
      },
      {
        "instruction": "TAKE_FROM_WORKTOP_BY_AMOUNT",
        "resource_address": {
          "type": "ResourceAddress",
          "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
        },
        "amount": {
          "type": "Decimal",
          "value": "2"
        },
        "into_bucket": {
          "type": "Bucket",
          "variant": "String",
          "identifier": "bucket1"
        }
      },
      {
        "instruction": "CALL_METHOD",
        "component_address": {
          "type": "ComponentAddress",
          "address": "component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum"
        },
        "method_name": {
          "type": "String",
          "value": "buy_gumball"
        },
        "arguments": [
          {
            "type": "Bucket",
            "variant": "String",
            "identifier": "bucket1"
          }
        ]
      },
      {
        "instruction": "ASSERT_WORKTOP_CONTAINS_BY_AMOUNT",
        "resource_address": {
          "type": "ResourceAddress",
          "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
        },
        "amount": {
          "type": "Decimal",
          "value": "3"
        }
      },
      {
        "instruction": "ASSERT_WORKTOP_CONTAINS",
        "resource_address": {
          "type": "ResourceAddress",
          "address": "resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6"
        }
      },
      {
        "instruction": "TAKE_FROM_WORKTOP",
        "resource_address": {
          "type": "ResourceAddress",
          "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
        },
        "into_bucket": {
          "type": "Bucket",
          "variant": "String",
          "identifier": "bucket2"
        }
      },
      {
        "instruction": "CREATE_PROOF_FROM_BUCKET",
        "bucket": {
          "type": "Bucket",
          "variant": "String",
          "identifier": "bucket2"
        },
        "into_proof": {
          "type": "Proof",
          "variant": "String",
          "identifier": "proof1"
        }
      },
      {
        "instruction": "CLONE_PROOF",
        "proof": {
          "type": "Proof",
          "variant": "String",
          "identifier": "proof1"
        },
        "into_proof": {
          "type": "Proof",
          "variant": "String",
          "identifier": "proof2"
        }
      },
      {
        "instruction": "DROP_PROOF",
        "proof": {
          "type": "Proof",
          "variant": "String",
          "identifier": "proof1"
        }
      },
      {
        "instruction": "DROP_PROOF",
        "proof": {
          "type": "Proof",
          "variant": "String",
          "identifier": "proof2"
        }
      },
      {
        "instruction": "CALL_METHOD",
        "component_address": {
          "type": "ComponentAddress",
          "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
        },
        "method_name": {
          "type": "String",
          "value": "create_proof_by_amount"
        },
        "arguments": [
          {
            "type": "Decimal",
            "value": "5"
          },
          {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          }
        ]
      },
      {
        "instruction": "POP_FROM_AUTH_ZONE",
        "into_proof": {
          "type": "Proof",
          "variant": "String",
          "identifier": "proof3"
        }
      },
      {
        "instruction": "DROP_PROOF",
        "proof": {
          "type": "Proof",
          "variant": "String",
          "identifier": "proof3"
        }
      },
      {
        "instruction": "RETURN_TO_WORKTOP",
        "bucket": {
          "type": "Bucket",
          "variant": "String",
          "identifier": "bucket2"
        }
      },
      {
        "instruction": "TAKE_FROM_WORKTOP_BY_IDS",
        "resource_address": {
          "type": "ResourceAddress",
          "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
        },
        "ids": [
          {
            "type": "NonFungibleId",
            "variant": "Number",
            "value": "1"
          }
        ],
        "into_bucket": {
          "type": "Bucket",
          "variant": "String",
          "identifier": "bucket3"
        }
      },
      {
        "instruction": "DROP_ALL_PROOFS"
      },
      {
        "instruction": "CALL_METHOD",
        "component_address": {
          "type": "ComponentAddress",
          "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
        },
        "method_name": {
          "type": "String",
          "value": "deposit_batch"
        },
        "arguments": [
          {
            "type": "Expression",
            "value": "ENTIRE_WORKTOP"
          }
        ]
      }
    ]
  },
  "blobs": []
}
```
</details>

## Compile Transaction Intent

| Function Name     | `compile_transaction_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_compileTransactionIntent` |
| Functionality     | Takes a transaction intent and compiles it by SBOR encoding it and returning it back to the caller. This is mainly useful when creating a transaction. |
| Request Type      | `CompileTransactionIntentRequest` |
| Response Type     | `CompileTransactionIntentResponse` |

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
          "instruction": "CALL_METHOD",
          "component_address": {
            "type": "ComponentAddress",
            "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
          },
          "method_name": {
            "type": "String",
            "value": "withdraw_by_amount"
          },
          "arguments": [
            {
              "type": "Decimal",
              "value": "5"
            },
            {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            }
          ]
        },
        {
          "instruction": "TAKE_FROM_WORKTOP_BY_AMOUNT",
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          },
          "amount": {
            "type": "Decimal",
            "value": "2"
          },
          "into_bucket": {
            "type": "Bucket",
            "variant": "String",
            "identifier": "bucket1"
          }
        },
        {
          "instruction": "CALL_METHOD",
          "component_address": {
            "type": "ComponentAddress",
            "address": "component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum"
          },
          "method_name": {
            "type": "String",
            "value": "buy_gumball"
          },
          "arguments": [
            {
              "type": "Bucket",
              "variant": "String",
              "identifier": "bucket1"
            }
          ]
        },
        {
          "instruction": "ASSERT_WORKTOP_CONTAINS_BY_AMOUNT",
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          },
          "amount": {
            "type": "Decimal",
            "value": "3"
          }
        },
        {
          "instruction": "ASSERT_WORKTOP_CONTAINS",
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6"
          }
        },
        {
          "instruction": "TAKE_FROM_WORKTOP",
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          },
          "into_bucket": {
            "type": "Bucket",
            "variant": "String",
            "identifier": "bucket2"
          }
        },
        {
          "instruction": "CREATE_PROOF_FROM_BUCKET",
          "bucket": {
            "type": "Bucket",
            "variant": "String",
            "identifier": "bucket2"
          },
          "into_proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof1"
          }
        },
        {
          "instruction": "CLONE_PROOF",
          "proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof1"
          },
          "into_proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof2"
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof1"
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof2"
          }
        },
        {
          "instruction": "CALL_METHOD",
          "component_address": {
            "type": "ComponentAddress",
            "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
          },
          "method_name": {
            "type": "String",
            "value": "create_proof_by_amount"
          },
          "arguments": [
            {
              "type": "Decimal",
              "value": "5"
            },
            {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            }
          ]
        },
        {
          "instruction": "POP_FROM_AUTH_ZONE",
          "into_proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof3"
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof3"
          }
        },
        {
          "instruction": "RETURN_TO_WORKTOP",
          "bucket": {
            "type": "Bucket",
            "variant": "String",
            "identifier": "bucket2"
          }
        },
        {
          "instruction": "TAKE_FROM_WORKTOP_BY_IDS",
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          },
          "ids": [
            {
              "type": "NonFungibleId",
              "variant": "Number",
              "value": "1"
            }
          ],
          "into_bucket": {
            "type": "Bucket",
            "variant": "String",
            "identifier": "bucket3"
          }
        },
        {
          "instruction": "DROP_ALL_PROOFS"
        },
        {
          "instruction": "CALL_METHOD",
          "component_address": {
            "type": "ComponentAddress",
            "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
          },
          "method_name": {
            "type": "String",
            "value": "deposit_batch"
          },
          "arguments": [
            {
              "type": "Expression",
              "value": "ENTIRE_WORKTOP"
            }
          ]
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
  "compiled_intent": "5c21022109070107f20a00020000000000000a10020000000000000a2200000000000000220001b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220221123038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040102b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000042303810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c2101a0000000000502b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000404018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001820000000000000000000000000000000000000000000000000000040d01a0010000000e01a1020000000f01a1020000000f01a10300000023038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f444829163450000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000407000f01a1040000000301a001000000020220b70100010000000000000082000000000000000000000000000000000000000000000000000004100023038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007055c2101a200202000"
}
```
</details>

## Decompile Transaction Intent

| Function Name     | `decompile_transaction_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_decompileTransactionIntent` |
| Functionality     | This function does the opposite of the compile_transaction_intent function. It takes in a compiled transaction intent and decompiles it into its human-readable / machine-readable format. |
| Request Type      | `DecompileTransactionIntentRequest` |
| Response Type     | `DecompileTransactionIntentResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "instructions_output_kind": "Parsed",
  "compiled_intent": "5c21022109070107f20a00020000000000000a10020000000000000a2200000000000000220001b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220221123038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040102b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000042303810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c2101a0000000000502b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000404018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001820000000000000000000000000000000000000000000000000000040d01a0010000000e01a1020000000f01a1020000000f01a10300000023038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f444829163450000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000407000f01a1040000000301a001000000020220b70100010000000000000082000000000000000000000000000000000000000000000000000004100023038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007055c2101a200202000"
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
          "instruction": "CALL_METHOD",
          "component_address": {
            "type": "ComponentAddress",
            "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
          },
          "method_name": {
            "type": "String",
            "value": "withdraw_by_amount"
          },
          "arguments": [
            {
              "type": "Decimal",
              "value": "5"
            },
            {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            }
          ]
        },
        {
          "instruction": "TAKE_FROM_WORKTOP_BY_AMOUNT",
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          },
          "amount": {
            "type": "Decimal",
            "value": "2"
          },
          "into_bucket": {
            "type": "Bucket",
            "variant": "String",
            "identifier": "bucket1"
          }
        },
        {
          "instruction": "CALL_METHOD",
          "component_address": {
            "type": "ComponentAddress",
            "address": "component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum"
          },
          "method_name": {
            "type": "String",
            "value": "buy_gumball"
          },
          "arguments": [
            {
              "type": "Bucket",
              "variant": "String",
              "identifier": "bucket1"
            }
          ]
        },
        {
          "instruction": "ASSERT_WORKTOP_CONTAINS_BY_AMOUNT",
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          },
          "amount": {
            "type": "Decimal",
            "value": "3"
          }
        },
        {
          "instruction": "ASSERT_WORKTOP_CONTAINS",
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6"
          }
        },
        {
          "instruction": "TAKE_FROM_WORKTOP",
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          },
          "into_bucket": {
            "type": "Bucket",
            "variant": "String",
            "identifier": "bucket2"
          }
        },
        {
          "instruction": "CREATE_PROOF_FROM_BUCKET",
          "bucket": {
            "type": "Bucket",
            "variant": "String",
            "identifier": "bucket2"
          },
          "into_proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof1"
          }
        },
        {
          "instruction": "CLONE_PROOF",
          "proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof1"
          },
          "into_proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof2"
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof1"
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof2"
          }
        },
        {
          "instruction": "CALL_METHOD",
          "component_address": {
            "type": "ComponentAddress",
            "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
          },
          "method_name": {
            "type": "String",
            "value": "create_proof_by_amount"
          },
          "arguments": [
            {
              "type": "Decimal",
              "value": "5"
            },
            {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            }
          ]
        },
        {
          "instruction": "POP_FROM_AUTH_ZONE",
          "into_proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof3"
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "variant": "String",
            "identifier": "proof3"
          }
        },
        {
          "instruction": "RETURN_TO_WORKTOP",
          "bucket": {
            "type": "Bucket",
            "variant": "String",
            "identifier": "bucket2"
          }
        },
        {
          "instruction": "TAKE_FROM_WORKTOP_BY_IDS",
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          },
          "ids": [
            {
              "type": "NonFungibleId",
              "variant": "Number",
              "value": "1"
            }
          ],
          "into_bucket": {
            "type": "Bucket",
            "variant": "String",
            "identifier": "bucket3"
          }
        },
        {
          "instruction": "DROP_ALL_PROOFS"
        },
        {
          "instruction": "CALL_METHOD",
          "component_address": {
            "type": "ComponentAddress",
            "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
          },
          "method_name": {
            "type": "String",
            "value": "deposit_batch"
          },
          "arguments": [
            {
              "type": "Expression",
              "value": "ENTIRE_WORKTOP"
            }
          ]
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
| Functionality     | This function takes in a raw transaction intent as well as its signatures and compiles it. This is useful when a notary wishes to notarize a signed transaction intent. |
| Request Type      | `CompileSignedTransactionIntentRequest` |
| Response Type     | `CompileSignedTransactionIntentResponse` |

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
            "instruction": "CALL_METHOD",
            "component_address": {
              "type": "ComponentAddress",
              "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
            },
            "method_name": {
              "type": "String",
              "value": "withdraw_by_amount"
            },
            "arguments": [
              {
                "type": "Decimal",
                "value": "5"
              },
              {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              }
            ]
          },
          {
            "instruction": "TAKE_FROM_WORKTOP_BY_AMOUNT",
            "resource_address": {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            },
            "amount": {
              "type": "Decimal",
              "value": "2"
            },
            "into_bucket": {
              "type": "Bucket",
              "variant": "String",
              "identifier": "bucket1"
            }
          },
          {
            "instruction": "CALL_METHOD",
            "component_address": {
              "type": "ComponentAddress",
              "address": "component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum"
            },
            "method_name": {
              "type": "String",
              "value": "buy_gumball"
            },
            "arguments": [
              {
                "type": "Bucket",
                "variant": "String",
                "identifier": "bucket1"
              }
            ]
          },
          {
            "instruction": "ASSERT_WORKTOP_CONTAINS_BY_AMOUNT",
            "resource_address": {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            },
            "amount": {
              "type": "Decimal",
              "value": "3"
            }
          },
          {
            "instruction": "ASSERT_WORKTOP_CONTAINS",
            "resource_address": {
              "type": "ResourceAddress",
              "address": "resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6"
            }
          },
          {
            "instruction": "TAKE_FROM_WORKTOP",
            "resource_address": {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            },
            "into_bucket": {
              "type": "Bucket",
              "variant": "String",
              "identifier": "bucket2"
            }
          },
          {
            "instruction": "CREATE_PROOF_FROM_BUCKET",
            "bucket": {
              "type": "Bucket",
              "variant": "String",
              "identifier": "bucket2"
            },
            "into_proof": {
              "type": "Proof",
              "variant": "String",
              "identifier": "proof1"
            }
          },
          {
            "instruction": "CLONE_PROOF",
            "proof": {
              "type": "Proof",
              "variant": "String",
              "identifier": "proof1"
            },
            "into_proof": {
              "type": "Proof",
              "variant": "String",
              "identifier": "proof2"
            }
          },
          {
            "instruction": "DROP_PROOF",
            "proof": {
              "type": "Proof",
              "variant": "String",
              "identifier": "proof1"
            }
          },
          {
            "instruction": "DROP_PROOF",
            "proof": {
              "type": "Proof",
              "variant": "String",
              "identifier": "proof2"
            }
          },
          {
            "instruction": "CALL_METHOD",
            "component_address": {
              "type": "ComponentAddress",
              "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
            },
            "method_name": {
              "type": "String",
              "value": "create_proof_by_amount"
            },
            "arguments": [
              {
                "type": "Decimal",
                "value": "5"
              },
              {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              }
            ]
          },
          {
            "instruction": "POP_FROM_AUTH_ZONE",
            "into_proof": {
              "type": "Proof",
              "variant": "String",
              "identifier": "proof3"
            }
          },
          {
            "instruction": "DROP_PROOF",
            "proof": {
              "type": "Proof",
              "variant": "String",
              "identifier": "proof3"
            }
          },
          {
            "instruction": "RETURN_TO_WORKTOP",
            "bucket": {
              "type": "Bucket",
              "variant": "String",
              "identifier": "bucket2"
            }
          },
          {
            "instruction": "TAKE_FROM_WORKTOP_BY_IDS",
            "resource_address": {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            },
            "ids": [
              {
                "type": "NonFungibleId",
                "variant": "Number",
                "value": "1"
              }
            ],
            "into_bucket": {
              "type": "Bucket",
              "variant": "String",
              "identifier": "bucket3"
            }
          },
          {
            "instruction": "DROP_ALL_PROOFS"
          },
          {
            "instruction": "CALL_METHOD",
            "component_address": {
              "type": "ComponentAddress",
              "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
            },
            "method_name": {
              "type": "String",
              "value": "deposit_batch"
            },
            "arguments": [
              {
                "type": "Expression",
                "value": "ENTIRE_WORKTOP"
              }
            ]
          }
        ]
      },
      "blobs": []
    }
  },
  "intent_signatures": [
    {
      "curve": "EcdsaSecp256k1",
      "signature": "00d97c6ee042ea5e522a609bd2b7ea30e32f081e89d9ca4c857a405a50fe16779b3198b51ed7b2ef033e9866643a8125524bd96438bb960ec1c3c02ac86eeabaf1"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "0004671578e1f5eac6b80754041380680c4a2aade0173dbac78452e2681b0fdb2b4856070494ee3e1857a3c6ab840a214f2e3b88bbb140b258a5eac5fdfa60001c"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "01607f3ec615db0e08d0b9b8aebe07eaa1f8694c83c1c0d7015c5b1a0ad0371b9c57967a57c302e9145df831b0f083116ae700bb9f50da0111d5ed9630f08b9729"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
      "signature": "30fccf0b2fff1e261e24fd7d5de8e4b31ce7d01a5ef609acca19e4f75cab58a2ec42f7c80b70ff0f866e4f37ff27108da11f779e97da5f1ad3b239a6f3c49208"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
      "signature": "638000d190edda9c5a724cd3cc53a3b3093ebb2a00966830224bbfd29c80f351b416005e7883e5d70e35b5cbab5893c27585f7302cccbb72997be0c7bfe5e507"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
      "signature": "9df6945807370c409e0890efff264637c04acc9289b2ad050910add5dcedab305ec84b73ef1b50c763d9fbe97c85806b16542b465d9d34879922ae06cd91f409"
    }
  ]
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "compiled_intent": "5c210221022109070107f20a00020000000000000a10020000000000000a2200000000000000220001b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220221123038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040102b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000042303810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c2101a0000000000502b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000404018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001820000000000000000000000000000000000000000000000000000040d01a0010000000e01a1020000000f01a1020000000f01a10300000023038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f444829163450000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000407000f01a1040000000301a001000000020220b70100010000000000000082000000000000000000000000000000000000000000000000000004100023038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007055c2101a2002020002022060001b200d97c6ee042ea5e522a609bd2b7ea30e32f081e89d9ca4c857a405a50fe16779b3198b51ed7b2ef033e9866643a8125524bd96438bb960ec1c3c02ac86eeabaf10001b20004671578e1f5eac6b80754041380680c4a2aade0173dbac78452e2681b0fdb2b4856070494ee3e1857a3c6ab840a214f2e3b88bbb140b258a5eac5fdfa60001c0001b201607f3ec615db0e08d0b9b8aebe07eaa1f8694c83c1c0d7015c5b1a0ad0371b9c57967a57c302e9145df831b0f083116ae700bb9f50da0111d5ed9630f08b97290102b34cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29b430fccf0b2fff1e261e24fd7d5de8e4b31ce7d01a5ef609acca19e4f75cab58a2ec42f7c80b70ff0f866e4f37ff27108da11f779e97da5f1ad3b239a6f3c492080102b37422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674b4638000d190edda9c5a724cd3cc53a3b3093ebb2a00966830224bbfd29c80f351b416005e7883e5d70e35b5cbab5893c27585f7302cccbb72997be0c7bfe5e5070102b3f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54bb49df6945807370c409e0890efff264637c04acc9289b2ad050910add5dcedab305ec84b73ef1b50c763d9fbe97c85806b16542b465d9d34879922ae06cd91f409"
}
```
</details>

## Decompile Signed Transaction Intent

| Function Name     | `decompile_signed_transaction_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_decompileSignedTransactionIntent` |
| Functionality     | This function does the opposite of the compile_signed_transaction_intent function. This function takes in a compiled signed transaction intent and decompiles it into its transaction intent and signatures. |
| Request Type      | `DecompileSignedTransactionIntentRequest` |
| Response Type     | `DecompileSignedTransactionIntentResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "instructions_output_kind": "Parsed",
  "compiled_signed_intent": "5c210221022109070107f20a00020000000000000a10020000000000000a2200000000000000220001b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220221123038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040102b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000042303810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c2101a0000000000502b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000404018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001820000000000000000000000000000000000000000000000000000040d01a0010000000e01a1020000000f01a1020000000f01a10300000023038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f444829163450000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000407000f01a1040000000301a001000000020220b70100010000000000000082000000000000000000000000000000000000000000000000000004100023038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007055c2101a2002020002022060001b200d97c6ee042ea5e522a609bd2b7ea30e32f081e89d9ca4c857a405a50fe16779b3198b51ed7b2ef033e9866643a8125524bd96438bb960ec1c3c02ac86eeabaf10001b20004671578e1f5eac6b80754041380680c4a2aade0173dbac78452e2681b0fdb2b4856070494ee3e1857a3c6ab840a214f2e3b88bbb140b258a5eac5fdfa60001c0001b201607f3ec615db0e08d0b9b8aebe07eaa1f8694c83c1c0d7015c5b1a0ad0371b9c57967a57c302e9145df831b0f083116ae700bb9f50da0111d5ed9630f08b97290102b34cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29b430fccf0b2fff1e261e24fd7d5de8e4b31ce7d01a5ef609acca19e4f75cab58a2ec42f7c80b70ff0f866e4f37ff27108da11f779e97da5f1ad3b239a6f3c492080102b37422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674b4638000d190edda9c5a724cd3cc53a3b3093ebb2a00966830224bbfd29c80f351b416005e7883e5d70e35b5cbab5893c27585f7302cccbb72997be0c7bfe5e5070102b3f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54bb49df6945807370c409e0890efff264637c04acc9289b2ad050910add5dcedab305ec84b73ef1b50c763d9fbe97c85806b16542b465d9d34879922ae06cd91f409"
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
            "instruction": "CALL_METHOD",
            "component_address": {
              "type": "ComponentAddress",
              "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
            },
            "method_name": {
              "type": "String",
              "value": "withdraw_by_amount"
            },
            "arguments": [
              {
                "type": "Decimal",
                "value": "5"
              },
              {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              }
            ]
          },
          {
            "instruction": "TAKE_FROM_WORKTOP_BY_AMOUNT",
            "resource_address": {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            },
            "amount": {
              "type": "Decimal",
              "value": "2"
            },
            "into_bucket": {
              "type": "Bucket",
              "variant": "String",
              "identifier": "bucket1"
            }
          },
          {
            "instruction": "CALL_METHOD",
            "component_address": {
              "type": "ComponentAddress",
              "address": "component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum"
            },
            "method_name": {
              "type": "String",
              "value": "buy_gumball"
            },
            "arguments": [
              {
                "type": "Bucket",
                "variant": "String",
                "identifier": "bucket1"
              }
            ]
          },
          {
            "instruction": "ASSERT_WORKTOP_CONTAINS_BY_AMOUNT",
            "resource_address": {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            },
            "amount": {
              "type": "Decimal",
              "value": "3"
            }
          },
          {
            "instruction": "ASSERT_WORKTOP_CONTAINS",
            "resource_address": {
              "type": "ResourceAddress",
              "address": "resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6"
            }
          },
          {
            "instruction": "TAKE_FROM_WORKTOP",
            "resource_address": {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            },
            "into_bucket": {
              "type": "Bucket",
              "variant": "String",
              "identifier": "bucket2"
            }
          },
          {
            "instruction": "CREATE_PROOF_FROM_BUCKET",
            "bucket": {
              "type": "Bucket",
              "variant": "String",
              "identifier": "bucket2"
            },
            "into_proof": {
              "type": "Proof",
              "variant": "String",
              "identifier": "proof1"
            }
          },
          {
            "instruction": "CLONE_PROOF",
            "proof": {
              "type": "Proof",
              "variant": "String",
              "identifier": "proof1"
            },
            "into_proof": {
              "type": "Proof",
              "variant": "String",
              "identifier": "proof2"
            }
          },
          {
            "instruction": "DROP_PROOF",
            "proof": {
              "type": "Proof",
              "variant": "String",
              "identifier": "proof1"
            }
          },
          {
            "instruction": "DROP_PROOF",
            "proof": {
              "type": "Proof",
              "variant": "String",
              "identifier": "proof2"
            }
          },
          {
            "instruction": "CALL_METHOD",
            "component_address": {
              "type": "ComponentAddress",
              "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
            },
            "method_name": {
              "type": "String",
              "value": "create_proof_by_amount"
            },
            "arguments": [
              {
                "type": "Decimal",
                "value": "5"
              },
              {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              }
            ]
          },
          {
            "instruction": "POP_FROM_AUTH_ZONE",
            "into_proof": {
              "type": "Proof",
              "variant": "String",
              "identifier": "proof3"
            }
          },
          {
            "instruction": "DROP_PROOF",
            "proof": {
              "type": "Proof",
              "variant": "String",
              "identifier": "proof3"
            }
          },
          {
            "instruction": "RETURN_TO_WORKTOP",
            "bucket": {
              "type": "Bucket",
              "variant": "String",
              "identifier": "bucket2"
            }
          },
          {
            "instruction": "TAKE_FROM_WORKTOP_BY_IDS",
            "resource_address": {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            },
            "ids": [
              {
                "type": "NonFungibleId",
                "variant": "Number",
                "value": "1"
              }
            ],
            "into_bucket": {
              "type": "Bucket",
              "variant": "String",
              "identifier": "bucket3"
            }
          },
          {
            "instruction": "DROP_ALL_PROOFS"
          },
          {
            "instruction": "CALL_METHOD",
            "component_address": {
              "type": "ComponentAddress",
              "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
            },
            "method_name": {
              "type": "String",
              "value": "deposit_batch"
            },
            "arguments": [
              {
                "type": "Expression",
                "value": "ENTIRE_WORKTOP"
              }
            ]
          }
        ]
      },
      "blobs": []
    }
  },
  "intent_signatures": [
    {
      "curve": "EcdsaSecp256k1",
      "signature": "00d97c6ee042ea5e522a609bd2b7ea30e32f081e89d9ca4c857a405a50fe16779b3198b51ed7b2ef033e9866643a8125524bd96438bb960ec1c3c02ac86eeabaf1"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "0004671578e1f5eac6b80754041380680c4a2aade0173dbac78452e2681b0fdb2b4856070494ee3e1857a3c6ab840a214f2e3b88bbb140b258a5eac5fdfa60001c"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "01607f3ec615db0e08d0b9b8aebe07eaa1f8694c83c1c0d7015c5b1a0ad0371b9c57967a57c302e9145df831b0f083116ae700bb9f50da0111d5ed9630f08b9729"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
      "signature": "30fccf0b2fff1e261e24fd7d5de8e4b31ce7d01a5ef609acca19e4f75cab58a2ec42f7c80b70ff0f866e4f37ff27108da11f779e97da5f1ad3b239a6f3c49208"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
      "signature": "638000d190edda9c5a724cd3cc53a3b3093ebb2a00966830224bbfd29c80f351b416005e7883e5d70e35b5cbab5893c27585f7302cccbb72997be0c7bfe5e507"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
      "signature": "9df6945807370c409e0890efff264637c04acc9289b2ad050910add5dcedab305ec84b73ef1b50c763d9fbe97c85806b16542b465d9d34879922ae06cd91f409"
    }
  ]
}
```
</details>

## Compile Notarized Transaction

| Function Name     | `compile_notarized_transaction` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_compileNotarizedTransaction` |
| Functionality     | This function takes in a raw signed transaction intent as well as the notary signature and compiles it. This is useful when we wish to submit a transaction to the Gateway API |
| Request Type      | `CompileNotarizedTransactionRequest` |
| Response Type     | `CompileNotarizedTransactionResponse` |

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
              "instruction": "CALL_METHOD",
              "component_address": {
                "type": "ComponentAddress",
                "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
              },
              "method_name": {
                "type": "String",
                "value": "withdraw_by_amount"
              },
              "arguments": [
                {
                  "type": "Decimal",
                  "value": "5"
                },
                {
                  "type": "ResourceAddress",
                  "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
                }
              ]
            },
            {
              "instruction": "TAKE_FROM_WORKTOP_BY_AMOUNT",
              "resource_address": {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              },
              "amount": {
                "type": "Decimal",
                "value": "2"
              },
              "into_bucket": {
                "type": "Bucket",
                "variant": "String",
                "identifier": "bucket1"
              }
            },
            {
              "instruction": "CALL_METHOD",
              "component_address": {
                "type": "ComponentAddress",
                "address": "component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum"
              },
              "method_name": {
                "type": "String",
                "value": "buy_gumball"
              },
              "arguments": [
                {
                  "type": "Bucket",
                  "variant": "String",
                  "identifier": "bucket1"
                }
              ]
            },
            {
              "instruction": "ASSERT_WORKTOP_CONTAINS_BY_AMOUNT",
              "resource_address": {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              },
              "amount": {
                "type": "Decimal",
                "value": "3"
              }
            },
            {
              "instruction": "ASSERT_WORKTOP_CONTAINS",
              "resource_address": {
                "type": "ResourceAddress",
                "address": "resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6"
              }
            },
            {
              "instruction": "TAKE_FROM_WORKTOP",
              "resource_address": {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              },
              "into_bucket": {
                "type": "Bucket",
                "variant": "String",
                "identifier": "bucket2"
              }
            },
            {
              "instruction": "CREATE_PROOF_FROM_BUCKET",
              "bucket": {
                "type": "Bucket",
                "variant": "String",
                "identifier": "bucket2"
              },
              "into_proof": {
                "type": "Proof",
                "variant": "String",
                "identifier": "proof1"
              }
            },
            {
              "instruction": "CLONE_PROOF",
              "proof": {
                "type": "Proof",
                "variant": "String",
                "identifier": "proof1"
              },
              "into_proof": {
                "type": "Proof",
                "variant": "String",
                "identifier": "proof2"
              }
            },
            {
              "instruction": "DROP_PROOF",
              "proof": {
                "type": "Proof",
                "variant": "String",
                "identifier": "proof1"
              }
            },
            {
              "instruction": "DROP_PROOF",
              "proof": {
                "type": "Proof",
                "variant": "String",
                "identifier": "proof2"
              }
            },
            {
              "instruction": "CALL_METHOD",
              "component_address": {
                "type": "ComponentAddress",
                "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
              },
              "method_name": {
                "type": "String",
                "value": "create_proof_by_amount"
              },
              "arguments": [
                {
                  "type": "Decimal",
                  "value": "5"
                },
                {
                  "type": "ResourceAddress",
                  "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
                }
              ]
            },
            {
              "instruction": "POP_FROM_AUTH_ZONE",
              "into_proof": {
                "type": "Proof",
                "variant": "String",
                "identifier": "proof3"
              }
            },
            {
              "instruction": "DROP_PROOF",
              "proof": {
                "type": "Proof",
                "variant": "String",
                "identifier": "proof3"
              }
            },
            {
              "instruction": "RETURN_TO_WORKTOP",
              "bucket": {
                "type": "Bucket",
                "variant": "String",
                "identifier": "bucket2"
              }
            },
            {
              "instruction": "TAKE_FROM_WORKTOP_BY_IDS",
              "resource_address": {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              },
              "ids": [
                {
                  "type": "NonFungibleId",
                  "variant": "Number",
                  "value": "1"
                }
              ],
              "into_bucket": {
                "type": "Bucket",
                "variant": "String",
                "identifier": "bucket3"
              }
            },
            {
              "instruction": "DROP_ALL_PROOFS"
            },
            {
              "instruction": "CALL_METHOD",
              "component_address": {
                "type": "ComponentAddress",
                "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
              },
              "method_name": {
                "type": "String",
                "value": "deposit_batch"
              },
              "arguments": [
                {
                  "type": "Expression",
                  "value": "ENTIRE_WORKTOP"
                }
              ]
            }
          ]
        },
        "blobs": []
      }
    },
    "intent_signatures": [
      {
        "curve": "EcdsaSecp256k1",
        "signature": "00d97c6ee042ea5e522a609bd2b7ea30e32f081e89d9ca4c857a405a50fe16779b3198b51ed7b2ef033e9866643a8125524bd96438bb960ec1c3c02ac86eeabaf1"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "0004671578e1f5eac6b80754041380680c4a2aade0173dbac78452e2681b0fdb2b4856070494ee3e1857a3c6ab840a214f2e3b88bbb140b258a5eac5fdfa60001c"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "01607f3ec615db0e08d0b9b8aebe07eaa1f8694c83c1c0d7015c5b1a0ad0371b9c57967a57c302e9145df831b0f083116ae700bb9f50da0111d5ed9630f08b9729"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
        "signature": "30fccf0b2fff1e261e24fd7d5de8e4b31ce7d01a5ef609acca19e4f75cab58a2ec42f7c80b70ff0f866e4f37ff27108da11f779e97da5f1ad3b239a6f3c49208"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
        "signature": "638000d190edda9c5a724cd3cc53a3b3093ebb2a00966830224bbfd29c80f351b416005e7883e5d70e35b5cbab5893c27585f7302cccbb72997be0c7bfe5e507"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
        "signature": "9df6945807370c409e0890efff264637c04acc9289b2ad050910add5dcedab305ec84b73ef1b50c763d9fbe97c85806b16542b465d9d34879922ae06cd91f409"
      }
    ]
  },
  "notary_signature": {
    "curve": "EcdsaSecp256k1",
    "signature": "00c2daa118a8541ef384a38f431a3dde3bf023463e49c124dca7631e2083495dbc1a50968d3bd21033c2bdf0b22c64d56c84a3f1c8e61b847e0208c694627835ef"
  }
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "compiled_intent": "5c2102210221022109070107f20a00020000000000000a10020000000000000a2200000000000000220001b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220221123038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040102b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000042303810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c2101a0000000000502b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000404018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001820000000000000000000000000000000000000000000000000000040d01a0010000000e01a1020000000f01a1020000000f01a10300000023038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f444829163450000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000407000f01a1040000000301a001000000020220b70100010000000000000082000000000000000000000000000000000000000000000000000004100023038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007055c2101a2002020002022060001b200d97c6ee042ea5e522a609bd2b7ea30e32f081e89d9ca4c857a405a50fe16779b3198b51ed7b2ef033e9866643a8125524bd96438bb960ec1c3c02ac86eeabaf10001b20004671578e1f5eac6b80754041380680c4a2aade0173dbac78452e2681b0fdb2b4856070494ee3e1857a3c6ab840a214f2e3b88bbb140b258a5eac5fdfa60001c0001b201607f3ec615db0e08d0b9b8aebe07eaa1f8694c83c1c0d7015c5b1a0ad0371b9c57967a57c302e9145df831b0f083116ae700bb9f50da0111d5ed9630f08b97290102b34cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29b430fccf0b2fff1e261e24fd7d5de8e4b31ce7d01a5ef609acca19e4f75cab58a2ec42f7c80b70ff0f866e4f37ff27108da11f779e97da5f1ad3b239a6f3c492080102b37422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674b4638000d190edda9c5a724cd3cc53a3b3093ebb2a00966830224bbfd29c80f351b416005e7883e5d70e35b5cbab5893c27585f7302cccbb72997be0c7bfe5e5070102b3f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54bb49df6945807370c409e0890efff264637c04acc9289b2ad050910add5dcedab305ec84b73ef1b50c763d9fbe97c85806b16542b465d9d34879922ae06cd91f409220001b200c2daa118a8541ef384a38f431a3dde3bf023463e49c124dca7631e2083495dbc1a50968d3bd21033c2bdf0b22c64d56c84a3f1c8e61b847e0208c694627835ef"
}
```
</details>

## Decompile Notarized Transaction

| Function Name     | `decompile_notarized_transaction` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_decompileNotarizedTransaction` |
| Functionality     | This function does the opposite of the compile_notarized_intent()_intent function. This function takes in a compiled notarized transaction intent and decompiles it into its signed transaction intent and notary signature. |
| Request Type      | `DecompileNotarizedTransactionRequest` |
| Response Type     | `DecompileNotarizedTransactionResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "instructions_output_kind": "Parsed",
  "compiled_notarized_intent": "5c2102210221022109070107f20a00020000000000000a10020000000000000a2200000000000000220001b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220221123038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040102b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000042303810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c2101a0000000000502b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000404018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001820000000000000000000000000000000000000000000000000000040d01a0010000000e01a1020000000f01a1020000000f01a10300000023038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f444829163450000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000407000f01a1040000000301a001000000020220b70100010000000000000082000000000000000000000000000000000000000000000000000004100023038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007055c2101a2002020002022060001b200d97c6ee042ea5e522a609bd2b7ea30e32f081e89d9ca4c857a405a50fe16779b3198b51ed7b2ef033e9866643a8125524bd96438bb960ec1c3c02ac86eeabaf10001b20004671578e1f5eac6b80754041380680c4a2aade0173dbac78452e2681b0fdb2b4856070494ee3e1857a3c6ab840a214f2e3b88bbb140b258a5eac5fdfa60001c0001b201607f3ec615db0e08d0b9b8aebe07eaa1f8694c83c1c0d7015c5b1a0ad0371b9c57967a57c302e9145df831b0f083116ae700bb9f50da0111d5ed9630f08b97290102b34cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29b430fccf0b2fff1e261e24fd7d5de8e4b31ce7d01a5ef609acca19e4f75cab58a2ec42f7c80b70ff0f866e4f37ff27108da11f779e97da5f1ad3b239a6f3c492080102b37422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674b4638000d190edda9c5a724cd3cc53a3b3093ebb2a00966830224bbfd29c80f351b416005e7883e5d70e35b5cbab5893c27585f7302cccbb72997be0c7bfe5e5070102b3f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54bb49df6945807370c409e0890efff264637c04acc9289b2ad050910add5dcedab305ec84b73ef1b50c763d9fbe97c85806b16542b465d9d34879922ae06cd91f409220001b200c2daa118a8541ef384a38f431a3dde3bf023463e49c124dca7631e2083495dbc1a50968d3bd21033c2bdf0b22c64d56c84a3f1c8e61b847e0208c694627835ef"
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
              "instruction": "CALL_METHOD",
              "component_address": {
                "type": "ComponentAddress",
                "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
              },
              "method_name": {
                "type": "String",
                "value": "withdraw_by_amount"
              },
              "arguments": [
                {
                  "type": "Decimal",
                  "value": "5"
                },
                {
                  "type": "ResourceAddress",
                  "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
                }
              ]
            },
            {
              "instruction": "TAKE_FROM_WORKTOP_BY_AMOUNT",
              "resource_address": {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              },
              "amount": {
                "type": "Decimal",
                "value": "2"
              },
              "into_bucket": {
                "type": "Bucket",
                "variant": "String",
                "identifier": "bucket1"
              }
            },
            {
              "instruction": "CALL_METHOD",
              "component_address": {
                "type": "ComponentAddress",
                "address": "component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum"
              },
              "method_name": {
                "type": "String",
                "value": "buy_gumball"
              },
              "arguments": [
                {
                  "type": "Bucket",
                  "variant": "String",
                  "identifier": "bucket1"
                }
              ]
            },
            {
              "instruction": "ASSERT_WORKTOP_CONTAINS_BY_AMOUNT",
              "resource_address": {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              },
              "amount": {
                "type": "Decimal",
                "value": "3"
              }
            },
            {
              "instruction": "ASSERT_WORKTOP_CONTAINS",
              "resource_address": {
                "type": "ResourceAddress",
                "address": "resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6"
              }
            },
            {
              "instruction": "TAKE_FROM_WORKTOP",
              "resource_address": {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              },
              "into_bucket": {
                "type": "Bucket",
                "variant": "String",
                "identifier": "bucket2"
              }
            },
            {
              "instruction": "CREATE_PROOF_FROM_BUCKET",
              "bucket": {
                "type": "Bucket",
                "variant": "String",
                "identifier": "bucket2"
              },
              "into_proof": {
                "type": "Proof",
                "variant": "String",
                "identifier": "proof1"
              }
            },
            {
              "instruction": "CLONE_PROOF",
              "proof": {
                "type": "Proof",
                "variant": "String",
                "identifier": "proof1"
              },
              "into_proof": {
                "type": "Proof",
                "variant": "String",
                "identifier": "proof2"
              }
            },
            {
              "instruction": "DROP_PROOF",
              "proof": {
                "type": "Proof",
                "variant": "String",
                "identifier": "proof1"
              }
            },
            {
              "instruction": "DROP_PROOF",
              "proof": {
                "type": "Proof",
                "variant": "String",
                "identifier": "proof2"
              }
            },
            {
              "instruction": "CALL_METHOD",
              "component_address": {
                "type": "ComponentAddress",
                "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
              },
              "method_name": {
                "type": "String",
                "value": "create_proof_by_amount"
              },
              "arguments": [
                {
                  "type": "Decimal",
                  "value": "5"
                },
                {
                  "type": "ResourceAddress",
                  "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
                }
              ]
            },
            {
              "instruction": "POP_FROM_AUTH_ZONE",
              "into_proof": {
                "type": "Proof",
                "variant": "String",
                "identifier": "proof3"
              }
            },
            {
              "instruction": "DROP_PROOF",
              "proof": {
                "type": "Proof",
                "variant": "String",
                "identifier": "proof3"
              }
            },
            {
              "instruction": "RETURN_TO_WORKTOP",
              "bucket": {
                "type": "Bucket",
                "variant": "String",
                "identifier": "bucket2"
              }
            },
            {
              "instruction": "TAKE_FROM_WORKTOP_BY_IDS",
              "resource_address": {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              },
              "ids": [
                {
                  "type": "NonFungibleId",
                  "variant": "Number",
                  "value": "1"
                }
              ],
              "into_bucket": {
                "type": "Bucket",
                "variant": "String",
                "identifier": "bucket3"
              }
            },
            {
              "instruction": "DROP_ALL_PROOFS"
            },
            {
              "instruction": "CALL_METHOD",
              "component_address": {
                "type": "ComponentAddress",
                "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
              },
              "method_name": {
                "type": "String",
                "value": "deposit_batch"
              },
              "arguments": [
                {
                  "type": "Expression",
                  "value": "ENTIRE_WORKTOP"
                }
              ]
            }
          ]
        },
        "blobs": []
      }
    },
    "intent_signatures": [
      {
        "curve": "EcdsaSecp256k1",
        "signature": "00d97c6ee042ea5e522a609bd2b7ea30e32f081e89d9ca4c857a405a50fe16779b3198b51ed7b2ef033e9866643a8125524bd96438bb960ec1c3c02ac86eeabaf1"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "0004671578e1f5eac6b80754041380680c4a2aade0173dbac78452e2681b0fdb2b4856070494ee3e1857a3c6ab840a214f2e3b88bbb140b258a5eac5fdfa60001c"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "01607f3ec615db0e08d0b9b8aebe07eaa1f8694c83c1c0d7015c5b1a0ad0371b9c57967a57c302e9145df831b0f083116ae700bb9f50da0111d5ed9630f08b9729"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
        "signature": "30fccf0b2fff1e261e24fd7d5de8e4b31ce7d01a5ef609acca19e4f75cab58a2ec42f7c80b70ff0f866e4f37ff27108da11f779e97da5f1ad3b239a6f3c49208"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
        "signature": "638000d190edda9c5a724cd3cc53a3b3093ebb2a00966830224bbfd29c80f351b416005e7883e5d70e35b5cbab5893c27585f7302cccbb72997be0c7bfe5e507"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
        "signature": "9df6945807370c409e0890efff264637c04acc9289b2ad050910add5dcedab305ec84b73ef1b50c763d9fbe97c85806b16542b465d9d34879922ae06cd91f409"
      }
    ]
  },
  "notary_signature": {
    "curve": "EcdsaSecp256k1",
    "signature": "00c2daa118a8541ef384a38f431a3dde3bf023463e49c124dca7631e2083495dbc1a50968d3bd21033c2bdf0b22c64d56c84a3f1c8e61b847e0208c694627835ef"
  }
}
```
</details>

## Encode Address

| Function Name     | `encode_address` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_encodeAddress` |
| Functionality     | This function can be used when we have a byte array which we wish to do Bech32m encoding on. In this case, the HRP to use will be determined through the entity byte of the passed address hex string. |
| Request Type      | `EncodeAddressRequest` |
| Response Type     | `EncodeAddressResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "address_bytes": "000000000000000000000000000000000000000000000000000002",
  "network_id": "242"
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "type": "ResourceAddress",
  "address": {
    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpqd60rqz"
  }
}
```
</details>

## Decode Address

| Function Name     | `decode_address` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_decodeAddress` |
| Functionality     | This function can be used to decode a Bech32m encoded address string into its equivalent hrp and data. In addition to that, this function provides other useful information on the address such as the network id and name that it is used for, and the entity type of the address. |
| Request Type      | `DecodeAddressRequest` |
| Response Type     | `DecodeAddressResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpqd60rqz"
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "network_id": "242",
  "network_name": "simulator",
  "entity_type": "Resource",
  "data": "000000000000000000000000000000000000000000000000000002",
  "hrp": "resource_sim"
}
```
</details>

## Sbor Encode

| Function Name     | `sbor_encode` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_sborEncode` |
| Functionality     | This function takes in a Value and encodes it in SBOR. |
| Request Type      | `SborEncodeRequest` |
| Response Type     | `SborEncodeResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "type": "Tuple",
  "elements": [
    {
      "type": "Decimal",
      "value": "10"
    },
    {
      "type": "PreciseDecimal",
      "value": "10"
    },
    {
      "type": "String",
      "value": "Hello World!"
    },
    {
      "type": "Tuple",
      "elements": [
        {
          "type": "Decimal",
          "value": "10"
        },
        {
          "type": "PreciseDecimal",
          "value": "10"
        },
        {
          "type": "String",
          "value": "Hello World!"
        },
        {
          "type": "Tuple",
          "elements": [
            {
              "type": "Decimal",
              "value": "10"
            },
            {
              "type": "PreciseDecimal",
              "value": "10"
            },
            {
              "type": "String",
              "value": "Hello World!"
            },
            {
              "type": "Tuple",
              "elements": [
                {
                  "type": "Decimal",
                  "value": "10"
                },
                {
                  "type": "PreciseDecimal",
                  "value": "10"
                },
                {
                  "type": "String",
                  "value": "Hello World!"
                },
                {
                  "type": "Array",
                  "element_kind": "Decimal",
                  "elements": [
                    {
                      "type": "Decimal",
                      "value": "20"
                    },
                    {
                      "type": "Decimal",
                      "value": "100"
                    },
                    {
                      "type": "Decimal",
                      "value": "192.31"
                    }
                  ]
                }
              ]
            }
          ]
        }
      ]
    }
  ]
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "encoded_value": "5c2104b50000e8890423c78a000000000000000000000000000000000000000000000000b600000000000000000a36257aef45394e46ef8b8a90c37f1c2716f3000000000000000000000000000000000000000000000000000000000000000000000000000c0c48656c6c6f20576f726c64212104b50000e8890423c78a000000000000000000000000000000000000000000000000b600000000000000000a36257aef45394e46ef8b8a90c37f1c2716f3000000000000000000000000000000000000000000000000000000000000000000000000000c0c48656c6c6f20576f726c64212104b50000e8890423c78a000000000000000000000000000000000000000000000000b600000000000000000a36257aef45394e46ef8b8a90c37f1c2716f3000000000000000000000000000000000000000000000000000000000000000000000000000c0c48656c6c6f20576f726c64212104b50000e8890423c78a000000000000000000000000000000000000000000000000b600000000000000000a36257aef45394e46ef8b8a90c37f1c2716f3000000000000000000000000000000000000000000000000000000000000000000000000000c0c48656c6c6f20576f726c642120b5030000d01309468e15010000000000000000000000000000000000000000000000000010632d5ec76b05000000000000000000000000000000000000000000000000005f13195ed66c0a0000000000000000000000000000000000000000000000"
}
```
</details>

## Sbor Decode

| Function Name     | `sbor_decode` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_sborDecode` |
| Functionality     | This function takes in a hex string and attemps to decode it into a Value. |
| Request Type      | `SborDecodeRequest` |
| Response Type     | `SborDecodeResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "encoded_value": "5c2104b50000e8890423c78a000000000000000000000000000000000000000000000000b600000000000000000a36257aef45394e46ef8b8a90c37f1c2716f3000000000000000000000000000000000000000000000000000000000000000000000000000c0c48656c6c6f20576f726c64212104b50000e8890423c78a000000000000000000000000000000000000000000000000b600000000000000000a36257aef45394e46ef8b8a90c37f1c2716f3000000000000000000000000000000000000000000000000000000000000000000000000000c0c48656c6c6f20576f726c64212104b50000e8890423c78a000000000000000000000000000000000000000000000000b600000000000000000a36257aef45394e46ef8b8a90c37f1c2716f3000000000000000000000000000000000000000000000000000000000000000000000000000c0c48656c6c6f20576f726c64212104b50000e8890423c78a000000000000000000000000000000000000000000000000b600000000000000000a36257aef45394e46ef8b8a90c37f1c2716f3000000000000000000000000000000000000000000000000000000000000000000000000000c0c48656c6c6f20576f726c642120b5030000d01309468e15010000000000000000000000000000000000000000000000000010632d5ec76b05000000000000000000000000000000000000000000000000005f13195ed66c0a0000000000000000000000000000000000000000000000",
  "network_id": "242"
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "type": "Tuple",
  "elements": [
    {
      "type": "Decimal",
      "value": "10"
    },
    {
      "type": "PreciseDecimal",
      "value": "10"
    },
    {
      "type": "String",
      "value": "Hello World!"
    },
    {
      "type": "Tuple",
      "elements": [
        {
          "type": "Decimal",
          "value": "10"
        },
        {
          "type": "PreciseDecimal",
          "value": "10"
        },
        {
          "type": "String",
          "value": "Hello World!"
        },
        {
          "type": "Tuple",
          "elements": [
            {
              "type": "Decimal",
              "value": "10"
            },
            {
              "type": "PreciseDecimal",
              "value": "10"
            },
            {
              "type": "String",
              "value": "Hello World!"
            },
            {
              "type": "Tuple",
              "elements": [
                {
                  "type": "Decimal",
                  "value": "10"
                },
                {
                  "type": "PreciseDecimal",
                  "value": "10"
                },
                {
                  "type": "String",
                  "value": "Hello World!"
                },
                {
                  "type": "Array",
                  "element_kind": "Decimal",
                  "elements": [
                    {
                      "type": "Decimal",
                      "value": "20"
                    },
                    {
                      "type": "Decimal",
                      "value": "100"
                    },
                    {
                      "type": "Decimal",
                      "value": "192.31"
                    }
                  ]
                }
              ]
            }
          ]
        }
      ]
    }
  ]
}
```
</details>

## Derive Virtual Account Address

| Function Name     | `derive_virtual_account_address` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_deriveVirtualAccountAddress` |
| Functionality     | Derives the virtual account component address given a public key and a network id. |
| Request Type      | `DeriveVirtualAccountAddressRequest` |
| Response Type     | `DeriveVirtualAccountAddressResponse` |

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
  "virtual_account_address": {
    "type": "ComponentAddress",
    "address": "account_sim1qcpveqrdmh7kw0vefxrzjw5e0fgw3sgdj4zrh66jkphqqc62xd"
  }
}
```
</details>

## Statically Validate Transaction

| Function Name     | `statically_validate_transaction` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_staticallyValidateTransaction` |
| Functionality     | Performs static validation on the given notarized transaction. |
| Request Type      | `StaticallyValidateTransactionRequest` |
| Response Type     | `StaticallyValidateTransactionResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "compiled_notarized_intent": "5c2102210221022109070107f20a00020000000000000a10020000000000000a2200000000000000220001b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220221123038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040102b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000042303810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c2101a0000000000502b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000404018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001820000000000000000000000000000000000000000000000000000040d01a0010000000e01a1020000000f01a1020000000f01a10300000023038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f444829163450000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000407000f01a1040000000301a001000000020220b70100010000000000000082000000000000000000000000000000000000000000000000000004100023038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007055c2101a2002020002022060001b200d97c6ee042ea5e522a609bd2b7ea30e32f081e89d9ca4c857a405a50fe16779b3198b51ed7b2ef033e9866643a8125524bd96438bb960ec1c3c02ac86eeabaf10001b20004671578e1f5eac6b80754041380680c4a2aade0173dbac78452e2681b0fdb2b4856070494ee3e1857a3c6ab840a214f2e3b88bbb140b258a5eac5fdfa60001c0001b201607f3ec615db0e08d0b9b8aebe07eaa1f8694c83c1c0d7015c5b1a0ad0371b9c57967a57c302e9145df831b0f083116ae700bb9f50da0111d5ed9630f08b97290102b34cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29b430fccf0b2fff1e261e24fd7d5de8e4b31ce7d01a5ef609acca19e4f75cab58a2ec42f7c80b70ff0f866e4f37ff27108da11f779e97da5f1ad3b239a6f3c492080102b37422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674b4638000d190edda9c5a724cd3cc53a3b3093ebb2a00966830224bbfd29c80f351b416005e7883e5d70e35b5cbab5893c27585f7302cccbb72997be0c7bfe5e5070102b3f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54bb49df6945807370c409e0890efff264637c04acc9289b2ad050910add5dcedab305ec84b73ef1b50c763d9fbe97c85806b16542b465d9d34879922ae06cd91f409220001b200d97c6ee042ea5e522a609bd2b7ea30e32f081e89d9ca4c857a405a50fe16779b3198b51ed7b2ef033e9866643a8125524bd96438bb960ec1c3c02ac86eeabaf1",
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
| Functionality     | Given a network id, this function derives the Bech32m-encoded addresses of the set of known addresses.</br>        </br>        As an example, this function allows users to derive the XRD resource address, faucet component address, or account package address on any network (given that they know its network id). |
| Request Type      | `KnownEntityAddressesRequest` |
| Response Type     | `KnownEntityAddressesResponse` |

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
  "faucet_component_address": {
    "type": "ComponentAddress",
    "address": "component_rdx1qg5fe67sukas49kytvluq72uederg3wfehh0rjfl2qsqe58dfx"
  },
  "faucet_package_address": {
    "type": "PackageAddress",
    "address": "package_rdx1qx9kvuz62mchk6kzwexh4exqerlxreps0h5656mf2a5sevg7d6"
  },
  "account_package_address": {
    "type": "PackageAddress",
    "address": "package_rdx1q8fqfj78l4t4vdlp5rgh6ghke6x7pg8vxmdam6g2y98su3qsvj"
  },
  "xrd_resource_address": {
    "type": "ResourceAddress",
    "address": "resource_rdx1qpflrslzpnprsd27ywcpmm9mqzncshp2sfjg6h59n48say30yn"
  },
  "system_token_resource_address": {
    "type": "ResourceAddress",
    "address": "resource_rdx1qrkx57yl3uxamygyyh370at22450nh2xxgrxmm2dyzqq9pl8qr"
  },
  "ecdsa_secp256k1_token_resource_address": {
    "type": "ResourceAddress",
    "address": "resource_rdx1qzms24rcrka4kdr2pn9zsw8jcghdvw6q2tux0rzq6gfs44jzu2"
  },
  "eddsa_ed25519_token_resource_address": {
    "type": "ResourceAddress",
    "address": "resource_rdx1qraaz8dkuk98cyl0sjhch5ekt3thklgaxpssclgxs0gqdd62m6"
  },
  "epoch_manager_system_address": {
    "type": "SystemAddress",
    "address": "system_rdx1qjjmc80hcmx4sfnyahhg4varselspsrm8gsuqa8dmsxs6y0ppl"
  },
  "clock_system_address": {
    "type": "SystemAddress",
    "address": "system_rdx1q53eheg7mcydm2xms66n397udu8awhhj0xz6c99dtnmsq5rme9"
  }
}
```
</details>
