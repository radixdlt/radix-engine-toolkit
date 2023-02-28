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
  "package_version": "0.8.0",
  "last_commit_hash": "This is just an example. We don't have a commit hash here :)"
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
            "identifier": {
              "type": "String",
              "value": "bucket1"
            }
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
              "identifier": {
                "type": "String",
                "value": "bucket1"
              }
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
            "identifier": {
              "type": "String",
              "value": "bucket2"
            }
          }
        },
        {
          "instruction": "CREATE_PROOF_FROM_BUCKET",
          "bucket": {
            "type": "Bucket",
            "identifier": {
              "type": "String",
              "value": "bucket2"
            }
          },
          "into_proof": {
            "type": "Proof",
            "identifier": {
              "type": "String",
              "value": "proof1"
            }
          }
        },
        {
          "instruction": "CLONE_PROOF",
          "proof": {
            "type": "Proof",
            "identifier": {
              "type": "String",
              "value": "proof1"
            }
          },
          "into_proof": {
            "type": "Proof",
            "identifier": {
              "type": "String",
              "value": "proof2"
            }
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "identifier": {
              "type": "String",
              "value": "proof1"
            }
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "identifier": {
              "type": "String",
              "value": "proof2"
            }
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
            "identifier": {
              "type": "String",
              "value": "proof3"
            }
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "identifier": {
              "type": "String",
              "value": "proof3"
            }
          }
        },
        {
          "instruction": "RETURN_TO_WORKTOP",
          "bucket": {
            "type": "Bucket",
            "identifier": {
              "type": "String",
              "value": "bucket2"
            }
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
              "type": "NonFungibleLocalId",
              "value": {
                "type": "Integer",
                "value": "1"
              }
            }
          ],
          "into_bucket": {
            "type": "Bucket",
            "identifier": {
              "type": "String",
              "value": "bucket3"
            }
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
          "identifier": {
            "type": "String",
            "value": "bucket1"
          }
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
            "identifier": {
              "type": "String",
              "value": "bucket1"
            }
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
          "identifier": {
            "type": "String",
            "value": "bucket2"
          }
        }
      },
      {
        "instruction": "CREATE_PROOF_FROM_BUCKET",
        "bucket": {
          "type": "Bucket",
          "identifier": {
            "type": "String",
            "value": "bucket2"
          }
        },
        "into_proof": {
          "type": "Proof",
          "identifier": {
            "type": "String",
            "value": "proof1"
          }
        }
      },
      {
        "instruction": "CLONE_PROOF",
        "proof": {
          "type": "Proof",
          "identifier": {
            "type": "String",
            "value": "proof1"
          }
        },
        "into_proof": {
          "type": "Proof",
          "identifier": {
            "type": "String",
            "value": "proof2"
          }
        }
      },
      {
        "instruction": "DROP_PROOF",
        "proof": {
          "type": "Proof",
          "identifier": {
            "type": "String",
            "value": "proof1"
          }
        }
      },
      {
        "instruction": "DROP_PROOF",
        "proof": {
          "type": "Proof",
          "identifier": {
            "type": "String",
            "value": "proof2"
          }
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
          "identifier": {
            "type": "String",
            "value": "proof3"
          }
        }
      },
      {
        "instruction": "DROP_PROOF",
        "proof": {
          "type": "Proof",
          "identifier": {
            "type": "String",
            "value": "proof3"
          }
        }
      },
      {
        "instruction": "RETURN_TO_WORKTOP",
        "bucket": {
          "type": "Bucket",
          "identifier": {
            "type": "String",
            "value": "bucket2"
          }
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
            "type": "NonFungibleLocalId",
            "value": {
              "type": "Integer",
              "value": "1"
            }
          }
        ],
        "into_bucket": {
          "type": "Bucket",
          "identifier": {
            "type": "String",
            "value": "bucket3"
          }
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

## Analyze Manifest

| Function Name     | `analyze_manifest` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_analyzeManifest` |
| Functionality     | Analyzes the manifest returning back all of the addresses involved in the manifest alongside some useful information on whether the accounts were withdrawn from, deposited into, or just used in the manifest in general. |
| Request Type      | `AnalyzeManifestRequest` |
| Response Type     | `AnalyzeManifestResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "network_id": "242",
  "manifest": {
    "instructions": {
      "type": "String",
      "value": "CALL_METHOD\n    ComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\")\n    \"withdraw_by_amount\"\n    Decimal(\"5\")\n    ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nTAKE_FROM_WORKTOP_BY_AMOUNT\n    Decimal(\"2\")\n    ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n    Bucket(\"bucket1\");\nCALL_METHOD\n    ComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\")\n    \"buy_gumball\"\n    Bucket(\"bucket1\");\nASSERT_WORKTOP_CONTAINS_BY_AMOUNT\n    Decimal(\"3\")\n    ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nASSERT_WORKTOP_CONTAINS\n    ResourceAddress(\"resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6\");\nTAKE_FROM_WORKTOP\n    ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n    Bucket(\"bucket2\");\nCREATE_PROOF_FROM_BUCKET\n    Bucket(\"bucket2\")\n    Proof(\"proof1\");\nCLONE_PROOF\n    Proof(\"proof1\")\n    Proof(\"proof2\");\nDROP_PROOF\n    Proof(\"proof1\");\nDROP_PROOF\n    Proof(\"proof2\");\nCALL_METHOD\n    ComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\")\n    \"create_proof_by_amount\"\n    Decimal(\"5\")\n    ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nPOP_FROM_AUTH_ZONE\n    Proof(\"proof3\");\nDROP_PROOF\n    Proof(\"proof3\");\nRETURN_TO_WORKTOP\n    Bucket(\"bucket2\");\nTAKE_FROM_WORKTOP_BY_IDS\n    Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"))\n    ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n    Bucket(\"bucket3\");\nDROP_ALL_PROOFS;\nCALL_METHOD\n    ComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\")\n    \"deposit_batch\"\n    Expression(\"ENTIRE_WORKTOP\");\n"
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
  "component_addresses": [
    {
      "type": "ComponentAddress",
      "address": "component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum"
    },
    {
      "type": "ComponentAddress",
      "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
    }
  ],
  "resource_addresses": [
    {
      "type": "ResourceAddress",
      "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
    },
    {
      "type": "ResourceAddress",
      "address": "resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6"
    }
  ],
  "account_addresses": [
    {
      "type": "ComponentAddress",
      "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
    }
  ],
  "accounts_requiring_auth": [
    {
      "type": "ComponentAddress",
      "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
    }
  ],
  "accounts_withdrawn_from": [
    {
      "type": "ComponentAddress",
      "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
    }
  ],
  "accounts_deposited_into": [
    {
      "type": "ComponentAddress",
      "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
    }
  ]
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
            "identifier": {
              "type": "String",
              "value": "bucket1"
            }
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
              "identifier": {
                "type": "String",
                "value": "bucket1"
              }
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
            "identifier": {
              "type": "String",
              "value": "bucket2"
            }
          }
        },
        {
          "instruction": "CREATE_PROOF_FROM_BUCKET",
          "bucket": {
            "type": "Bucket",
            "identifier": {
              "type": "String",
              "value": "bucket2"
            }
          },
          "into_proof": {
            "type": "Proof",
            "identifier": {
              "type": "String",
              "value": "proof1"
            }
          }
        },
        {
          "instruction": "CLONE_PROOF",
          "proof": {
            "type": "Proof",
            "identifier": {
              "type": "String",
              "value": "proof1"
            }
          },
          "into_proof": {
            "type": "Proof",
            "identifier": {
              "type": "String",
              "value": "proof2"
            }
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "identifier": {
              "type": "String",
              "value": "proof1"
            }
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "identifier": {
              "type": "String",
              "value": "proof2"
            }
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
            "identifier": {
              "type": "String",
              "value": "proof3"
            }
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "identifier": {
              "type": "String",
              "value": "proof3"
            }
          }
        },
        {
          "instruction": "RETURN_TO_WORKTOP",
          "bucket": {
            "type": "Bucket",
            "identifier": {
              "type": "String",
              "value": "bucket2"
            }
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
              "type": "NonFungibleLocalId",
              "value": {
                "type": "Integer",
                "value": "1"
              }
            }
          ],
          "into_bucket": {
            "type": "Bucket",
            "identifier": {
              "type": "String",
              "value": "bucket3"
            }
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
  "compiled_intent": "5c21022109070107f20a00020000000000000a10020000000000000a2200000000000000220001b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220221127038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040102b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000042703810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c2101a0000000000502b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000404018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001820000000000000000000000000000000000000000000000000000040d01a0010000000e01a1020000000f01a1020000000f01a10300000027038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f444829163450000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000407000f01a1040000000301a001000000020220b70101000000000000000182000000000000000000000000000000000000000000000000000004100027038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007055c2101a200202000"
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
  "compiled_intent": "5c21022109070107f20a00020000000000000a10020000000000000a2200000000000000220001b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220221127038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040102b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000042703810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c2101a0000000000502b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000404018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001820000000000000000000000000000000000000000000000000000040d01a0010000000e01a1020000000f01a1020000000f01a10300000027038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f444829163450000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000407000f01a1040000000301a001000000020220b70101000000000000000182000000000000000000000000000000000000000000000000000004100027038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007055c2101a200202000"
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
            "identifier": {
              "type": "String",
              "value": "bucket1"
            }
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
              "identifier": {
                "type": "String",
                "value": "bucket1"
              }
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
            "identifier": {
              "type": "String",
              "value": "bucket2"
            }
          }
        },
        {
          "instruction": "CREATE_PROOF_FROM_BUCKET",
          "bucket": {
            "type": "Bucket",
            "identifier": {
              "type": "String",
              "value": "bucket2"
            }
          },
          "into_proof": {
            "type": "Proof",
            "identifier": {
              "type": "String",
              "value": "proof1"
            }
          }
        },
        {
          "instruction": "CLONE_PROOF",
          "proof": {
            "type": "Proof",
            "identifier": {
              "type": "String",
              "value": "proof1"
            }
          },
          "into_proof": {
            "type": "Proof",
            "identifier": {
              "type": "String",
              "value": "proof2"
            }
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "identifier": {
              "type": "String",
              "value": "proof1"
            }
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "identifier": {
              "type": "String",
              "value": "proof2"
            }
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
            "identifier": {
              "type": "String",
              "value": "proof3"
            }
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "identifier": {
              "type": "String",
              "value": "proof3"
            }
          }
        },
        {
          "instruction": "RETURN_TO_WORKTOP",
          "bucket": {
            "type": "Bucket",
            "identifier": {
              "type": "String",
              "value": "bucket2"
            }
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
              "type": "NonFungibleLocalId",
              "value": {
                "type": "Integer",
                "value": "1"
              }
            }
          ],
          "into_bucket": {
            "type": "Bucket",
            "identifier": {
              "type": "String",
              "value": "bucket3"
            }
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
              "identifier": {
                "type": "String",
                "value": "bucket1"
              }
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
                "identifier": {
                  "type": "String",
                  "value": "bucket1"
                }
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
              "identifier": {
                "type": "String",
                "value": "bucket2"
              }
            }
          },
          {
            "instruction": "CREATE_PROOF_FROM_BUCKET",
            "bucket": {
              "type": "Bucket",
              "identifier": {
                "type": "String",
                "value": "bucket2"
              }
            },
            "into_proof": {
              "type": "Proof",
              "identifier": {
                "type": "String",
                "value": "proof1"
              }
            }
          },
          {
            "instruction": "CLONE_PROOF",
            "proof": {
              "type": "Proof",
              "identifier": {
                "type": "String",
                "value": "proof1"
              }
            },
            "into_proof": {
              "type": "Proof",
              "identifier": {
                "type": "String",
                "value": "proof2"
              }
            }
          },
          {
            "instruction": "DROP_PROOF",
            "proof": {
              "type": "Proof",
              "identifier": {
                "type": "String",
                "value": "proof1"
              }
            }
          },
          {
            "instruction": "DROP_PROOF",
            "proof": {
              "type": "Proof",
              "identifier": {
                "type": "String",
                "value": "proof2"
              }
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
              "identifier": {
                "type": "String",
                "value": "proof3"
              }
            }
          },
          {
            "instruction": "DROP_PROOF",
            "proof": {
              "type": "Proof",
              "identifier": {
                "type": "String",
                "value": "proof3"
              }
            }
          },
          {
            "instruction": "RETURN_TO_WORKTOP",
            "bucket": {
              "type": "Bucket",
              "identifier": {
                "type": "String",
                "value": "bucket2"
              }
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
                "type": "NonFungibleLocalId",
                "value": {
                  "type": "Integer",
                  "value": "1"
                }
              }
            ],
            "into_bucket": {
              "type": "Bucket",
              "identifier": {
                "type": "String",
                "value": "bucket3"
              }
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
      "signature": "00a9b850915b885064d78f2e03ca727e30b4df39a005ee9656db09c233a74821e77948c22817bffc000ba375aefeb37b13ad9d2648f92a9efe34ebf3c3a74a30aa"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "0133a742dae4e19a372bfa792fb6176f1516cf9b336c2411138e7b15e5ebaefb2045487a0578b22848a4f86602212a214d443c7ae3dcc04e787b7319ca177ae61e"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "01578ce40c9829b82c95f837f1b28cc2acedc43119f8fb01b978067ca2cbc5b5db73240d1cb39362bf68c4da25ec35c4a88ba58b5dd608b1204a5f35102985e9e8"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
      "signature": "623fefe903d3c9bab8462221c6cfd5e9c0c35cf4676717897049acc22b5ff42cbb89ed33d9647ff81ac5ec8a8bbbb99e33b6b0dd4ddbbf2c18ef1285ee666c08"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
      "signature": "630908f90628964394cb6252b3eebb2ac635f5a09f7b658b183022292de097ce579fac4b532adc7284da45fd52a81a257153d67af20d38a18c600be198951504"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
      "signature": "e3fe9ddf453096b36448624e2bfb55b8a8dcb7d2ebd3fd8704c9923c72e14e5958d79c69b8b81319b9a2eb95f38044bee8efbacbc50abe5765694c740a8fc10d"
    }
  ]
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "compiled_intent": "5c210221022109070107f20a00020000000000000a10020000000000000a2200000000000000220001b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220221127038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040102b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000042703810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c2101a0000000000502b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000404018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001820000000000000000000000000000000000000000000000000000040d01a0010000000e01a1020000000f01a1020000000f01a10300000027038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f444829163450000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000407000f01a1040000000301a001000000020220b70101000000000000000182000000000000000000000000000000000000000000000000000004100027038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007055c2101a2002020002022060001b200a9b850915b885064d78f2e03ca727e30b4df39a005ee9656db09c233a74821e77948c22817bffc000ba375aefeb37b13ad9d2648f92a9efe34ebf3c3a74a30aa0001b20133a742dae4e19a372bfa792fb6176f1516cf9b336c2411138e7b15e5ebaefb2045487a0578b22848a4f86602212a214d443c7ae3dcc04e787b7319ca177ae61e0001b201578ce40c9829b82c95f837f1b28cc2acedc43119f8fb01b978067ca2cbc5b5db73240d1cb39362bf68c4da25ec35c4a88ba58b5dd608b1204a5f35102985e9e80102b34cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29b4623fefe903d3c9bab8462221c6cfd5e9c0c35cf4676717897049acc22b5ff42cbb89ed33d9647ff81ac5ec8a8bbbb99e33b6b0dd4ddbbf2c18ef1285ee666c080102b37422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674b4630908f90628964394cb6252b3eebb2ac635f5a09f7b658b183022292de097ce579fac4b532adc7284da45fd52a81a257153d67af20d38a18c600be1989515040102b3f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54bb4e3fe9ddf453096b36448624e2bfb55b8a8dcb7d2ebd3fd8704c9923c72e14e5958d79c69b8b81319b9a2eb95f38044bee8efbacbc50abe5765694c740a8fc10d"
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
  "compiled_signed_intent": "5c210221022109070107f20a00020000000000000a10020000000000000a2200000000000000220001b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220221127038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040102b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000042703810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c2101a0000000000502b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000404018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001820000000000000000000000000000000000000000000000000000040d01a0010000000e01a1020000000f01a1020000000f01a10300000027038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f444829163450000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000407000f01a1040000000301a001000000020220b70101000000000000000182000000000000000000000000000000000000000000000000000004100027038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007055c2101a2002020002022060001b200a9b850915b885064d78f2e03ca727e30b4df39a005ee9656db09c233a74821e77948c22817bffc000ba375aefeb37b13ad9d2648f92a9efe34ebf3c3a74a30aa0001b20133a742dae4e19a372bfa792fb6176f1516cf9b336c2411138e7b15e5ebaefb2045487a0578b22848a4f86602212a214d443c7ae3dcc04e787b7319ca177ae61e0001b201578ce40c9829b82c95f837f1b28cc2acedc43119f8fb01b978067ca2cbc5b5db73240d1cb39362bf68c4da25ec35c4a88ba58b5dd608b1204a5f35102985e9e80102b34cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29b4623fefe903d3c9bab8462221c6cfd5e9c0c35cf4676717897049acc22b5ff42cbb89ed33d9647ff81ac5ec8a8bbbb99e33b6b0dd4ddbbf2c18ef1285ee666c080102b37422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674b4630908f90628964394cb6252b3eebb2ac635f5a09f7b658b183022292de097ce579fac4b532adc7284da45fd52a81a257153d67af20d38a18c600be1989515040102b3f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54bb4e3fe9ddf453096b36448624e2bfb55b8a8dcb7d2ebd3fd8704c9923c72e14e5958d79c69b8b81319b9a2eb95f38044bee8efbacbc50abe5765694c740a8fc10d"
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
              "identifier": {
                "type": "String",
                "value": "bucket1"
              }
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
                "identifier": {
                  "type": "String",
                  "value": "bucket1"
                }
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
              "identifier": {
                "type": "String",
                "value": "bucket2"
              }
            }
          },
          {
            "instruction": "CREATE_PROOF_FROM_BUCKET",
            "bucket": {
              "type": "Bucket",
              "identifier": {
                "type": "String",
                "value": "bucket2"
              }
            },
            "into_proof": {
              "type": "Proof",
              "identifier": {
                "type": "String",
                "value": "proof1"
              }
            }
          },
          {
            "instruction": "CLONE_PROOF",
            "proof": {
              "type": "Proof",
              "identifier": {
                "type": "String",
                "value": "proof1"
              }
            },
            "into_proof": {
              "type": "Proof",
              "identifier": {
                "type": "String",
                "value": "proof2"
              }
            }
          },
          {
            "instruction": "DROP_PROOF",
            "proof": {
              "type": "Proof",
              "identifier": {
                "type": "String",
                "value": "proof1"
              }
            }
          },
          {
            "instruction": "DROP_PROOF",
            "proof": {
              "type": "Proof",
              "identifier": {
                "type": "String",
                "value": "proof2"
              }
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
              "identifier": {
                "type": "String",
                "value": "proof3"
              }
            }
          },
          {
            "instruction": "DROP_PROOF",
            "proof": {
              "type": "Proof",
              "identifier": {
                "type": "String",
                "value": "proof3"
              }
            }
          },
          {
            "instruction": "RETURN_TO_WORKTOP",
            "bucket": {
              "type": "Bucket",
              "identifier": {
                "type": "String",
                "value": "bucket2"
              }
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
                "type": "NonFungibleLocalId",
                "value": {
                  "type": "Integer",
                  "value": "1"
                }
              }
            ],
            "into_bucket": {
              "type": "Bucket",
              "identifier": {
                "type": "String",
                "value": "bucket3"
              }
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
      "signature": "00a9b850915b885064d78f2e03ca727e30b4df39a005ee9656db09c233a74821e77948c22817bffc000ba375aefeb37b13ad9d2648f92a9efe34ebf3c3a74a30aa"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "0133a742dae4e19a372bfa792fb6176f1516cf9b336c2411138e7b15e5ebaefb2045487a0578b22848a4f86602212a214d443c7ae3dcc04e787b7319ca177ae61e"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "01578ce40c9829b82c95f837f1b28cc2acedc43119f8fb01b978067ca2cbc5b5db73240d1cb39362bf68c4da25ec35c4a88ba58b5dd608b1204a5f35102985e9e8"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
      "signature": "623fefe903d3c9bab8462221c6cfd5e9c0c35cf4676717897049acc22b5ff42cbb89ed33d9647ff81ac5ec8a8bbbb99e33b6b0dd4ddbbf2c18ef1285ee666c08"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
      "signature": "630908f90628964394cb6252b3eebb2ac635f5a09f7b658b183022292de097ce579fac4b532adc7284da45fd52a81a257153d67af20d38a18c600be198951504"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
      "signature": "e3fe9ddf453096b36448624e2bfb55b8a8dcb7d2ebd3fd8704c9923c72e14e5958d79c69b8b81319b9a2eb95f38044bee8efbacbc50abe5765694c740a8fc10d"
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
                "identifier": {
                  "type": "String",
                  "value": "bucket1"
                }
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
                  "identifier": {
                    "type": "String",
                    "value": "bucket1"
                  }
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
                "identifier": {
                  "type": "String",
                  "value": "bucket2"
                }
              }
            },
            {
              "instruction": "CREATE_PROOF_FROM_BUCKET",
              "bucket": {
                "type": "Bucket",
                "identifier": {
                  "type": "String",
                  "value": "bucket2"
                }
              },
              "into_proof": {
                "type": "Proof",
                "identifier": {
                  "type": "String",
                  "value": "proof1"
                }
              }
            },
            {
              "instruction": "CLONE_PROOF",
              "proof": {
                "type": "Proof",
                "identifier": {
                  "type": "String",
                  "value": "proof1"
                }
              },
              "into_proof": {
                "type": "Proof",
                "identifier": {
                  "type": "String",
                  "value": "proof2"
                }
              }
            },
            {
              "instruction": "DROP_PROOF",
              "proof": {
                "type": "Proof",
                "identifier": {
                  "type": "String",
                  "value": "proof1"
                }
              }
            },
            {
              "instruction": "DROP_PROOF",
              "proof": {
                "type": "Proof",
                "identifier": {
                  "type": "String",
                  "value": "proof2"
                }
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
                "identifier": {
                  "type": "String",
                  "value": "proof3"
                }
              }
            },
            {
              "instruction": "DROP_PROOF",
              "proof": {
                "type": "Proof",
                "identifier": {
                  "type": "String",
                  "value": "proof3"
                }
              }
            },
            {
              "instruction": "RETURN_TO_WORKTOP",
              "bucket": {
                "type": "Bucket",
                "identifier": {
                  "type": "String",
                  "value": "bucket2"
                }
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
                  "type": "NonFungibleLocalId",
                  "value": {
                    "type": "Integer",
                    "value": "1"
                  }
                }
              ],
              "into_bucket": {
                "type": "Bucket",
                "identifier": {
                  "type": "String",
                  "value": "bucket3"
                }
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
        "signature": "00a9b850915b885064d78f2e03ca727e30b4df39a005ee9656db09c233a74821e77948c22817bffc000ba375aefeb37b13ad9d2648f92a9efe34ebf3c3a74a30aa"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "0133a742dae4e19a372bfa792fb6176f1516cf9b336c2411138e7b15e5ebaefb2045487a0578b22848a4f86602212a214d443c7ae3dcc04e787b7319ca177ae61e"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "01578ce40c9829b82c95f837f1b28cc2acedc43119f8fb01b978067ca2cbc5b5db73240d1cb39362bf68c4da25ec35c4a88ba58b5dd608b1204a5f35102985e9e8"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
        "signature": "623fefe903d3c9bab8462221c6cfd5e9c0c35cf4676717897049acc22b5ff42cbb89ed33d9647ff81ac5ec8a8bbbb99e33b6b0dd4ddbbf2c18ef1285ee666c08"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
        "signature": "630908f90628964394cb6252b3eebb2ac635f5a09f7b658b183022292de097ce579fac4b532adc7284da45fd52a81a257153d67af20d38a18c600be198951504"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
        "signature": "e3fe9ddf453096b36448624e2bfb55b8a8dcb7d2ebd3fd8704c9923c72e14e5958d79c69b8b81319b9a2eb95f38044bee8efbacbc50abe5765694c740a8fc10d"
      }
    ]
  },
  "notary_signature": {
    "curve": "EcdsaSecp256k1",
    "signature": "0161d5430b6d8051ed9b4a68e96cedd25594afb3da0d8fdcbb5783faffbce188185345bbd0eefb743860614be4184a2c2d5374fd71c7cbb8422b38f89e9c79fd00"
  }
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "compiled_intent": "5c2102210221022109070107f20a00020000000000000a10020000000000000a2200000000000000220001b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220221127038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040102b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000042703810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c2101a0000000000502b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000404018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001820000000000000000000000000000000000000000000000000000040d01a0010000000e01a1020000000f01a1020000000f01a10300000027038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f444829163450000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000407000f01a1040000000301a001000000020220b70101000000000000000182000000000000000000000000000000000000000000000000000004100027038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007055c2101a2002020002022060001b200a9b850915b885064d78f2e03ca727e30b4df39a005ee9656db09c233a74821e77948c22817bffc000ba375aefeb37b13ad9d2648f92a9efe34ebf3c3a74a30aa0001b20133a742dae4e19a372bfa792fb6176f1516cf9b336c2411138e7b15e5ebaefb2045487a0578b22848a4f86602212a214d443c7ae3dcc04e787b7319ca177ae61e0001b201578ce40c9829b82c95f837f1b28cc2acedc43119f8fb01b978067ca2cbc5b5db73240d1cb39362bf68c4da25ec35c4a88ba58b5dd608b1204a5f35102985e9e80102b34cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29b4623fefe903d3c9bab8462221c6cfd5e9c0c35cf4676717897049acc22b5ff42cbb89ed33d9647ff81ac5ec8a8bbbb99e33b6b0dd4ddbbf2c18ef1285ee666c080102b37422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674b4630908f90628964394cb6252b3eebb2ac635f5a09f7b658b183022292de097ce579fac4b532adc7284da45fd52a81a257153d67af20d38a18c600be1989515040102b3f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54bb4e3fe9ddf453096b36448624e2bfb55b8a8dcb7d2ebd3fd8704c9923c72e14e5958d79c69b8b81319b9a2eb95f38044bee8efbacbc50abe5765694c740a8fc10d220001b20161d5430b6d8051ed9b4a68e96cedd25594afb3da0d8fdcbb5783faffbce188185345bbd0eefb743860614be4184a2c2d5374fd71c7cbb8422b38f89e9c79fd00"
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
  "compiled_notarized_intent": "5c2102210221022109070107f20a00020000000000000a10020000000000000a2200000000000000220001b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220221127038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040102b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000042703810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c2101a0000000000502b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000404018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001820000000000000000000000000000000000000000000000000000040d01a0010000000e01a1020000000f01a1020000000f01a10300000027038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f444829163450000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000407000f01a1040000000301a001000000020220b70101000000000000000182000000000000000000000000000000000000000000000000000004100027038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007055c2101a2002020002022060001b200a9b850915b885064d78f2e03ca727e30b4df39a005ee9656db09c233a74821e77948c22817bffc000ba375aefeb37b13ad9d2648f92a9efe34ebf3c3a74a30aa0001b20133a742dae4e19a372bfa792fb6176f1516cf9b336c2411138e7b15e5ebaefb2045487a0578b22848a4f86602212a214d443c7ae3dcc04e787b7319ca177ae61e0001b201578ce40c9829b82c95f837f1b28cc2acedc43119f8fb01b978067ca2cbc5b5db73240d1cb39362bf68c4da25ec35c4a88ba58b5dd608b1204a5f35102985e9e80102b34cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29b4623fefe903d3c9bab8462221c6cfd5e9c0c35cf4676717897049acc22b5ff42cbb89ed33d9647ff81ac5ec8a8bbbb99e33b6b0dd4ddbbf2c18ef1285ee666c080102b37422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674b4630908f90628964394cb6252b3eebb2ac635f5a09f7b658b183022292de097ce579fac4b532adc7284da45fd52a81a257153d67af20d38a18c600be1989515040102b3f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54bb4e3fe9ddf453096b36448624e2bfb55b8a8dcb7d2ebd3fd8704c9923c72e14e5958d79c69b8b81319b9a2eb95f38044bee8efbacbc50abe5765694c740a8fc10d220001b20161d5430b6d8051ed9b4a68e96cedd25594afb3da0d8fdcbb5783faffbce188185345bbd0eefb743860614be4184a2c2d5374fd71c7cbb8422b38f89e9c79fd00"
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
                "identifier": {
                  "type": "String",
                  "value": "bucket1"
                }
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
                  "identifier": {
                    "type": "String",
                    "value": "bucket1"
                  }
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
                "identifier": {
                  "type": "String",
                  "value": "bucket2"
                }
              }
            },
            {
              "instruction": "CREATE_PROOF_FROM_BUCKET",
              "bucket": {
                "type": "Bucket",
                "identifier": {
                  "type": "String",
                  "value": "bucket2"
                }
              },
              "into_proof": {
                "type": "Proof",
                "identifier": {
                  "type": "String",
                  "value": "proof1"
                }
              }
            },
            {
              "instruction": "CLONE_PROOF",
              "proof": {
                "type": "Proof",
                "identifier": {
                  "type": "String",
                  "value": "proof1"
                }
              },
              "into_proof": {
                "type": "Proof",
                "identifier": {
                  "type": "String",
                  "value": "proof2"
                }
              }
            },
            {
              "instruction": "DROP_PROOF",
              "proof": {
                "type": "Proof",
                "identifier": {
                  "type": "String",
                  "value": "proof1"
                }
              }
            },
            {
              "instruction": "DROP_PROOF",
              "proof": {
                "type": "Proof",
                "identifier": {
                  "type": "String",
                  "value": "proof2"
                }
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
                "identifier": {
                  "type": "String",
                  "value": "proof3"
                }
              }
            },
            {
              "instruction": "DROP_PROOF",
              "proof": {
                "type": "Proof",
                "identifier": {
                  "type": "String",
                  "value": "proof3"
                }
              }
            },
            {
              "instruction": "RETURN_TO_WORKTOP",
              "bucket": {
                "type": "Bucket",
                "identifier": {
                  "type": "String",
                  "value": "bucket2"
                }
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
                  "type": "NonFungibleLocalId",
                  "value": {
                    "type": "Integer",
                    "value": "1"
                  }
                }
              ],
              "into_bucket": {
                "type": "Bucket",
                "identifier": {
                  "type": "String",
                  "value": "bucket3"
                }
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
        "signature": "00a9b850915b885064d78f2e03ca727e30b4df39a005ee9656db09c233a74821e77948c22817bffc000ba375aefeb37b13ad9d2648f92a9efe34ebf3c3a74a30aa"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "0133a742dae4e19a372bfa792fb6176f1516cf9b336c2411138e7b15e5ebaefb2045487a0578b22848a4f86602212a214d443c7ae3dcc04e787b7319ca177ae61e"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "01578ce40c9829b82c95f837f1b28cc2acedc43119f8fb01b978067ca2cbc5b5db73240d1cb39362bf68c4da25ec35c4a88ba58b5dd608b1204a5f35102985e9e8"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
        "signature": "623fefe903d3c9bab8462221c6cfd5e9c0c35cf4676717897049acc22b5ff42cbb89ed33d9647ff81ac5ec8a8bbbb99e33b6b0dd4ddbbf2c18ef1285ee666c08"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
        "signature": "630908f90628964394cb6252b3eebb2ac635f5a09f7b658b183022292de097ce579fac4b532adc7284da45fd52a81a257153d67af20d38a18c600be198951504"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
        "signature": "e3fe9ddf453096b36448624e2bfb55b8a8dcb7d2ebd3fd8704c9923c72e14e5958d79c69b8b81319b9a2eb95f38044bee8efbacbc50abe5765694c740a8fc10d"
      }
    ]
  },
  "notary_signature": {
    "curve": "EcdsaSecp256k1",
    "signature": "0161d5430b6d8051ed9b4a68e96cedd25594afb3da0d8fdcbb5783faffbce188185345bbd0eefb743860614be4184a2c2d5374fd71c7cbb8422b38f89e9c79fd00"
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
  "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpqd60rqz"
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
  "value": {
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
    "address": "account_sim1qupveqrdmh7kw0vefxrzjw5e0fgw3sgdj4zrh66jkphqj9h5gl"
  }
}
```
</details>

## Derive Virtual Identity Address

| Function Name     | `derive_virtual_identity_address` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_deriveVirtualIdentityAddress` |
| Functionality     | Derives the virtual identity component address given a public key and a network id. |
| Request Type      | `DeriveVirtualIdentityAddressRequest` |
| Response Type     | `DeriveVirtualIdentityAddressResponse` |

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
  "virtual_identity_address": {
    "type": "ComponentAddress",
    "address": "identity_sim1pgpveqrdmh7kw0vefxrzjw5e0fgw3sgdj4zrh66jkphq06jcsf"
  }
}
```
</details>

## Derive Non Fungible Global Id From Public Key

| Function Name     | `derive_non_fungible_global_id_from_public_key` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_deriveNonFungibleGlobalIdFromPublicKey` |
| Functionality     | Derives the non-fungible global id of the virtual badge associated with a given public key |
| Request Type      | `DeriveNonFungibleGlobalIdFromPublicKeyRequest` |
| Response Type     | `DeriveNonFungibleGlobalIdFromPublicKeyResponse` |

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
  "type": "NonFungibleGlobalId",
  "resource_address": {
    "type": "ResourceAddress",
    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqshxgp7h"
  },
  "non_fungible_local_id": {
    "type": "NonFungibleLocalId",
    "value": {
      "type": "Bytes",
      "value": "02cc806dddfd673d994986293a997a50e8c10d95443beb52b06e"
    }
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
  "compiled_notarized_intent": "5c2102210221022109070107f20a00020000000000000a10020000000000000a2200000000000000220001b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220221127038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040102b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000042703810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c2101a0000000000502b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000404018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001820000000000000000000000000000000000000000000000000000040d01a0010000000e01a1020000000f01a1020000000f01a10300000027038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f444829163450000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000407000f01a1040000000301a001000000020220b70101000000000000000182000000000000000000000000000000000000000000000000000004100027038103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007055c2101a2002020002022060001b200a9b850915b885064d78f2e03ca727e30b4df39a005ee9656db09c233a74821e77948c22817bffc000ba375aefeb37b13ad9d2648f92a9efe34ebf3c3a74a30aa0001b20133a742dae4e19a372bfa792fb6176f1516cf9b336c2411138e7b15e5ebaefb2045487a0578b22848a4f86602212a214d443c7ae3dcc04e787b7319ca177ae61e0001b201578ce40c9829b82c95f837f1b28cc2acedc43119f8fb01b978067ca2cbc5b5db73240d1cb39362bf68c4da25ec35c4a88ba58b5dd608b1204a5f35102985e9e80102b34cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29b4623fefe903d3c9bab8462221c6cfd5e9c0c35cf4676717897049acc22b5ff42cbb89ed33d9647ff81ac5ec8a8bbbb99e33b6b0dd4ddbbf2c18ef1285ee666c080102b37422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674b4630908f90628964394cb6252b3eebb2ac635f5a09f7b658b183022292de097ce579fac4b532adc7284da45fd52a81a257153d67af20d38a18c600be1989515040102b3f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54bb4e3fe9ddf453096b36448624e2bfb55b8a8dcb7d2ebd3fd8704c9923c72e14e5958d79c69b8b81319b9a2eb95f38044bee8efbacbc50abe5765694c740a8fc10d220001b200a9b850915b885064d78f2e03ca727e30b4df39a005ee9656db09c233a74821e77948c22817bffc000ba375aefeb37b13ad9d2648f92a9efe34ebf3c3a74a30aa",
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
    "address": "component_rdx1qgehpqdhhr62xh76wh6gppnyn88a0uau68epljprvj3sq5kftu"
  },
  "faucet_package_address": {
    "type": "PackageAddress",
    "address": "package_rdx1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqp7hql"
  },
  "account_package_address": {
    "type": "PackageAddress",
    "address": "package_rdx1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs4qk04v"
  },
  "xrd_resource_address": {
    "type": "ResourceAddress",
    "address": "resource_rdx1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqy99qqm"
  },
  "system_token_resource_address": {
    "type": "ResourceAddress",
    "address": "resource_rdx1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqps7ezz7w"
  },
  "ecdsa_secp256k1_token_resource_address": {
    "type": "ResourceAddress",
    "address": "resource_rdx1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs3ydc4g"
  },
  "eddsa_ed25519_token_resource_address": {
    "type": "ResourceAddress",
    "address": "resource_rdx1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpqtc26ta"
  },
  "package_token_resource_address": {
    "type": "ResourceAddress",
    "address": "resource_rdx1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzq6kmakh"
  },
  "epoch_manager_system_address": {
    "type": "ComponentAddress",
    "address": "epochmanager_rdx1qsqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq2xdjnl"
  },
  "clock_system_address": {
    "type": "ComponentAddress",
    "address": "clock_rdx1qcqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqm2y56s"
  }
}
```
</details>
