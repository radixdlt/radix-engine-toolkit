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
  "package_version": "0.9.0",
  "last_commit_hash": "This is just an example. We don't have a commit hash here"
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
            "type": "Address",
            "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
          },
          "method_name": {
            "type": "String",
            "value": "withdraw"
          },
          "arguments": [
            {
              "type": "Address",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            },
            {
              "type": "Decimal",
              "value": "5"
            }
          ]
        },
        {
          "instruction": "TAKE_FROM_WORKTOP_BY_AMOUNT",
          "resource_address": {
            "type": "Address",
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
            "type": "Address",
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
            "type": "Address",
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
            "type": "Address",
            "address": "resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6"
          }
        },
        {
          "instruction": "TAKE_FROM_WORKTOP",
          "resource_address": {
            "type": "Address",
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
            "type": "Address",
            "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
          },
          "method_name": {
            "type": "String",
            "value": "create_proof_by_amount"
          },
          "arguments": [
            {
              "type": "Address",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            },
            {
              "type": "Decimal",
              "value": "5"
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
            "type": "Address",
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
            "type": "Address",
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
          "value": "withdraw"
        },
        "arguments": [
          {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          },
          {
            "type": "Decimal",
            "value": "5"
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
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          },
          {
            "type": "Decimal",
            "value": "5"
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
      "value": "CALL_METHOD\n    Address(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\")\n    \"withdraw\"\n    Address(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n    Decimal(\"5\");\nTAKE_FROM_WORKTOP_BY_AMOUNT\n    Decimal(\"2\")\n    Address(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n    Bucket(\"bucket1\");\nCALL_METHOD\n    Address(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\")\n    \"buy_gumball\"\n    Bucket(\"bucket1\");\nASSERT_WORKTOP_CONTAINS_BY_AMOUNT\n    Decimal(\"3\")\n    Address(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nASSERT_WORKTOP_CONTAINS\n    Address(\"resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6\");\nTAKE_FROM_WORKTOP\n    Address(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n    Bucket(\"bucket2\");\nCREATE_PROOF_FROM_BUCKET\n    Bucket(\"bucket2\")\n    Proof(\"proof1\");\nCLONE_PROOF\n    Proof(\"proof1\")\n    Proof(\"proof2\");\nDROP_PROOF\n    Proof(\"proof1\");\nDROP_PROOF\n    Proof(\"proof2\");\nCALL_METHOD\n    Address(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\")\n    \"create_proof_by_amount\"\n    Address(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n    Decimal(\"5\");\nPOP_FROM_AUTH_ZONE\n    Proof(\"proof3\");\nDROP_PROOF\n    Proof(\"proof3\");\nRETURN_TO_WORKTOP\n    Bucket(\"bucket2\");\nTAKE_FROM_WORKTOP_BY_IDS\n    Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"))\n    Address(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n    Bucket(\"bucket3\");\nDROP_ALL_PROOFS;\nCALL_METHOD\n    Address(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\")\n    \"deposit_batch\"\n    Expression(\"ENTIRE_WORKTOP\");\n"
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
            "type": "Address",
            "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
          },
          "method_name": {
            "type": "String",
            "value": "withdraw"
          },
          "arguments": [
            {
              "type": "Address",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            },
            {
              "type": "Decimal",
              "value": "5"
            }
          ]
        },
        {
          "instruction": "TAKE_FROM_WORKTOP_BY_AMOUNT",
          "resource_address": {
            "type": "Address",
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
            "type": "Address",
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
            "type": "Address",
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
            "type": "Address",
            "address": "resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6"
          }
        },
        {
          "instruction": "TAKE_FROM_WORKTOP",
          "resource_address": {
            "type": "Address",
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
            "type": "Address",
            "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
          },
          "method_name": {
            "type": "String",
            "value": "create_proof_by_amount"
          },
          "arguments": [
            {
              "type": "Address",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            },
            {
              "type": "Decimal",
              "value": "5"
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
            "type": "Address",
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
            "type": "Address",
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
  "compiled_intent": "4d21022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f50508000021022022111f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0877697468647261772007404d210280000000000000000000000000000000000000000000000000000004850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000041f03800292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007084d2101810000000005028500002cf61a24a2290000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000404018000aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001800000000000000000000000000000000000000000000000000000040d0181010000000e0182020000000f0182020000000f0182030000001f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007404d210280000000000000000000000000000000000000000000000000000004850000f4448291634500000000000000000000000000000000000000000000000007000f0182040000000301810100000002022087010100000000000000018000000000000000000000000000000000000000000000000000000410001f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007054d21018300202000"
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
  "compiled_intent": "4d21022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f50508000021022022111f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0877697468647261772007404d210280000000000000000000000000000000000000000000000000000004850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000041f03800292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007084d2101810000000005028500002cf61a24a2290000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000404018000aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001800000000000000000000000000000000000000000000000000000040d0181010000000e0182020000000f0182020000000f0182030000001f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007404d210280000000000000000000000000000000000000000000000000000004850000f4448291634500000000000000000000000000000000000000000000000007000f0182040000000301810100000002022087010100000000000000018000000000000000000000000000000000000000000000000000000410001f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007054d21018300202000"
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
            "value": "withdraw"
          },
          "arguments": [
            {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            },
            {
              "type": "Decimal",
              "value": "5"
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
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            },
            {
              "type": "Decimal",
              "value": "5"
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
              "type": "Address",
              "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
            },
            "method_name": {
              "type": "String",
              "value": "withdraw"
            },
            "arguments": [
              {
                "type": "Address",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              },
              {
                "type": "Decimal",
                "value": "5"
              }
            ]
          },
          {
            "instruction": "TAKE_FROM_WORKTOP_BY_AMOUNT",
            "resource_address": {
              "type": "Address",
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
              "type": "Address",
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
              "type": "Address",
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
              "type": "Address",
              "address": "resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6"
            }
          },
          {
            "instruction": "TAKE_FROM_WORKTOP",
            "resource_address": {
              "type": "Address",
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
              "type": "Address",
              "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
            },
            "method_name": {
              "type": "String",
              "value": "create_proof_by_amount"
            },
            "arguments": [
              {
                "type": "Address",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              },
              {
                "type": "Decimal",
                "value": "5"
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
              "type": "Address",
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
              "type": "Address",
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
      "signature": "0065ad0bb4ebe66451b760156fe7c1780beae38f99dc81d690fe951f6f143c596533543908fb6b2664f4c551ef0134fbd6d91feb4374cebb0aaf746de17189e1be"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "00cacb6ca003adfbe6a2e7210613ceb5bcff1377f295eb448f0bf44e71ff30e7d66c050f2fa053bea17657dcaba31ae53454f0f52ba49a8ffc544c48a83fc6f067"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "01461c46a081894cf99f09f87329333200420031214df82893dbc8fe2847cdc4914177caf78577dca8256beffac8b424532c52576c1df33af962b11304862e3696"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
      "signature": "fe98f54c1094e5889c15ee4d5a941b3e3159cdd40fcc8b73764cabb37c3401dbbb7d466a34a38d7f32aadb213418e5af7e642a9ebea4085b990021029e75ba09"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
      "signature": "f75b8424f5f58ce4bb5ca851c21c66a4a33d4cd13123b6afd476cd242e5b89da6407bd6fccf33acb3ba9eadc136642a178cdddfd339fa886d37ec0852d6a1e0b"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
      "signature": "4fb69776587a85473bdd3cbdbcf685061d2d64eb6f6720629cc7866105f297f36909fb41fc077400f0d9622bfc787a48b1b92411f964006b4453773d8e2b420c"
    }
  ]
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "compiled_intent": "4d210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f50508000021022022111f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0877697468647261772007404d210280000000000000000000000000000000000000000000000000000004850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000041f03800292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007084d2101810000000005028500002cf61a24a2290000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000404018000aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001800000000000000000000000000000000000000000000000000000040d0181010000000e0182020000000f0182020000000f0182030000001f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007404d210280000000000000000000000000000000000000000000000000000004850000f4448291634500000000000000000000000000000000000000000000000007000f0182040000000301810100000002022087010100000000000000018000000000000000000000000000000000000000000000000000000410001f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007054d21018300202000202206000121012007410065ad0bb4ebe66451b760156fe7c1780beae38f99dc81d690fe951f6f143c596533543908fb6b2664f4c551ef0134fbd6d91feb4374cebb0aaf746de17189e1be0001210120074100cacb6ca003adfbe6a2e7210613ceb5bcff1377f295eb448f0bf44e71ff30e7d66c050f2fa053bea17657dcaba31ae53454f0f52ba49a8ffc544c48a83fc6f0670001210120074101461c46a081894cf99f09f87329333200420031214df82893dbc8fe2847cdc4914177caf78577dca8256beffac8b424532c52576c1df33af962b11304862e369601022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740fe98f54c1094e5889c15ee4d5a941b3e3159cdd40fcc8b73764cabb37c3401dbbb7d466a34a38d7f32aadb213418e5af7e642a9ebea4085b990021029e75ba0901022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740f75b8424f5f58ce4bb5ca851c21c66a4a33d4cd13123b6afd476cd242e5b89da6407bd6fccf33acb3ba9eadc136642a178cdddfd339fa886d37ec0852d6a1e0b0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b21012007404fb69776587a85473bdd3cbdbcf685061d2d64eb6f6720629cc7866105f297f36909fb41fc077400f0d9622bfc787a48b1b92411f964006b4453773d8e2b420c"
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
  "compiled_signed_intent": "4d210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f50508000021022022111f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0877697468647261772007404d210280000000000000000000000000000000000000000000000000000004850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000041f03800292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007084d2101810000000005028500002cf61a24a2290000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000404018000aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001800000000000000000000000000000000000000000000000000000040d0181010000000e0182020000000f0182020000000f0182030000001f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007404d210280000000000000000000000000000000000000000000000000000004850000f4448291634500000000000000000000000000000000000000000000000007000f0182040000000301810100000002022087010100000000000000018000000000000000000000000000000000000000000000000000000410001f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007054d21018300202000202206000121012007410065ad0bb4ebe66451b760156fe7c1780beae38f99dc81d690fe951f6f143c596533543908fb6b2664f4c551ef0134fbd6d91feb4374cebb0aaf746de17189e1be0001210120074100cacb6ca003adfbe6a2e7210613ceb5bcff1377f295eb448f0bf44e71ff30e7d66c050f2fa053bea17657dcaba31ae53454f0f52ba49a8ffc544c48a83fc6f0670001210120074101461c46a081894cf99f09f87329333200420031214df82893dbc8fe2847cdc4914177caf78577dca8256beffac8b424532c52576c1df33af962b11304862e369601022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740fe98f54c1094e5889c15ee4d5a941b3e3159cdd40fcc8b73764cabb37c3401dbbb7d466a34a38d7f32aadb213418e5af7e642a9ebea4085b990021029e75ba0901022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740f75b8424f5f58ce4bb5ca851c21c66a4a33d4cd13123b6afd476cd242e5b89da6407bd6fccf33acb3ba9eadc136642a178cdddfd339fa886d37ec0852d6a1e0b0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b21012007404fb69776587a85473bdd3cbdbcf685061d2d64eb6f6720629cc7866105f297f36909fb41fc077400f0d9622bfc787a48b1b92411f964006b4453773d8e2b420c"
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
              "value": "withdraw"
            },
            "arguments": [
              {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              },
              {
                "type": "Decimal",
                "value": "5"
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
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              },
              {
                "type": "Decimal",
                "value": "5"
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
      "signature": "0065ad0bb4ebe66451b760156fe7c1780beae38f99dc81d690fe951f6f143c596533543908fb6b2664f4c551ef0134fbd6d91feb4374cebb0aaf746de17189e1be"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "00cacb6ca003adfbe6a2e7210613ceb5bcff1377f295eb448f0bf44e71ff30e7d66c050f2fa053bea17657dcaba31ae53454f0f52ba49a8ffc544c48a83fc6f067"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "01461c46a081894cf99f09f87329333200420031214df82893dbc8fe2847cdc4914177caf78577dca8256beffac8b424532c52576c1df33af962b11304862e3696"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
      "signature": "fe98f54c1094e5889c15ee4d5a941b3e3159cdd40fcc8b73764cabb37c3401dbbb7d466a34a38d7f32aadb213418e5af7e642a9ebea4085b990021029e75ba09"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
      "signature": "f75b8424f5f58ce4bb5ca851c21c66a4a33d4cd13123b6afd476cd242e5b89da6407bd6fccf33acb3ba9eadc136642a178cdddfd339fa886d37ec0852d6a1e0b"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
      "signature": "4fb69776587a85473bdd3cbdbcf685061d2d64eb6f6720629cc7866105f297f36909fb41fc077400f0d9622bfc787a48b1b92411f964006b4453773d8e2b420c"
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
                "type": "Address",
                "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
              },
              "method_name": {
                "type": "String",
                "value": "withdraw"
              },
              "arguments": [
                {
                  "type": "Address",
                  "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
                },
                {
                  "type": "Decimal",
                  "value": "5"
                }
              ]
            },
            {
              "instruction": "TAKE_FROM_WORKTOP_BY_AMOUNT",
              "resource_address": {
                "type": "Address",
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
                "type": "Address",
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
                "type": "Address",
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
                "type": "Address",
                "address": "resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6"
              }
            },
            {
              "instruction": "TAKE_FROM_WORKTOP",
              "resource_address": {
                "type": "Address",
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
                "type": "Address",
                "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
              },
              "method_name": {
                "type": "String",
                "value": "create_proof_by_amount"
              },
              "arguments": [
                {
                  "type": "Address",
                  "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
                },
                {
                  "type": "Decimal",
                  "value": "5"
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
                "type": "Address",
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
                "type": "Address",
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
        "signature": "0065ad0bb4ebe66451b760156fe7c1780beae38f99dc81d690fe951f6f143c596533543908fb6b2664f4c551ef0134fbd6d91feb4374cebb0aaf746de17189e1be"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "00cacb6ca003adfbe6a2e7210613ceb5bcff1377f295eb448f0bf44e71ff30e7d66c050f2fa053bea17657dcaba31ae53454f0f52ba49a8ffc544c48a83fc6f067"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "01461c46a081894cf99f09f87329333200420031214df82893dbc8fe2847cdc4914177caf78577dca8256beffac8b424532c52576c1df33af962b11304862e3696"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
        "signature": "fe98f54c1094e5889c15ee4d5a941b3e3159cdd40fcc8b73764cabb37c3401dbbb7d466a34a38d7f32aadb213418e5af7e642a9ebea4085b990021029e75ba09"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
        "signature": "f75b8424f5f58ce4bb5ca851c21c66a4a33d4cd13123b6afd476cd242e5b89da6407bd6fccf33acb3ba9eadc136642a178cdddfd339fa886d37ec0852d6a1e0b"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
        "signature": "4fb69776587a85473bdd3cbdbcf685061d2d64eb6f6720629cc7866105f297f36909fb41fc077400f0d9622bfc787a48b1b92411f964006b4453773d8e2b420c"
      }
    ]
  },
  "notary_signature": {
    "curve": "EcdsaSecp256k1",
    "signature": "012fb66c12ac06582d21b10fa375530999b30084c3d5c7226e1559a4c5cab02096584acdf555109b67683a727660cb798efe8d7e13bf83c1e41a510f85566bfa41"
  }
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "compiled_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f50508000021022022111f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0877697468647261772007404d210280000000000000000000000000000000000000000000000000000004850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000041f03800292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007084d2101810000000005028500002cf61a24a2290000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000404018000aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001800000000000000000000000000000000000000000000000000000040d0181010000000e0182020000000f0182020000000f0182030000001f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007404d210280000000000000000000000000000000000000000000000000000004850000f4448291634500000000000000000000000000000000000000000000000007000f0182040000000301810100000002022087010100000000000000018000000000000000000000000000000000000000000000000000000410001f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007054d21018300202000202206000121012007410065ad0bb4ebe66451b760156fe7c1780beae38f99dc81d690fe951f6f143c596533543908fb6b2664f4c551ef0134fbd6d91feb4374cebb0aaf746de17189e1be0001210120074100cacb6ca003adfbe6a2e7210613ceb5bcff1377f295eb448f0bf44e71ff30e7d66c050f2fa053bea17657dcaba31ae53454f0f52ba49a8ffc544c48a83fc6f0670001210120074101461c46a081894cf99f09f87329333200420031214df82893dbc8fe2847cdc4914177caf78577dca8256beffac8b424532c52576c1df33af962b11304862e369601022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740fe98f54c1094e5889c15ee4d5a941b3e3159cdd40fcc8b73764cabb37c3401dbbb7d466a34a38d7f32aadb213418e5af7e642a9ebea4085b990021029e75ba0901022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740f75b8424f5f58ce4bb5ca851c21c66a4a33d4cd13123b6afd476cd242e5b89da6407bd6fccf33acb3ba9eadc136642a178cdddfd339fa886d37ec0852d6a1e0b0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b21012007404fb69776587a85473bdd3cbdbcf685061d2d64eb6f6720629cc7866105f297f36909fb41fc077400f0d9622bfc787a48b1b92411f964006b4453773d8e2b420c2200012101200741012fb66c12ac06582d21b10fa375530999b30084c3d5c7226e1559a4c5cab02096584acdf555109b67683a727660cb798efe8d7e13bf83c1e41a510f85566bfa41"
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
  "compiled_notarized_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f50508000021022022111f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0877697468647261772007404d210280000000000000000000000000000000000000000000000000000004850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000041f03800292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007084d2101810000000005028500002cf61a24a2290000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000404018000aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001800000000000000000000000000000000000000000000000000000040d0181010000000e0182020000000f0182020000000f0182030000001f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007404d210280000000000000000000000000000000000000000000000000000004850000f4448291634500000000000000000000000000000000000000000000000007000f0182040000000301810100000002022087010100000000000000018000000000000000000000000000000000000000000000000000000410001f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007054d21018300202000202206000121012007410065ad0bb4ebe66451b760156fe7c1780beae38f99dc81d690fe951f6f143c596533543908fb6b2664f4c551ef0134fbd6d91feb4374cebb0aaf746de17189e1be0001210120074100cacb6ca003adfbe6a2e7210613ceb5bcff1377f295eb448f0bf44e71ff30e7d66c050f2fa053bea17657dcaba31ae53454f0f52ba49a8ffc544c48a83fc6f0670001210120074101461c46a081894cf99f09f87329333200420031214df82893dbc8fe2847cdc4914177caf78577dca8256beffac8b424532c52576c1df33af962b11304862e369601022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740fe98f54c1094e5889c15ee4d5a941b3e3159cdd40fcc8b73764cabb37c3401dbbb7d466a34a38d7f32aadb213418e5af7e642a9ebea4085b990021029e75ba0901022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740f75b8424f5f58ce4bb5ca851c21c66a4a33d4cd13123b6afd476cd242e5b89da6407bd6fccf33acb3ba9eadc136642a178cdddfd339fa886d37ec0852d6a1e0b0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b21012007404fb69776587a85473bdd3cbdbcf685061d2d64eb6f6720629cc7866105f297f36909fb41fc077400f0d9622bfc787a48b1b92411f964006b4453773d8e2b420c2200012101200741012fb66c12ac06582d21b10fa375530999b30084c3d5c7226e1559a4c5cab02096584acdf555109b67683a727660cb798efe8d7e13bf83c1e41a510f85566bfa41"
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
                "value": "withdraw"
              },
              "arguments": [
                {
                  "type": "ResourceAddress",
                  "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
                },
                {
                  "type": "Decimal",
                  "value": "5"
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
                  "type": "ResourceAddress",
                  "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
                },
                {
                  "type": "Decimal",
                  "value": "5"
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
        "signature": "0065ad0bb4ebe66451b760156fe7c1780beae38f99dc81d690fe951f6f143c596533543908fb6b2664f4c551ef0134fbd6d91feb4374cebb0aaf746de17189e1be"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "00cacb6ca003adfbe6a2e7210613ceb5bcff1377f295eb448f0bf44e71ff30e7d66c050f2fa053bea17657dcaba31ae53454f0f52ba49a8ffc544c48a83fc6f067"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "01461c46a081894cf99f09f87329333200420031214df82893dbc8fe2847cdc4914177caf78577dca8256beffac8b424532c52576c1df33af962b11304862e3696"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
        "signature": "fe98f54c1094e5889c15ee4d5a941b3e3159cdd40fcc8b73764cabb37c3401dbbb7d466a34a38d7f32aadb213418e5af7e642a9ebea4085b990021029e75ba09"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
        "signature": "f75b8424f5f58ce4bb5ca851c21c66a4a33d4cd13123b6afd476cd242e5b89da6407bd6fccf33acb3ba9eadc136642a178cdddfd339fa886d37ec0852d6a1e0b"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
        "signature": "4fb69776587a85473bdd3cbdbcf685061d2d64eb6f6720629cc7866105f297f36909fb41fc077400f0d9622bfc787a48b1b92411f964006b4453773d8e2b420c"
      }
    ]
  },
  "notary_signature": {
    "curve": "EcdsaSecp256k1",
    "signature": "012fb66c12ac06582d21b10fa375530999b30084c3d5c7226e1559a4c5cab02096584acdf555109b67683a727660cb798efe8d7e13bf83c1e41a510f85566bfa41"
  }
}
```
</details>

## Decompile Unknown Transaction Intent

| Function Name     | `decompile_unknown_transaction_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_decompileUnknownTransactionIntent` |
| Functionality     | There are certain cases where we might have some blob which we suspect is a transaction intent but we have no way of verifying whether that is true or not. Looking at the type id byte of the blob does not help either as it's a generic Struct type which is not too telling. For this specific use case, this library provides this function which attempts to decompile a transaction intent of an unknown type. |
| Request Type      | `DecompileUnknownTransactionIntentRequest` |
| Response Type     | `DecompileUnknownTransactionIntentResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "instructions_output_kind": "Parsed",
  "compiled_unknown_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f50508000021022022111f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0877697468647261772007404d210280000000000000000000000000000000000000000000000000000004850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000041f03800292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007084d2101810000000005028500002cf61a24a2290000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000404018000aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001800000000000000000000000000000000000000000000000000000040d0181010000000e0182020000000f0182020000000f0182030000001f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007404d210280000000000000000000000000000000000000000000000000000004850000f4448291634500000000000000000000000000000000000000000000000007000f0182040000000301810100000002022087010100000000000000018000000000000000000000000000000000000000000000000000000410001f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007054d21018300202000202206000121012007410065ad0bb4ebe66451b760156fe7c1780beae38f99dc81d690fe951f6f143c596533543908fb6b2664f4c551ef0134fbd6d91feb4374cebb0aaf746de17189e1be0001210120074100cacb6ca003adfbe6a2e7210613ceb5bcff1377f295eb448f0bf44e71ff30e7d66c050f2fa053bea17657dcaba31ae53454f0f52ba49a8ffc544c48a83fc6f0670001210120074101461c46a081894cf99f09f87329333200420031214df82893dbc8fe2847cdc4914177caf78577dca8256beffac8b424532c52576c1df33af962b11304862e369601022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740fe98f54c1094e5889c15ee4d5a941b3e3159cdd40fcc8b73764cabb37c3401dbbb7d466a34a38d7f32aadb213418e5af7e642a9ebea4085b990021029e75ba0901022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740f75b8424f5f58ce4bb5ca851c21c66a4a33d4cd13123b6afd476cd242e5b89da6407bd6fccf33acb3ba9eadc136642a178cdddfd339fa886d37ec0852d6a1e0b0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b21012007404fb69776587a85473bdd3cbdbcf685061d2d64eb6f6720629cc7866105f297f36909fb41fc077400f0d9622bfc787a48b1b92411f964006b4453773d8e2b420c2200012101200741012fb66c12ac06582d21b10fa375530999b30084c3d5c7226e1559a4c5cab02096584acdf555109b67683a727660cb798efe8d7e13bf83c1e41a510f85566bfa41"
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
                "instruction": "CALL_METHOD",
                "component_address": {
                  "type": "ComponentAddress",
                  "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
                },
                "method_name": {
                  "type": "String",
                  "value": "withdraw"
                },
                "arguments": [
                  {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
                  },
                  {
                    "type": "Decimal",
                    "value": "5"
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
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
                  },
                  {
                    "type": "Decimal",
                    "value": "5"
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
          "signature": "0065ad0bb4ebe66451b760156fe7c1780beae38f99dc81d690fe951f6f143c596533543908fb6b2664f4c551ef0134fbd6d91feb4374cebb0aaf746de17189e1be"
        },
        {
          "curve": "EcdsaSecp256k1",
          "signature": "00cacb6ca003adfbe6a2e7210613ceb5bcff1377f295eb448f0bf44e71ff30e7d66c050f2fa053bea17657dcaba31ae53454f0f52ba49a8ffc544c48a83fc6f067"
        },
        {
          "curve": "EcdsaSecp256k1",
          "signature": "01461c46a081894cf99f09f87329333200420031214df82893dbc8fe2847cdc4914177caf78577dca8256beffac8b424532c52576c1df33af962b11304862e3696"
        },
        {
          "curve": "EddsaEd25519",
          "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
          "signature": "fe98f54c1094e5889c15ee4d5a941b3e3159cdd40fcc8b73764cabb37c3401dbbb7d466a34a38d7f32aadb213418e5af7e642a9ebea4085b990021029e75ba09"
        },
        {
          "curve": "EddsaEd25519",
          "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
          "signature": "f75b8424f5f58ce4bb5ca851c21c66a4a33d4cd13123b6afd476cd242e5b89da6407bd6fccf33acb3ba9eadc136642a178cdddfd339fa886d37ec0852d6a1e0b"
        },
        {
          "curve": "EddsaEd25519",
          "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
          "signature": "4fb69776587a85473bdd3cbdbcf685061d2d64eb6f6720629cc7866105f297f36909fb41fc077400f0d9622bfc787a48b1b92411f964006b4453773d8e2b420c"
        }
      ]
    },
    "notary_signature": {
      "curve": "EcdsaSecp256k1",
      "signature": "012fb66c12ac06582d21b10fa375530999b30084c3d5c7226e1559a4c5cab02096584acdf555109b67683a727660cb798efe8d7e13bf83c1e41a510f85566bfa41"
    }
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
| Functionality     | This function takes in a ScryptoSborValue and encodes it in SBOR. |
| Request Type      | `SborEncodeRequest` |
| Response Type     | `SborEncodeResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "type": "ScryptoSbor",
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

<details>
    <summary>Response Example</summary>
    
```json
{
  "encoded_value": "5c2104a00000e8890423c78a000000000000000000000000000000000000000000000000b000000000000000000a36257aef45394e46ef8b8a90c37f1c2716f3000000000000000000000000000000000000000000000000000000000000000000000000000c0c48656c6c6f20576f726c64212104a00000e8890423c78a000000000000000000000000000000000000000000000000b000000000000000000a36257aef45394e46ef8b8a90c37f1c2716f3000000000000000000000000000000000000000000000000000000000000000000000000000c0c48656c6c6f20576f726c64212104a00000e8890423c78a000000000000000000000000000000000000000000000000b000000000000000000a36257aef45394e46ef8b8a90c37f1c2716f3000000000000000000000000000000000000000000000000000000000000000000000000000c0c48656c6c6f20576f726c64212104a00000e8890423c78a000000000000000000000000000000000000000000000000b000000000000000000a36257aef45394e46ef8b8a90c37f1c2716f3000000000000000000000000000000000000000000000000000000000000000000000000000c0c48656c6c6f20576f726c642120a0030000d01309468e15010000000000000000000000000000000000000000000000000010632d5ec76b05000000000000000000000000000000000000000000000000005f13195ed66c0a0000000000000000000000000000000000000000000000"
}
```
</details>

## Sbor Decode

| Function Name     | `sbor_decode` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_sborDecode` |
| Functionality     | This function takes in a hex string and attempts to decode it into a ScryptoSborValue. |
| Request Type      | `SborDecodeRequest` |
| Response Type     | `SborDecodeResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "encoded_value": "4d210a8000000000000000000000000000000000000000000000000000000080010101010101010101010101010101010101010101010101010101800202020202020202020202020202020202020202020202020202028104000000820500000083018406060606060606060606060606060606060606060606060606060606060606068507070707070707070707070707070707070707070707070707070707070707078608080808080808080808080808080808080808080808080808080808080808080808080808080808080808080808080808080808080808080808080808080808870003616263",
  "network_id": "242"
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "type": "ManifestSbor",
  "value": {
    "type": "Tuple",
    "elements": [
      {
        "type": "Address",
        "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
      },
      {
        "type": "Address",
        "address": "package_sim1qyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqs5ttlus"
      },
      {
        "type": "Address",
        "address": "component_sim1qgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqqn89f8"
      },
      {
        "type": "Bucket",
        "identifier": 4
      },
      {
        "type": "Proof",
        "identifier": 5
      },
      {
        "type": "Expression",
        "value": "ENTIRE_AUTH_ZONE"
      },
      {
        "type": "Blob",
        "hash": "0606060606060606060606060606060606060606060606060606060606060606"
      },
      {
        "type": "Decimal",
        "value": "3178606371220444580254889784552217078325058402586211561867.463090413301597959"
      },
      {
        "type": "PreciseDecimal",
        "value": "42063711152761088939840078425743830988170559437152606675211173156774161662975833652711762.5040530303613804921041144660418941298284296362978711643890386952"
      },
      {
        "type": "NonFungibleLocalId",
        "value": {
          "type": "String",
          "value": "abc"
        }
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
  "virtual_account_address": "account_sim1qakfdmv0q2cwz9cjxk5t8u0zx6pdydd9p2jv22nuwdhqe6yywy"
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
  "virtual_identity_address": "identity_sim1pfkfdmv0q2cwz9cjxk5t8u0zx6pdydd9p2jv22nuwdhqy9pgkj"
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
  "non_fungible_global_id": {
    "resource_address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqshxgp7h",
    "non_fungible_local_id": {
      "type": "Bytes",
      "value": "6c96ed8f02b0e1171235a8b3f1e23682d235a50aa4c52a7c736e"
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
  "compiled_notarized_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f50508000021022022111f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0877697468647261772007404d210280000000000000000000000000000000000000000000000000000004850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000041f03800292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007084d2101810000000005028500002cf61a24a2290000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000404018000aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001800000000000000000000000000000000000000000000000000000040d0181010000000e0182020000000f0182020000000f0182030000001f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007404d210280000000000000000000000000000000000000000000000000000004850000f4448291634500000000000000000000000000000000000000000000000007000f0182040000000301810100000002022087010100000000000000018000000000000000000000000000000000000000000000000000000410001f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007054d21018300202000202206000121012007410065ad0bb4ebe66451b760156fe7c1780beae38f99dc81d690fe951f6f143c596533543908fb6b2664f4c551ef0134fbd6d91feb4374cebb0aaf746de17189e1be0001210120074100cacb6ca003adfbe6a2e7210613ceb5bcff1377f295eb448f0bf44e71ff30e7d66c050f2fa053bea17657dcaba31ae53454f0f52ba49a8ffc544c48a83fc6f0670001210120074101461c46a081894cf99f09f87329333200420031214df82893dbc8fe2847cdc4914177caf78577dca8256beffac8b424532c52576c1df33af962b11304862e369601022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740fe98f54c1094e5889c15ee4d5a941b3e3159cdd40fcc8b73764cabb37c3401dbbb7d466a34a38d7f32aadb213418e5af7e642a9ebea4085b990021029e75ba0901022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740f75b8424f5f58ce4bb5ca851c21c66a4a33d4cd13123b6afd476cd242e5b89da6407bd6fccf33acb3ba9eadc136642a178cdddfd339fa886d37ec0852d6a1e0b0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b21012007404fb69776587a85473bdd3cbdbcf685061d2d64eb6f6720629cc7866105f297f36909fb41fc077400f0d9622bfc787a48b1b92411f964006b4453773d8e2b420c22000121012007410065ad0bb4ebe66451b760156fe7c1780beae38f99dc81d690fe951f6f143c596533543908fb6b2664f4c551ef0134fbd6d91feb4374cebb0aaf746de17189e1be",
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
    "address": "component_rdx1qtkryz5scup945usk39qjc2yjh6l5zsyuh8t7v5pk0tsrdcazt"
  },
  "faucet_package_address": {
    "type": "PackageAddress",
    "address": "package_rdx1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpqqzrhqe8"
  },
  "account_package_address": {
    "type": "PackageAddress",
    "address": "package_rdx1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzstngjrq"
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

## Hash

| Function Name     | `hash` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_hash` |
| Functionality     | Hashes some payload through the hashing algorithm used in Scrypto and the Radix Engine. |
| Request Type      | `HashRequest` |
| Response Type     | `HashResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "payload": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f50508000021022022111f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0877697468647261772007404d210280000000000000000000000000000000000000000000000000000004850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000041f03800292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007084d2101810000000005028500002cf61a24a2290000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000404018000aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510001800000000000000000000000000000000000000000000000000000040d0181010000000e0182020000000f0182020000000f0182030000001f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007404d210280000000000000000000000000000000000000000000000000000004850000f4448291634500000000000000000000000000000000000000000000000007000f0182040000000301810100000002022087010100000000000000018000000000000000000000000000000000000000000000000000000410001f038003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007054d21018300202000202206000121012007410065ad0bb4ebe66451b760156fe7c1780beae38f99dc81d690fe951f6f143c596533543908fb6b2664f4c551ef0134fbd6d91feb4374cebb0aaf746de17189e1be0001210120074100cacb6ca003adfbe6a2e7210613ceb5bcff1377f295eb448f0bf44e71ff30e7d66c050f2fa053bea17657dcaba31ae53454f0f52ba49a8ffc544c48a83fc6f0670001210120074101461c46a081894cf99f09f87329333200420031214df82893dbc8fe2847cdc4914177caf78577dca8256beffac8b424532c52576c1df33af962b11304862e369601022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740fe98f54c1094e5889c15ee4d5a941b3e3159cdd40fcc8b73764cabb37c3401dbbb7d466a34a38d7f32aadb213418e5af7e642a9ebea4085b990021029e75ba0901022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740f75b8424f5f58ce4bb5ca851c21c66a4a33d4cd13123b6afd476cd242e5b89da6407bd6fccf33acb3ba9eadc136642a178cdddfd339fa886d37ec0852d6a1e0b0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b21012007404fb69776587a85473bdd3cbdbcf685061d2d64eb6f6720629cc7866105f297f36909fb41fc077400f0d9622bfc787a48b1b92411f964006b4453773d8e2b420c22000121012007410065ad0bb4ebe66451b760156fe7c1780beae38f99dc81d690fe951f6f143c596533543908fb6b2664f4c551ef0134fbd6d91feb4374cebb0aaf746de17189e1be"
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "value": "73abfbd0ddb0f08cfc1df1030798d359fc6785386910e4fc9e977c4ebcf3c972"
}
```
</details>
