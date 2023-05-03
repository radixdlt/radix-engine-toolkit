# Requests Examples

This document contains examples and descriptions of the different requests and responses which the Radix Engine Toolkit may provide. As long as all of the CI test pass, then you may treat this document as the canonical truth for the format of the different requests and as valid examples of the payload and responses of these requests.


## Information

| Function Name     | `information` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_information` |
| Functionality     | The function provides information information on the currently in-use radix engine toolkit such as the version of the radix engine toolkit. In most cases, this is the first function written when integrating new clients; so, this function is often times seen as the "Hello World" example of the radix engine toolkit.   |
| Required Features | default   |
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
| Functionality     | Clients have a need to be able to read, parse, understand, and interrogate transaction manifests to get more information on what a transactions might be doing. Transaction manifests have so far existed in one format: as strings. While the string format is very human readable, it is not easily readable by machines as a lexer and parser are needed to make sense of them; thus, it is for clients to programmatically make sense of transactions. As such, there is a need for another transaction manifest format (to supplement, NOT replace) which machines can easily make sense of without the need to implement a lexer and parser.</br></br>Therefore, this library introduces a Parsed format for transaction manifests which clients can use when wanting to read and interrogate their transaction manifests in code. The transaction manifest Parsed format has a 1:1 mapping to the string format of transaction manifests, meaning that anything which can be done in the string format of transaction manifests, can be done in the Parsed format as well.</br></br>This function allows the client the convert their manifest between the two supported manifest types: string and Parsed.   |
| Required Features | default   |
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
            "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
          },
          "method_name": {
            "type": "String",
            "value": "withdraw"
          },
          "arguments": [
            {
              "type": "Address",
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
            "address": "component_sim1qd8djmepmq7hxqaakt9rl3hkce532px42s8eh4qmqlks9f87dn"
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
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
            "address": "resource_sim1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsgvqtwe"
          }
        },
        {
          "instruction": "TAKE_FROM_WORKTOP",
          "resource_address": {
            "type": "Address",
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
          "instruction": "CALL_METHOD",
          "component_address": {
            "type": "Address",
            "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
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
          "type": "Address",
          "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
        },
        "method_name": {
          "type": "String",
          "value": "withdraw"
        },
        "arguments": [
          {
            "type": "Address",
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
          "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
          "address": "component_sim1qd8djmepmq7hxqaakt9rl3hkce532px42s8eh4qmqlks9f87dn"
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
          "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
          "address": "resource_sim1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsgvqtwe"
        }
      },
      {
        "instruction": "TAKE_FROM_WORKTOP",
        "resource_address": {
          "type": "Address",
          "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
          "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
        "instruction": "CALL_METHOD",
        "component_address": {
          "type": "Address",
          "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
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
| Functionality     | Analyzes the manifest returning back all of the addresses involved in the manifest alongside some useful information on whether the accounts were withdrawn from, deposited into, or just used in the manifest in general.   |
| Required Features | default   |
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
      "value": "CALL_METHOD\n    Address(\"account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn\")\n    \"withdraw\"\n    Address(\"resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k\")\n    Decimal(\"5\");\nTAKE_FROM_WORKTOP_BY_AMOUNT\n    Decimal(\"2\")\n    Address(\"resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k\")\n    Bucket(\"bucket1\");\nCALL_METHOD\n    Address(\"component_sim1qd8djmepmq7hxqaakt9rl3hkce532px42s8eh4qmqlks9f87dn\")\n    \"buy_gumball\"\n    Bucket(\"bucket1\");\nASSERT_WORKTOP_CONTAINS_BY_AMOUNT\n    Decimal(\"3\")\n    Address(\"resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k\");\nASSERT_WORKTOP_CONTAINS\n    Address(\"resource_sim1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsgvqtwe\");\nTAKE_FROM_WORKTOP\n    Address(\"resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k\")\n    Bucket(\"bucket2\");\nRETURN_TO_WORKTOP\n    Bucket(\"bucket2\");\nTAKE_FROM_WORKTOP_BY_IDS\n    Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"))\n    Address(\"resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k\")\n    Bucket(\"bucket3\");\nCALL_METHOD\n    Address(\"account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn\")\n    \"deposit_batch\"\n    Expression(\"ENTIRE_WORKTOP\");\n"
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
      "address": "component_sim1qd8djmepmq7hxqaakt9rl3hkce532px42s8eh4qmqlks9f87dn"
    },
    {
      "type": "ComponentAddress",
      "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
    }
  ],
  "resource_addresses": [
    {
      "type": "ResourceAddress",
      "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
    },
    {
      "type": "ResourceAddress",
      "address": "resource_sim1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsgvqtwe"
    }
  ],
  "account_addresses": [
    {
      "type": "ComponentAddress",
      "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
    }
  ],
  "accounts_requiring_auth": [
    {
      "type": "ComponentAddress",
      "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
    }
  ],
  "accounts_withdrawn_from": [
    {
      "type": "ComponentAddress",
      "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
    }
  ],
  "accounts_deposited_into": [
    {
      "type": "ComponentAddress",
      "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
    }
  ]
}
```
</details>

## Compile Transaction Intent

| Function Name     | `compile_transaction_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_compileTransactionIntent` |
| Functionality     | Takes a transaction intent and compiles it by SBOR encoding it and returning it back to the caller. This is mainly useful when creating a transaction.   |
| Required Features | default   |
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
            "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
          },
          "method_name": {
            "type": "String",
            "value": "withdraw"
          },
          "arguments": [
            {
              "type": "Address",
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
            "address": "component_sim1qd8djmepmq7hxqaakt9rl3hkce532px42s8eh4qmqlks9f87dn"
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
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
            "address": "resource_sim1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsgvqtwe"
          }
        },
        {
          "instruction": "TAKE_FROM_WORKTOP",
          "resource_address": {
            "type": "Address",
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
          "instruction": "CALL_METHOD",
          "component_address": {
            "type": "Address",
            "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
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
  "compiled_intent": "4d21022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f5050800002102202209210380048944f6ce22315a26c4b90c6e7e09a093dcbc98c89e67d32b2dc10c087769746864726177210280010000000000000000000000000000000000000000000000000000850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b00000000000000000000000000000000000000000000000080010000000000000000000000000000000000000000000000000000210380034ed96f21d83d7303bdb2ca3fc6f6c6691504d5540f9bd41b07ed0c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a229000000000000000000000000000000000000000000000000800100000000000000000000000000000000000000000000000000000401800289ba4758731898e0850fbde5d412c080e4f8b7ea03174cb180d900018001000000000000000000000000000000000000000000000000000003018101000000020220870101000000000000000180010000000000000000000000000000000000000000000000000000210380048944f6ce22315a26c4b90c6e7e09a093dcbc98c89e67d32b2dc10c0d6465706f7369745f626174636821018300202000"
}
```
</details>

## Decompile Transaction Intent

| Function Name     | `decompile_transaction_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_decompileTransactionIntent` |
| Functionality     | This function does the opposite of the compile_transaction_intent function. It takes in a compiled transaction intent and decompiles it into its human-readable / machine-readable format.   |
| Required Features | default   |
| Request Type      | `DecompileTransactionIntentRequest` |
| Response Type     | `DecompileTransactionIntentResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "instructions_output_kind": "Parsed",
  "compiled_intent": "4d21022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f5050800002102202209210380048944f6ce22315a26c4b90c6e7e09a093dcbc98c89e67d32b2dc10c087769746864726177210280010000000000000000000000000000000000000000000000000000850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b00000000000000000000000000000000000000000000000080010000000000000000000000000000000000000000000000000000210380034ed96f21d83d7303bdb2ca3fc6f6c6691504d5540f9bd41b07ed0c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a229000000000000000000000000000000000000000000000000800100000000000000000000000000000000000000000000000000000401800289ba4758731898e0850fbde5d412c080e4f8b7ea03174cb180d900018001000000000000000000000000000000000000000000000000000003018101000000020220870101000000000000000180010000000000000000000000000000000000000000000000000000210380048944f6ce22315a26c4b90c6e7e09a093dcbc98c89e67d32b2dc10c0d6465706f7369745f626174636821018300202000"
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
            "type": "Address",
            "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
          },
          "method_name": {
            "type": "String",
            "value": "withdraw"
          },
          "arguments": [
            {
              "type": "Address",
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
            "address": "component_sim1qd8djmepmq7hxqaakt9rl3hkce532px42s8eh4qmqlks9f87dn"
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
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
            "address": "resource_sim1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsgvqtwe"
          }
        },
        {
          "instruction": "TAKE_FROM_WORKTOP",
          "resource_address": {
            "type": "Address",
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
          "instruction": "CALL_METHOD",
          "component_address": {
            "type": "Address",
            "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
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
| Functionality     | This function takes in a raw transaction intent as well as its signatures and compiles it. This is useful when a notary wishes to notarize a signed transaction intent.   |
| Required Features | default   |
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
              "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
            },
            "method_name": {
              "type": "String",
              "value": "withdraw"
            },
            "arguments": [
              {
                "type": "Address",
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
              "address": "component_sim1qd8djmepmq7hxqaakt9rl3hkce532px42s8eh4qmqlks9f87dn"
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
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
              "address": "resource_sim1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsgvqtwe"
            }
          },
          {
            "instruction": "TAKE_FROM_WORKTOP",
            "resource_address": {
              "type": "Address",
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
            "instruction": "CALL_METHOD",
            "component_address": {
              "type": "Address",
              "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
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
      "signature": "001dc81ce1fd9b1de438972231e81db2cab2ec01f205019c7e947b2ef049c18763283394f18f7efd1ede7122a5b0ae68bcab671c6f28a83061c13c1b7413728a74"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "008b86278af6e6336c8e7a3d635f0fec9c467588397c4df4818f32e897238f2a3c1edb19118c9d9a09f9c2f98506486e96db89acc987a5b3dee4861e01ca8761d0"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "0075a6696b28b00b4295ffdfeaf852e52736f8fbd2314e1ea087ce0215b799cba14a98d918be28cf71ed51eaa58d7b88e1a15ef50297e04ec97dbad77d7702784e"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
      "signature": "2a310d3ed1eacc1ccf3b7d59a91a6474415c647f55af42e2e912dc850b79f7418108a1bed1ee34103530372b1899853fff078c32b5e590bb718f74a1df32400a"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
      "signature": "7eb1c121969dfba6cf43695a12f889e15081407ea455396cb02acdc101b277618531cb9a231ec48798c02c0bb73f9350f0e58bda6b6c8b6b5d6416ecd1f21801"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
      "signature": "c2277b73ff69e7e63b1c5ec98b23f71d7e419df1c69d5e58d4a73a9bdb18192b72f64410c7e19e7b88fd339112c8171928ae3669f39cad38050eb48a8ecb3c0d"
    }
  ]
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "compiled_intent": "4d210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f5050800002102202209210380048944f6ce22315a26c4b90c6e7e09a093dcbc98c89e67d32b2dc10c087769746864726177210280010000000000000000000000000000000000000000000000000000850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b00000000000000000000000000000000000000000000000080010000000000000000000000000000000000000000000000000000210380034ed96f21d83d7303bdb2ca3fc6f6c6691504d5540f9bd41b07ed0c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a229000000000000000000000000000000000000000000000000800100000000000000000000000000000000000000000000000000000401800289ba4758731898e0850fbde5d412c080e4f8b7ea03174cb180d900018001000000000000000000000000000000000000000000000000000003018101000000020220870101000000000000000180010000000000000000000000000000000000000000000000000000210380048944f6ce22315a26c4b90c6e7e09a093dcbc98c89e67d32b2dc10c0d6465706f7369745f62617463682101830020200020220600012101200741001dc81ce1fd9b1de438972231e81db2cab2ec01f205019c7e947b2ef049c18763283394f18f7efd1ede7122a5b0ae68bcab671c6f28a83061c13c1b7413728a7400012101200741008b86278af6e6336c8e7a3d635f0fec9c467588397c4df4818f32e897238f2a3c1edb19118c9d9a09f9c2f98506486e96db89acc987a5b3dee4861e01ca8761d0000121012007410075a6696b28b00b4295ffdfeaf852e52736f8fbd2314e1ea087ce0215b799cba14a98d918be28cf71ed51eaa58d7b88e1a15ef50297e04ec97dbad77d7702784e01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba2921012007402a310d3ed1eacc1ccf3b7d59a91a6474415c647f55af42e2e912dc850b79f7418108a1bed1ee34103530372b1899853fff078c32b5e590bb718f74a1df32400a01022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe267421012007407eb1c121969dfba6cf43695a12f889e15081407ea455396cb02acdc101b277618531cb9a231ec48798c02c0bb73f9350f0e58bda6b6c8b6b5d6416ecd1f218010102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740c2277b73ff69e7e63b1c5ec98b23f71d7e419df1c69d5e58d4a73a9bdb18192b72f64410c7e19e7b88fd339112c8171928ae3669f39cad38050eb48a8ecb3c0d"
}
```
</details>

## Decompile Signed Transaction Intent

| Function Name     | `decompile_signed_transaction_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_decompileSignedTransactionIntent` |
| Functionality     | This function does the opposite of the compile_signed_transaction_intent function. This function takes in a compiled signed transaction intent and decompiles it into its transaction intent and signatures.   |
| Required Features | default   |
| Request Type      | `DecompileSignedTransactionIntentRequest` |
| Response Type     | `DecompileSignedTransactionIntentResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "instructions_output_kind": "Parsed",
  "compiled_signed_intent": "4d210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f5050800002102202209210380048944f6ce22315a26c4b90c6e7e09a093dcbc98c89e67d32b2dc10c087769746864726177210280010000000000000000000000000000000000000000000000000000850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b00000000000000000000000000000000000000000000000080010000000000000000000000000000000000000000000000000000210380034ed96f21d83d7303bdb2ca3fc6f6c6691504d5540f9bd41b07ed0c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a229000000000000000000000000000000000000000000000000800100000000000000000000000000000000000000000000000000000401800289ba4758731898e0850fbde5d412c080e4f8b7ea03174cb180d900018001000000000000000000000000000000000000000000000000000003018101000000020220870101000000000000000180010000000000000000000000000000000000000000000000000000210380048944f6ce22315a26c4b90c6e7e09a093dcbc98c89e67d32b2dc10c0d6465706f7369745f62617463682101830020200020220600012101200741001dc81ce1fd9b1de438972231e81db2cab2ec01f205019c7e947b2ef049c18763283394f18f7efd1ede7122a5b0ae68bcab671c6f28a83061c13c1b7413728a7400012101200741008b86278af6e6336c8e7a3d635f0fec9c467588397c4df4818f32e897238f2a3c1edb19118c9d9a09f9c2f98506486e96db89acc987a5b3dee4861e01ca8761d0000121012007410075a6696b28b00b4295ffdfeaf852e52736f8fbd2314e1ea087ce0215b799cba14a98d918be28cf71ed51eaa58d7b88e1a15ef50297e04ec97dbad77d7702784e01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba2921012007402a310d3ed1eacc1ccf3b7d59a91a6474415c647f55af42e2e912dc850b79f7418108a1bed1ee34103530372b1899853fff078c32b5e590bb718f74a1df32400a01022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe267421012007407eb1c121969dfba6cf43695a12f889e15081407ea455396cb02acdc101b277618531cb9a231ec48798c02c0bb73f9350f0e58bda6b6c8b6b5d6416ecd1f218010102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740c2277b73ff69e7e63b1c5ec98b23f71d7e419df1c69d5e58d4a73a9bdb18192b72f64410c7e19e7b88fd339112c8171928ae3669f39cad38050eb48a8ecb3c0d"
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
              "type": "Address",
              "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
            },
            "method_name": {
              "type": "String",
              "value": "withdraw"
            },
            "arguments": [
              {
                "type": "Address",
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
              "address": "component_sim1qd8djmepmq7hxqaakt9rl3hkce532px42s8eh4qmqlks9f87dn"
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
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
              "address": "resource_sim1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsgvqtwe"
            }
          },
          {
            "instruction": "TAKE_FROM_WORKTOP",
            "resource_address": {
              "type": "Address",
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
            "instruction": "CALL_METHOD",
            "component_address": {
              "type": "Address",
              "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
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
      "signature": "001dc81ce1fd9b1de438972231e81db2cab2ec01f205019c7e947b2ef049c18763283394f18f7efd1ede7122a5b0ae68bcab671c6f28a83061c13c1b7413728a74"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "008b86278af6e6336c8e7a3d635f0fec9c467588397c4df4818f32e897238f2a3c1edb19118c9d9a09f9c2f98506486e96db89acc987a5b3dee4861e01ca8761d0"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "0075a6696b28b00b4295ffdfeaf852e52736f8fbd2314e1ea087ce0215b799cba14a98d918be28cf71ed51eaa58d7b88e1a15ef50297e04ec97dbad77d7702784e"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
      "signature": "2a310d3ed1eacc1ccf3b7d59a91a6474415c647f55af42e2e912dc850b79f7418108a1bed1ee34103530372b1899853fff078c32b5e590bb718f74a1df32400a"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
      "signature": "7eb1c121969dfba6cf43695a12f889e15081407ea455396cb02acdc101b277618531cb9a231ec48798c02c0bb73f9350f0e58bda6b6c8b6b5d6416ecd1f21801"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
      "signature": "c2277b73ff69e7e63b1c5ec98b23f71d7e419df1c69d5e58d4a73a9bdb18192b72f64410c7e19e7b88fd339112c8171928ae3669f39cad38050eb48a8ecb3c0d"
    }
  ]
}
```
</details>

## Compile Notarized Transaction

| Function Name     | `compile_notarized_transaction` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_compileNotarizedTransaction` |
| Functionality     | This function takes in a raw signed transaction intent as well as the notary signature and compiles it. This is useful when we wish to submit a transaction to the Gateway API   |
| Required Features | default   |
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
                "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
              },
              "method_name": {
                "type": "String",
                "value": "withdraw"
              },
              "arguments": [
                {
                  "type": "Address",
                  "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
                "address": "component_sim1qd8djmepmq7hxqaakt9rl3hkce532px42s8eh4qmqlks9f87dn"
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
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
                "address": "resource_sim1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsgvqtwe"
              }
            },
            {
              "instruction": "TAKE_FROM_WORKTOP",
              "resource_address": {
                "type": "Address",
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
              "instruction": "CALL_METHOD",
              "component_address": {
                "type": "Address",
                "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
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
        "signature": "001dc81ce1fd9b1de438972231e81db2cab2ec01f205019c7e947b2ef049c18763283394f18f7efd1ede7122a5b0ae68bcab671c6f28a83061c13c1b7413728a74"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "008b86278af6e6336c8e7a3d635f0fec9c467588397c4df4818f32e897238f2a3c1edb19118c9d9a09f9c2f98506486e96db89acc987a5b3dee4861e01ca8761d0"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "0075a6696b28b00b4295ffdfeaf852e52736f8fbd2314e1ea087ce0215b799cba14a98d918be28cf71ed51eaa58d7b88e1a15ef50297e04ec97dbad77d7702784e"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
        "signature": "2a310d3ed1eacc1ccf3b7d59a91a6474415c647f55af42e2e912dc850b79f7418108a1bed1ee34103530372b1899853fff078c32b5e590bb718f74a1df32400a"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
        "signature": "7eb1c121969dfba6cf43695a12f889e15081407ea455396cb02acdc101b277618531cb9a231ec48798c02c0bb73f9350f0e58bda6b6c8b6b5d6416ecd1f21801"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
        "signature": "c2277b73ff69e7e63b1c5ec98b23f71d7e419df1c69d5e58d4a73a9bdb18192b72f64410c7e19e7b88fd339112c8171928ae3669f39cad38050eb48a8ecb3c0d"
      }
    ]
  },
  "notary_signature": {
    "curve": "EcdsaSecp256k1",
    "signature": "005f9290847274a47ce592718db3b1d7ace56db9fbcbc9139cd9e680715f7a316a535cccd292b1176e8d25a2b5dfc6907fc0527c8f5bd36db750a02972945bc6d7"
  }
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "compiled_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f5050800002102202209210380048944f6ce22315a26c4b90c6e7e09a093dcbc98c89e67d32b2dc10c087769746864726177210280010000000000000000000000000000000000000000000000000000850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b00000000000000000000000000000000000000000000000080010000000000000000000000000000000000000000000000000000210380034ed96f21d83d7303bdb2ca3fc6f6c6691504d5540f9bd41b07ed0c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a229000000000000000000000000000000000000000000000000800100000000000000000000000000000000000000000000000000000401800289ba4758731898e0850fbde5d412c080e4f8b7ea03174cb180d900018001000000000000000000000000000000000000000000000000000003018101000000020220870101000000000000000180010000000000000000000000000000000000000000000000000000210380048944f6ce22315a26c4b90c6e7e09a093dcbc98c89e67d32b2dc10c0d6465706f7369745f62617463682101830020200020220600012101200741001dc81ce1fd9b1de438972231e81db2cab2ec01f205019c7e947b2ef049c18763283394f18f7efd1ede7122a5b0ae68bcab671c6f28a83061c13c1b7413728a7400012101200741008b86278af6e6336c8e7a3d635f0fec9c467588397c4df4818f32e897238f2a3c1edb19118c9d9a09f9c2f98506486e96db89acc987a5b3dee4861e01ca8761d0000121012007410075a6696b28b00b4295ffdfeaf852e52736f8fbd2314e1ea087ce0215b799cba14a98d918be28cf71ed51eaa58d7b88e1a15ef50297e04ec97dbad77d7702784e01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba2921012007402a310d3ed1eacc1ccf3b7d59a91a6474415c647f55af42e2e912dc850b79f7418108a1bed1ee34103530372b1899853fff078c32b5e590bb718f74a1df32400a01022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe267421012007407eb1c121969dfba6cf43695a12f889e15081407ea455396cb02acdc101b277618531cb9a231ec48798c02c0bb73f9350f0e58bda6b6c8b6b5d6416ecd1f218010102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740c2277b73ff69e7e63b1c5ec98b23f71d7e419df1c69d5e58d4a73a9bdb18192b72f64410c7e19e7b88fd339112c8171928ae3669f39cad38050eb48a8ecb3c0d2200012101200741005f9290847274a47ce592718db3b1d7ace56db9fbcbc9139cd9e680715f7a316a535cccd292b1176e8d25a2b5dfc6907fc0527c8f5bd36db750a02972945bc6d7"
}
```
</details>

## Decompile Notarized Transaction

| Function Name     | `decompile_notarized_transaction` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_decompileNotarizedTransaction` |
| Functionality     | This function does the opposite of the compile_notarized_intent()_intent function. This function takes in a compiled notarized transaction intent and decompiles it into its signed transaction intent and notary signature.   |
| Required Features | default   |
| Request Type      | `DecompileNotarizedTransactionRequest` |
| Response Type     | `DecompileNotarizedTransactionResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "instructions_output_kind": "Parsed",
  "compiled_notarized_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f5050800002102202209210380048944f6ce22315a26c4b90c6e7e09a093dcbc98c89e67d32b2dc10c087769746864726177210280010000000000000000000000000000000000000000000000000000850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b00000000000000000000000000000000000000000000000080010000000000000000000000000000000000000000000000000000210380034ed96f21d83d7303bdb2ca3fc6f6c6691504d5540f9bd41b07ed0c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a229000000000000000000000000000000000000000000000000800100000000000000000000000000000000000000000000000000000401800289ba4758731898e0850fbde5d412c080e4f8b7ea03174cb180d900018001000000000000000000000000000000000000000000000000000003018101000000020220870101000000000000000180010000000000000000000000000000000000000000000000000000210380048944f6ce22315a26c4b90c6e7e09a093dcbc98c89e67d32b2dc10c0d6465706f7369745f62617463682101830020200020220600012101200741001dc81ce1fd9b1de438972231e81db2cab2ec01f205019c7e947b2ef049c18763283394f18f7efd1ede7122a5b0ae68bcab671c6f28a83061c13c1b7413728a7400012101200741008b86278af6e6336c8e7a3d635f0fec9c467588397c4df4818f32e897238f2a3c1edb19118c9d9a09f9c2f98506486e96db89acc987a5b3dee4861e01ca8761d0000121012007410075a6696b28b00b4295ffdfeaf852e52736f8fbd2314e1ea087ce0215b799cba14a98d918be28cf71ed51eaa58d7b88e1a15ef50297e04ec97dbad77d7702784e01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba2921012007402a310d3ed1eacc1ccf3b7d59a91a6474415c647f55af42e2e912dc850b79f7418108a1bed1ee34103530372b1899853fff078c32b5e590bb718f74a1df32400a01022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe267421012007407eb1c121969dfba6cf43695a12f889e15081407ea455396cb02acdc101b277618531cb9a231ec48798c02c0bb73f9350f0e58bda6b6c8b6b5d6416ecd1f218010102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740c2277b73ff69e7e63b1c5ec98b23f71d7e419df1c69d5e58d4a73a9bdb18192b72f64410c7e19e7b88fd339112c8171928ae3669f39cad38050eb48a8ecb3c0d2200012101200741005f9290847274a47ce592718db3b1d7ace56db9fbcbc9139cd9e680715f7a316a535cccd292b1176e8d25a2b5dfc6907fc0527c8f5bd36db750a02972945bc6d7"
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
                "type": "Address",
                "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
              },
              "method_name": {
                "type": "String",
                "value": "withdraw"
              },
              "arguments": [
                {
                  "type": "Address",
                  "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
                "address": "component_sim1qd8djmepmq7hxqaakt9rl3hkce532px42s8eh4qmqlks9f87dn"
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
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
                "address": "resource_sim1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsgvqtwe"
              }
            },
            {
              "instruction": "TAKE_FROM_WORKTOP",
              "resource_address": {
                "type": "Address",
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
              "instruction": "CALL_METHOD",
              "component_address": {
                "type": "Address",
                "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
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
        "signature": "001dc81ce1fd9b1de438972231e81db2cab2ec01f205019c7e947b2ef049c18763283394f18f7efd1ede7122a5b0ae68bcab671c6f28a83061c13c1b7413728a74"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "008b86278af6e6336c8e7a3d635f0fec9c467588397c4df4818f32e897238f2a3c1edb19118c9d9a09f9c2f98506486e96db89acc987a5b3dee4861e01ca8761d0"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "0075a6696b28b00b4295ffdfeaf852e52736f8fbd2314e1ea087ce0215b799cba14a98d918be28cf71ed51eaa58d7b88e1a15ef50297e04ec97dbad77d7702784e"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
        "signature": "2a310d3ed1eacc1ccf3b7d59a91a6474415c647f55af42e2e912dc850b79f7418108a1bed1ee34103530372b1899853fff078c32b5e590bb718f74a1df32400a"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
        "signature": "7eb1c121969dfba6cf43695a12f889e15081407ea455396cb02acdc101b277618531cb9a231ec48798c02c0bb73f9350f0e58bda6b6c8b6b5d6416ecd1f21801"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
        "signature": "c2277b73ff69e7e63b1c5ec98b23f71d7e419df1c69d5e58d4a73a9bdb18192b72f64410c7e19e7b88fd339112c8171928ae3669f39cad38050eb48a8ecb3c0d"
      }
    ]
  },
  "notary_signature": {
    "curve": "EcdsaSecp256k1",
    "signature": "005f9290847274a47ce592718db3b1d7ace56db9fbcbc9139cd9e680715f7a316a535cccd292b1176e8d25a2b5dfc6907fc0527c8f5bd36db750a02972945bc6d7"
  }
}
```
</details>

## Decompile Unknown Transaction Intent

| Function Name     | `decompile_unknown_transaction_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_decompileUnknownTransactionIntent` |
| Functionality     | There are certain cases where we might have some blob which we suspect is a transaction intent but we have no way of verifying whether that is true or not. Looking at the type id byte of the blob does not help either as it's a generic Struct type which is not too telling. For this specific use case, this library provides this function which attempts to decompile a transaction intent of an unknown type.   |
| Required Features | default   |
| Request Type      | `DecompileUnknownTransactionIntentRequest` |
| Response Type     | `DecompileUnknownTransactionIntentResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "instructions_output_kind": "Parsed",
  "compiled_unknown_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f5050800002102202209210380048944f6ce22315a26c4b90c6e7e09a093dcbc98c89e67d32b2dc10c087769746864726177210280010000000000000000000000000000000000000000000000000000850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b00000000000000000000000000000000000000000000000080010000000000000000000000000000000000000000000000000000210380034ed96f21d83d7303bdb2ca3fc6f6c6691504d5540f9bd41b07ed0c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a229000000000000000000000000000000000000000000000000800100000000000000000000000000000000000000000000000000000401800289ba4758731898e0850fbde5d412c080e4f8b7ea03174cb180d900018001000000000000000000000000000000000000000000000000000003018101000000020220870101000000000000000180010000000000000000000000000000000000000000000000000000210380048944f6ce22315a26c4b90c6e7e09a093dcbc98c89e67d32b2dc10c0d6465706f7369745f62617463682101830020200020220600012101200741001dc81ce1fd9b1de438972231e81db2cab2ec01f205019c7e947b2ef049c18763283394f18f7efd1ede7122a5b0ae68bcab671c6f28a83061c13c1b7413728a7400012101200741008b86278af6e6336c8e7a3d635f0fec9c467588397c4df4818f32e897238f2a3c1edb19118c9d9a09f9c2f98506486e96db89acc987a5b3dee4861e01ca8761d0000121012007410075a6696b28b00b4295ffdfeaf852e52736f8fbd2314e1ea087ce0215b799cba14a98d918be28cf71ed51eaa58d7b88e1a15ef50297e04ec97dbad77d7702784e01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba2921012007402a310d3ed1eacc1ccf3b7d59a91a6474415c647f55af42e2e912dc850b79f7418108a1bed1ee34103530372b1899853fff078c32b5e590bb718f74a1df32400a01022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe267421012007407eb1c121969dfba6cf43695a12f889e15081407ea455396cb02acdc101b277618531cb9a231ec48798c02c0bb73f9350f0e58bda6b6c8b6b5d6416ecd1f218010102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740c2277b73ff69e7e63b1c5ec98b23f71d7e419df1c69d5e58d4a73a9bdb18192b72f64410c7e19e7b88fd339112c8171928ae3669f39cad38050eb48a8ecb3c0d2200012101200741005f9290847274a47ce592718db3b1d7ace56db9fbcbc9139cd9e680715f7a316a535cccd292b1176e8d25a2b5dfc6907fc0527c8f5bd36db750a02972945bc6d7"
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
                  "type": "Address",
                  "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
                },
                "method_name": {
                  "type": "String",
                  "value": "withdraw"
                },
                "arguments": [
                  {
                    "type": "Address",
                    "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
                  "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
                  "address": "component_sim1qd8djmepmq7hxqaakt9rl3hkce532px42s8eh4qmqlks9f87dn"
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
                  "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
                  "address": "resource_sim1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsgvqtwe"
                }
              },
              {
                "instruction": "TAKE_FROM_WORKTOP",
                "resource_address": {
                  "type": "Address",
                  "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
                  "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
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
                "instruction": "CALL_METHOD",
                "component_address": {
                  "type": "Address",
                  "address": "account_sim1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqs7d0gtn"
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
          "signature": "001dc81ce1fd9b1de438972231e81db2cab2ec01f205019c7e947b2ef049c18763283394f18f7efd1ede7122a5b0ae68bcab671c6f28a83061c13c1b7413728a74"
        },
        {
          "curve": "EcdsaSecp256k1",
          "signature": "008b86278af6e6336c8e7a3d635f0fec9c467588397c4df4818f32e897238f2a3c1edb19118c9d9a09f9c2f98506486e96db89acc987a5b3dee4861e01ca8761d0"
        },
        {
          "curve": "EcdsaSecp256k1",
          "signature": "0075a6696b28b00b4295ffdfeaf852e52736f8fbd2314e1ea087ce0215b799cba14a98d918be28cf71ed51eaa58d7b88e1a15ef50297e04ec97dbad77d7702784e"
        },
        {
          "curve": "EddsaEd25519",
          "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
          "signature": "2a310d3ed1eacc1ccf3b7d59a91a6474415c647f55af42e2e912dc850b79f7418108a1bed1ee34103530372b1899853fff078c32b5e590bb718f74a1df32400a"
        },
        {
          "curve": "EddsaEd25519",
          "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
          "signature": "7eb1c121969dfba6cf43695a12f889e15081407ea455396cb02acdc101b277618531cb9a231ec48798c02c0bb73f9350f0e58bda6b6c8b6b5d6416ecd1f21801"
        },
        {
          "curve": "EddsaEd25519",
          "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
          "signature": "c2277b73ff69e7e63b1c5ec98b23f71d7e419df1c69d5e58d4a73a9bdb18192b72f64410c7e19e7b88fd339112c8171928ae3669f39cad38050eb48a8ecb3c0d"
        }
      ]
    },
    "notary_signature": {
      "curve": "EcdsaSecp256k1",
      "signature": "005f9290847274a47ce592718db3b1d7ace56db9fbcbc9139cd9e680715f7a316a535cccd292b1176e8d25a2b5dfc6907fc0527c8f5bd36db750a02972945bc6d7"
    }
  }
}
```
</details>

## Encode Address

| Function Name     | `encode_address` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_encodeAddress` |
| Functionality     | This function can be used when we have a byte array which we wish to do Bech32m encoding on. In this case, the HRP to use will be determined through the entity byte of the passed address hex string.   |
| Required Features | default   |
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
  "type": "PackageAddress",
  "address": "package_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpqmre2w5"
}
```
</details>

## Decode Address

| Function Name     | `decode_address` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_decodeAddress` |
| Functionality     | This function can be used to decode a Bech32m encoded address string into its equivalent hrp and data. In addition to that, this function provides other useful information on the address such as the network id and name that it is used for, and the entity type of the address.   |
| Required Features | default   |
| Request Type      | `DecodeAddressRequest` |
| Response Type     | `DecodeAddressResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k"
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "network_id": "242",
  "network_name": "simulator",
  "entity_type": "FungibleResource",
  "data": "010000000000000000000000000000000000000000000000000000",
  "hrp": "resource_sim"
}
```
</details>

## Sbor Encode

| Function Name     | `sbor_encode` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_sborEncode` |
| Functionality     | This function takes in a ScryptoSborValue and encodes it in SBOR.   |
| Required Features | default   |
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
| Functionality     | This function takes in a hex string and attempts to decode it into a ScryptoSborValue.   |
| Required Features | default   |
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
        "address": "package_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq57ks9j"
      },
      {
        "type": "Address",
        "address": "resource_sim1qyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszjakjx"
      },
      {
        "type": "Address",
        "address": "resource_sim1qgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqg79t2z"
      },
      {
        "type": "Bucket",
        "identifier": "4"
      },
      {
        "type": "Proof",
        "identifier": "5"
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
| Functionality     | Derives the virtual account component address given a public key and a network id.   |
| Required Features | default   |
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
    "address": "account_sim1ppkfdmv0q2cwz9cjxk5t8u0zx6pdydd9p2jv22nuwdhqyn4rgj"
  }
}
```
</details>

## Derive Virtual Identity Address

| Function Name     | `derive_virtual_identity_address` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_deriveVirtualIdentityAddress` |
| Functionality     | Derives the virtual identity component address given a public key and a network id.   |
| Required Features | default   |
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
    "address": "identity_sim1pdkfdmv0q2cwz9cjxk5t8u0zx6pdydd9p2jv22nuwdhqkcvkcq"
  }
}
```
</details>

## Derive Babylon Address From Olympia Address

| Function Name     | `derive_babylon_address_from_olympia_address` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_deriveBabylonAddressFromOlympiaAddress` |
| Functionality     | Derives the Babylon account address associated with the given Olympia account address   |
| Required Features | default   |
| Request Type      | `DeriveBabylonAddressFromOlympiaAddressRequest` |
| Response Type     | `DeriveBabylonAddressFromOlympiaAddressResponse` |

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
  "babylon_account_address": {
    "type": "ComponentAddress",
    "address": "account_rdx1pzg7l46zndnsluatt7dvyvvjuynzal3wa8rylqggh8xsq5pwrq"
  },
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
| Functionality     | Given an ECDSA Secp256k1 Public Key and Olympia network, this function derives the Olympia account address associated with the public key on that network.   |
| Required Features | default   |
| Request Type      | `DeriveOlympiaAddressFromPublicKeyRequest` |
| Response Type     | `DeriveOlympiaAddressFromPublicKeyResponse` |

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

## Derive Non Fungible Global Id From Public Key

| Function Name     | `derive_non_fungible_global_id_from_public_key` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_deriveNonFungibleGlobalIdFromPublicKey` |
| Functionality     | Derives the non-fungible global id of the virtual badge associated with a given public key   |
| Required Features | default   |
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
    "resource_address": {
      "type": "ResourceAddress",
      "address": "resource_sim1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq056vhf"
    },
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
| Functionality     | Performs static validation on the given notarized transaction.   |
| Required Features | default   |
| Request Type      | `StaticallyValidateTransactionRequest` |
| Response Type     | `StaticallyValidateTransactionResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "compiled_notarized_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f5050800002102202209210380048944f6ce22315a26c4b90c6e7e09a093dcbc98c89e67d32b2dc10c087769746864726177210280010000000000000000000000000000000000000000000000000000850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b00000000000000000000000000000000000000000000000080010000000000000000000000000000000000000000000000000000210380034ed96f21d83d7303bdb2ca3fc6f6c6691504d5540f9bd41b07ed0c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a229000000000000000000000000000000000000000000000000800100000000000000000000000000000000000000000000000000000401800289ba4758731898e0850fbde5d412c080e4f8b7ea03174cb180d900018001000000000000000000000000000000000000000000000000000003018101000000020220870101000000000000000180010000000000000000000000000000000000000000000000000000210380048944f6ce22315a26c4b90c6e7e09a093dcbc98c89e67d32b2dc10c0d6465706f7369745f62617463682101830020200020220600012101200741001dc81ce1fd9b1de438972231e81db2cab2ec01f205019c7e947b2ef049c18763283394f18f7efd1ede7122a5b0ae68bcab671c6f28a83061c13c1b7413728a7400012101200741008b86278af6e6336c8e7a3d635f0fec9c467588397c4df4818f32e897238f2a3c1edb19118c9d9a09f9c2f98506486e96db89acc987a5b3dee4861e01ca8761d0000121012007410075a6696b28b00b4295ffdfeaf852e52736f8fbd2314e1ea087ce0215b799cba14a98d918be28cf71ed51eaa58d7b88e1a15ef50297e04ec97dbad77d7702784e01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba2921012007402a310d3ed1eacc1ccf3b7d59a91a6474415c647f55af42e2e912dc850b79f7418108a1bed1ee34103530372b1899853fff078c32b5e590bb718f74a1df32400a01022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe267421012007407eb1c121969dfba6cf43695a12f889e15081407ea455396cb02acdc101b277618531cb9a231ec48798c02c0bb73f9350f0e58bda6b6c8b6b5d6416ecd1f218010102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740c2277b73ff69e7e63b1c5ec98b23f71d7e419df1c69d5e58d4a73a9bdb18192b72f64410c7e19e7b88fd339112c8171928ae3669f39cad38050eb48a8ecb3c0d2200012101200741001dc81ce1fd9b1de438972231e81db2cab2ec01f205019c7e947b2ef049c18763283394f18f7efd1ede7122a5b0ae68bcab671c6f28a83061c13c1b7413728a74",
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
| Functionality     | Given a network id, this function derives the Bech32m-encoded addresses of the set of known addresses.</br>        </br>        As an example, this function allows users to derive the XRD resource address, faucet component address, or account package address on any network (given that they know its network id).   |
| Required Features | default   |
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
    "address": "component_rdx1q0kryz5scup945usk39qjc2yjh6l5zsyuh8t7v5pk0ts3s4rve"
  },
  "faucet_package_address": {
    "type": "PackageAddress",
    "address": "package_rdx1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpqqs767h4"
  },
  "account_package_address": {
    "type": "PackageAddress",
    "address": "package_rdx1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzsew9vdj"
  },
  "xrd_resource_address": {
    "type": "ResourceAddress",
    "address": "resource_rdx1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqkcg7wf"
  },
  "system_token_resource_address": {
    "type": "ResourceAddress",
    "address": "resource_rdx1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpqxts0hs"
  },
  "ecdsa_secp256k1_token_resource_address": {
    "type": "ResourceAddress",
    "address": "resource_rdx1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqfkl4uk"
  },
  "eddsa_ed25519_token_resource_address": {
    "type": "ResourceAddress",
    "address": "resource_rdx1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqsuhhdf9"
  },
  "package_token_resource_address": {
    "type": "ResourceAddress",
    "address": "resource_rdx1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpsn2chzr"
  },
  "epoch_manager_system_address": {
    "type": "ComponentAddress",
    "address": "epochmanager_rdx1q5qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqcmqvad"
  },
  "clock_system_address": {
    "type": "ComponentAddress",
    "address": "clock_rdx1quqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqfhf25z"
  }
}
```
</details>

## Hash

| Function Name     | `hash` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_RadixEngineToolkitFFI_hash` |
| Functionality     | Hashes some payload through the hashing algorithm used in Scrypto and the Radix Engine.   |
| Required Features | default   |
| Request Type      | `HashRequest` |
| Response Type     | `HashResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "payload": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f5050800002102202209210380048944f6ce22315a26c4b90c6e7e09a093dcbc98c89e67d32b2dc10c087769746864726177210280010000000000000000000000000000000000000000000000000000850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b00000000000000000000000000000000000000000000000080010000000000000000000000000000000000000000000000000000210380034ed96f21d83d7303bdb2ca3fc6f6c6691504d5540f9bd41b07ed0c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a229000000000000000000000000000000000000000000000000800100000000000000000000000000000000000000000000000000000401800289ba4758731898e0850fbde5d412c080e4f8b7ea03174cb180d900018001000000000000000000000000000000000000000000000000000003018101000000020220870101000000000000000180010000000000000000000000000000000000000000000000000000210380048944f6ce22315a26c4b90c6e7e09a093dcbc98c89e67d32b2dc10c0d6465706f7369745f62617463682101830020200020220600012101200741001dc81ce1fd9b1de438972231e81db2cab2ec01f205019c7e947b2ef049c18763283394f18f7efd1ede7122a5b0ae68bcab671c6f28a83061c13c1b7413728a7400012101200741008b86278af6e6336c8e7a3d635f0fec9c467588397c4df4818f32e897238f2a3c1edb19118c9d9a09f9c2f98506486e96db89acc987a5b3dee4861e01ca8761d0000121012007410075a6696b28b00b4295ffdfeaf852e52736f8fbd2314e1ea087ce0215b799cba14a98d918be28cf71ed51eaa58d7b88e1a15ef50297e04ec97dbad77d7702784e01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba2921012007402a310d3ed1eacc1ccf3b7d59a91a6474415c647f55af42e2e912dc850b79f7418108a1bed1ee34103530372b1899853fff078c32b5e590bb718f74a1df32400a01022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe267421012007407eb1c121969dfba6cf43695a12f889e15081407ea455396cb02acdc101b277618531cb9a231ec48798c02c0bb73f9350f0e58bda6b6c8b6b5d6416ecd1f218010102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740c2277b73ff69e7e63b1c5ec98b23f71d7e419df1c69d5e58d4a73a9bdb18192b72f64410c7e19e7b88fd339112c8171928ae3669f39cad38050eb48a8ecb3c0d2200012101200741001dc81ce1fd9b1de438972231e81db2cab2ec01f205019c7e947b2ef049c18763283394f18f7efd1ede7122a5b0ae68bcab671c6f28a83061c13c1b7413728a74"
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "value": "87a708b4c71fcbf0cfddf4d6f31e099c52eb07eb9a3d4a5f9c23bb27f34be944"
}
```
</details>
