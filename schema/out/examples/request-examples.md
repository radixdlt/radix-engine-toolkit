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
            "address": "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
          },
          "method_name": {
            "type": "String",
            "value": "withdraw"
          },
          "arguments": [
            {
              "type": "Address",
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
            "address": "component_sim1pyakxvzls3cwkfp25xz7dufp9jnw6wzxe3cxaku2ju7tlyuvusk6y9"
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
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
            "address": "resource_sim1qy227ueqye9wcv9nlt7jqfdvw3a3wvlhwp5p53sttpefsg2d5zv25c"
          }
        },
        {
          "instruction": "TAKE_FROM_WORKTOP",
          "resource_address": {
            "type": "Address",
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
            "address": "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
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
          "address": "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
        },
        "method_name": {
          "type": "String",
          "value": "withdraw"
        },
        "arguments": [
          {
            "type": "Address",
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
          "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
          "address": "component_sim1pyakxvzls3cwkfp25xz7dufp9jnw6wzxe3cxaku2ju7tlyuvusk6y9"
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
          "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
          "address": "resource_sim1qy227ueqye9wcv9nlt7jqfdvw3a3wvlhwp5p53sttpefsg2d5zv25c"
        }
      },
      {
        "instruction": "TAKE_FROM_WORKTOP",
        "resource_address": {
          "type": "Address",
          "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
          "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
          "address": "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
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
      "value": "CALL_METHOD\n    Address(\"account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl\")\n    \"withdraw\"\n    Address(\"resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00\")\n    Decimal(\"5\");\nTAKE_FROM_WORKTOP_BY_AMOUNT\n    Decimal(\"2\")\n    Address(\"resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00\")\n    Bucket(\"bucket1\");\nCALL_METHOD\n    Address(\"component_sim1pyakxvzls3cwkfp25xz7dufp9jnw6wzxe3cxaku2ju7tlyuvusk6y9\")\n    \"buy_gumball\"\n    Bucket(\"bucket1\");\nASSERT_WORKTOP_CONTAINS_BY_AMOUNT\n    Decimal(\"3\")\n    Address(\"resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00\");\nASSERT_WORKTOP_CONTAINS\n    Address(\"resource_sim1qy227ueqye9wcv9nlt7jqfdvw3a3wvlhwp5p53sttpefsg2d5zv25c\");\nTAKE_FROM_WORKTOP\n    Address(\"resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00\")\n    Bucket(\"bucket2\");\nRETURN_TO_WORKTOP\n    Bucket(\"bucket2\");\nTAKE_FROM_WORKTOP_BY_IDS\n    Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"))\n    Address(\"resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00\")\n    Bucket(\"bucket3\");\nCALL_METHOD\n    Address(\"account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl\")\n    \"deposit_batch\"\n    Expression(\"ENTIRE_WORKTOP\");\n"
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
    "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl",
    "component_sim1pyakxvzls3cwkfp25xz7dufp9jnw6wzxe3cxaku2ju7tlyuvusk6y9"
  ],
  "resource_addresses": [
    "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00",
    "resource_sim1qy227ueqye9wcv9nlt7jqfdvw3a3wvlhwp5p53sttpefsg2d5zv25c"
  ],
  "account_addresses": [
    "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
  ],
  "accounts_requiring_auth": [
    "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
  ],
  "accounts_withdrawn_from": [
    "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
  ],
  "accounts_deposited_into": [
    "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
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
            "address": "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
          },
          "method_name": {
            "type": "String",
            "value": "withdraw"
          },
          "arguments": [
            {
              "type": "Address",
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
            "address": "component_sim1pyakxvzls3cwkfp25xz7dufp9jnw6wzxe3cxaku2ju7tlyuvusk6y9"
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
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
            "address": "resource_sim1qy227ueqye9wcv9nlt7jqfdvw3a3wvlhwp5p53sttpefsg2d5zv25c"
          }
        },
        {
          "instruction": "TAKE_FROM_WORKTOP",
          "resource_address": {
            "type": "Address",
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
            "address": "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
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
  "compiled_intent": "4d21022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f5050800002102202209210380073660440a73ddca360f5839cfece756db9080f88c2333cca9937d6234f10c087769746864726177210280010000000000000000000000000000000000000000000000000000000000850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b00000000000000000000000000000000000000000000000080010000000000000000000000000000000000000000000000000000000000210380093b63305f8470eb242aa185e6f1212ca6ed3846cc706edb8a973cbf938c0c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a229000000000000000000000000000000000000000000000000800100000000000000000000000000000000000000000000000000000000000401800114af7320264aec30b3fafd2025ac747b1733f770681a460b587298214d00018001000000000000000000000000000000000000000000000000000000000003018101000000020220870101000000000000000180010000000000000000000000000000000000000000000000000000000000210380073660440a73ddca360f5839cfece756db9080f88c2333cca9937d6234f10c0d6465706f7369745f626174636821018300202000"
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
  "compiled_intent": "4d21022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f5050800002102202209210380073660440a73ddca360f5839cfece756db9080f88c2333cca9937d6234f10c087769746864726177210280010000000000000000000000000000000000000000000000000000000000850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b00000000000000000000000000000000000000000000000080010000000000000000000000000000000000000000000000000000000000210380093b63305f8470eb242aa185e6f1212ca6ed3846cc706edb8a973cbf938c0c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a229000000000000000000000000000000000000000000000000800100000000000000000000000000000000000000000000000000000000000401800114af7320264aec30b3fafd2025ac747b1733f770681a460b587298214d00018001000000000000000000000000000000000000000000000000000000000003018101000000020220870101000000000000000180010000000000000000000000000000000000000000000000000000000000210380073660440a73ddca360f5839cfece756db9080f88c2333cca9937d6234f10c0d6465706f7369745f626174636821018300202000"
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
            "address": "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
          },
          "method_name": {
            "type": "String",
            "value": "withdraw"
          },
          "arguments": [
            {
              "type": "Address",
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
            "address": "component_sim1pyakxvzls3cwkfp25xz7dufp9jnw6wzxe3cxaku2ju7tlyuvusk6y9"
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
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
            "address": "resource_sim1qy227ueqye9wcv9nlt7jqfdvw3a3wvlhwp5p53sttpefsg2d5zv25c"
          }
        },
        {
          "instruction": "TAKE_FROM_WORKTOP",
          "resource_address": {
            "type": "Address",
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
            "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
            "address": "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
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
              "address": "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
            },
            "method_name": {
              "type": "String",
              "value": "withdraw"
            },
            "arguments": [
              {
                "type": "Address",
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
              "address": "component_sim1pyakxvzls3cwkfp25xz7dufp9jnw6wzxe3cxaku2ju7tlyuvusk6y9"
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
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
              "address": "resource_sim1qy227ueqye9wcv9nlt7jqfdvw3a3wvlhwp5p53sttpefsg2d5zv25c"
            }
          },
          {
            "instruction": "TAKE_FROM_WORKTOP",
            "resource_address": {
              "type": "Address",
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
              "address": "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
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
      "signature": "010fe04368eb040b84a52477b08dac77bfa0a8c88845b9c6de509c02673daf712b56c88bcb13523ba264cdf25d00b48771d3619f6fb8d815199a03cf438971ed36"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "00334d7c48f72eab4fcc7868e18340d657a074495fe2d8c2e3542093cfbf32c4a11787073f8ad036c8f250d9bb2394c04e26412c4f47d8fae190cdaab0db65d20e"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "01964f30050b145cf3f32e89d8a407fae2b838ce491a85aea39f4efe4a3093994e466cde03e3628b8c56b7d02b3cb0781795085f35e825620298338e626b0756ae"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
      "signature": "523cf102570eed7cada9c616ac918f61f5341f7e7cecc3ee9869a22184edb84cf4a8df93ebcfb56a7a36b2a771819fe5aa8abb9309a2b90f52eefaa70400b808"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
      "signature": "c227eae48be23995067e016aee13f57270f3d8b7d30d48d19a9795f1d4efc4be1e4077a0e57bc0bbc120660fc2655db30c224ec71e55a7e9c2eb187badafae08"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
      "signature": "26957d8f507caea34293ac2895492996ddeacba0f1be0ebf11b5262fa75e2e233742024e97b3abf7f626ec27b47fc4ee68c6fdd4bc0978fca705efdf6389c80e"
    }
  ]
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "compiled_intent": "4d210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f5050800002102202209210380073660440a73ddca360f5839cfece756db9080f88c2333cca9937d6234f10c087769746864726177210280010000000000000000000000000000000000000000000000000000000000850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b00000000000000000000000000000000000000000000000080010000000000000000000000000000000000000000000000000000000000210380093b63305f8470eb242aa185e6f1212ca6ed3846cc706edb8a973cbf938c0c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a229000000000000000000000000000000000000000000000000800100000000000000000000000000000000000000000000000000000000000401800114af7320264aec30b3fafd2025ac747b1733f770681a460b587298214d00018001000000000000000000000000000000000000000000000000000000000003018101000000020220870101000000000000000180010000000000000000000000000000000000000000000000000000000000210380073660440a73ddca360f5839cfece756db9080f88c2333cca9937d6234f10c0d6465706f7369745f62617463682101830020200020220600012101200741010fe04368eb040b84a52477b08dac77bfa0a8c88845b9c6de509c02673daf712b56c88bcb13523ba264cdf25d00b48771d3619f6fb8d815199a03cf438971ed360001210120074100334d7c48f72eab4fcc7868e18340d657a074495fe2d8c2e3542093cfbf32c4a11787073f8ad036c8f250d9bb2394c04e26412c4f47d8fae190cdaab0db65d20e0001210120074101964f30050b145cf3f32e89d8a407fae2b838ce491a85aea39f4efe4a3093994e466cde03e3628b8c56b7d02b3cb0781795085f35e825620298338e626b0756ae01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740523cf102570eed7cada9c616ac918f61f5341f7e7cecc3ee9869a22184edb84cf4a8df93ebcfb56a7a36b2a771819fe5aa8abb9309a2b90f52eefaa70400b80801022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740c227eae48be23995067e016aee13f57270f3d8b7d30d48d19a9795f1d4efc4be1e4077a0e57bc0bbc120660fc2655db30c224ec71e55a7e9c2eb187badafae080102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b210120074026957d8f507caea34293ac2895492996ddeacba0f1be0ebf11b5262fa75e2e233742024e97b3abf7f626ec27b47fc4ee68c6fdd4bc0978fca705efdf6389c80e"
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
  "compiled_signed_intent": "4d210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f5050800002102202209210380073660440a73ddca360f5839cfece756db9080f88c2333cca9937d6234f10c087769746864726177210280010000000000000000000000000000000000000000000000000000000000850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b00000000000000000000000000000000000000000000000080010000000000000000000000000000000000000000000000000000000000210380093b63305f8470eb242aa185e6f1212ca6ed3846cc706edb8a973cbf938c0c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a229000000000000000000000000000000000000000000000000800100000000000000000000000000000000000000000000000000000000000401800114af7320264aec30b3fafd2025ac747b1733f770681a460b587298214d00018001000000000000000000000000000000000000000000000000000000000003018101000000020220870101000000000000000180010000000000000000000000000000000000000000000000000000000000210380073660440a73ddca360f5839cfece756db9080f88c2333cca9937d6234f10c0d6465706f7369745f62617463682101830020200020220600012101200741010fe04368eb040b84a52477b08dac77bfa0a8c88845b9c6de509c02673daf712b56c88bcb13523ba264cdf25d00b48771d3619f6fb8d815199a03cf438971ed360001210120074100334d7c48f72eab4fcc7868e18340d657a074495fe2d8c2e3542093cfbf32c4a11787073f8ad036c8f250d9bb2394c04e26412c4f47d8fae190cdaab0db65d20e0001210120074101964f30050b145cf3f32e89d8a407fae2b838ce491a85aea39f4efe4a3093994e466cde03e3628b8c56b7d02b3cb0781795085f35e825620298338e626b0756ae01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740523cf102570eed7cada9c616ac918f61f5341f7e7cecc3ee9869a22184edb84cf4a8df93ebcfb56a7a36b2a771819fe5aa8abb9309a2b90f52eefaa70400b80801022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740c227eae48be23995067e016aee13f57270f3d8b7d30d48d19a9795f1d4efc4be1e4077a0e57bc0bbc120660fc2655db30c224ec71e55a7e9c2eb187badafae080102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b210120074026957d8f507caea34293ac2895492996ddeacba0f1be0ebf11b5262fa75e2e233742024e97b3abf7f626ec27b47fc4ee68c6fdd4bc0978fca705efdf6389c80e"
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
              "address": "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
            },
            "method_name": {
              "type": "String",
              "value": "withdraw"
            },
            "arguments": [
              {
                "type": "Address",
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
              "address": "component_sim1pyakxvzls3cwkfp25xz7dufp9jnw6wzxe3cxaku2ju7tlyuvusk6y9"
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
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
              "address": "resource_sim1qy227ueqye9wcv9nlt7jqfdvw3a3wvlhwp5p53sttpefsg2d5zv25c"
            }
          },
          {
            "instruction": "TAKE_FROM_WORKTOP",
            "resource_address": {
              "type": "Address",
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
              "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
              "address": "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
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
      "signature": "010fe04368eb040b84a52477b08dac77bfa0a8c88845b9c6de509c02673daf712b56c88bcb13523ba264cdf25d00b48771d3619f6fb8d815199a03cf438971ed36"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "00334d7c48f72eab4fcc7868e18340d657a074495fe2d8c2e3542093cfbf32c4a11787073f8ad036c8f250d9bb2394c04e26412c4f47d8fae190cdaab0db65d20e"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "01964f30050b145cf3f32e89d8a407fae2b838ce491a85aea39f4efe4a3093994e466cde03e3628b8c56b7d02b3cb0781795085f35e825620298338e626b0756ae"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
      "signature": "523cf102570eed7cada9c616ac918f61f5341f7e7cecc3ee9869a22184edb84cf4a8df93ebcfb56a7a36b2a771819fe5aa8abb9309a2b90f52eefaa70400b808"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
      "signature": "c227eae48be23995067e016aee13f57270f3d8b7d30d48d19a9795f1d4efc4be1e4077a0e57bc0bbc120660fc2655db30c224ec71e55a7e9c2eb187badafae08"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
      "signature": "26957d8f507caea34293ac2895492996ddeacba0f1be0ebf11b5262fa75e2e233742024e97b3abf7f626ec27b47fc4ee68c6fdd4bc0978fca705efdf6389c80e"
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
                "address": "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
              },
              "method_name": {
                "type": "String",
                "value": "withdraw"
              },
              "arguments": [
                {
                  "type": "Address",
                  "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
                "address": "component_sim1pyakxvzls3cwkfp25xz7dufp9jnw6wzxe3cxaku2ju7tlyuvusk6y9"
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
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
                "address": "resource_sim1qy227ueqye9wcv9nlt7jqfdvw3a3wvlhwp5p53sttpefsg2d5zv25c"
              }
            },
            {
              "instruction": "TAKE_FROM_WORKTOP",
              "resource_address": {
                "type": "Address",
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
                "address": "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
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
        "signature": "010fe04368eb040b84a52477b08dac77bfa0a8c88845b9c6de509c02673daf712b56c88bcb13523ba264cdf25d00b48771d3619f6fb8d815199a03cf438971ed36"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "00334d7c48f72eab4fcc7868e18340d657a074495fe2d8c2e3542093cfbf32c4a11787073f8ad036c8f250d9bb2394c04e26412c4f47d8fae190cdaab0db65d20e"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "01964f30050b145cf3f32e89d8a407fae2b838ce491a85aea39f4efe4a3093994e466cde03e3628b8c56b7d02b3cb0781795085f35e825620298338e626b0756ae"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
        "signature": "523cf102570eed7cada9c616ac918f61f5341f7e7cecc3ee9869a22184edb84cf4a8df93ebcfb56a7a36b2a771819fe5aa8abb9309a2b90f52eefaa70400b808"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
        "signature": "c227eae48be23995067e016aee13f57270f3d8b7d30d48d19a9795f1d4efc4be1e4077a0e57bc0bbc120660fc2655db30c224ec71e55a7e9c2eb187badafae08"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
        "signature": "26957d8f507caea34293ac2895492996ddeacba0f1be0ebf11b5262fa75e2e233742024e97b3abf7f626ec27b47fc4ee68c6fdd4bc0978fca705efdf6389c80e"
      }
    ]
  },
  "notary_signature": {
    "curve": "EcdsaSecp256k1",
    "signature": "0154ced0890bb19f8a59aab7afcefa6989d644cbe69acb7abd5077b04bf5b2afa93bc7d6ab2909ea1c8c7bf9d0acd0dd6af7d77ba9733ff16eb7dec32b2bb19d78"
  }
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "compiled_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f5050800002102202209210380073660440a73ddca360f5839cfece756db9080f88c2333cca9937d6234f10c087769746864726177210280010000000000000000000000000000000000000000000000000000000000850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b00000000000000000000000000000000000000000000000080010000000000000000000000000000000000000000000000000000000000210380093b63305f8470eb242aa185e6f1212ca6ed3846cc706edb8a973cbf938c0c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a229000000000000000000000000000000000000000000000000800100000000000000000000000000000000000000000000000000000000000401800114af7320264aec30b3fafd2025ac747b1733f770681a460b587298214d00018001000000000000000000000000000000000000000000000000000000000003018101000000020220870101000000000000000180010000000000000000000000000000000000000000000000000000000000210380073660440a73ddca360f5839cfece756db9080f88c2333cca9937d6234f10c0d6465706f7369745f62617463682101830020200020220600012101200741010fe04368eb040b84a52477b08dac77bfa0a8c88845b9c6de509c02673daf712b56c88bcb13523ba264cdf25d00b48771d3619f6fb8d815199a03cf438971ed360001210120074100334d7c48f72eab4fcc7868e18340d657a074495fe2d8c2e3542093cfbf32c4a11787073f8ad036c8f250d9bb2394c04e26412c4f47d8fae190cdaab0db65d20e0001210120074101964f30050b145cf3f32e89d8a407fae2b838ce491a85aea39f4efe4a3093994e466cde03e3628b8c56b7d02b3cb0781795085f35e825620298338e626b0756ae01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740523cf102570eed7cada9c616ac918f61f5341f7e7cecc3ee9869a22184edb84cf4a8df93ebcfb56a7a36b2a771819fe5aa8abb9309a2b90f52eefaa70400b80801022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740c227eae48be23995067e016aee13f57270f3d8b7d30d48d19a9795f1d4efc4be1e4077a0e57bc0bbc120660fc2655db30c224ec71e55a7e9c2eb187badafae080102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b210120074026957d8f507caea34293ac2895492996ddeacba0f1be0ebf11b5262fa75e2e233742024e97b3abf7f626ec27b47fc4ee68c6fdd4bc0978fca705efdf6389c80e22000121012007410154ced0890bb19f8a59aab7afcefa6989d644cbe69acb7abd5077b04bf5b2afa93bc7d6ab2909ea1c8c7bf9d0acd0dd6af7d77ba9733ff16eb7dec32b2bb19d78"
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
  "compiled_notarized_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f5050800002102202209210380073660440a73ddca360f5839cfece756db9080f88c2333cca9937d6234f10c087769746864726177210280010000000000000000000000000000000000000000000000000000000000850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b00000000000000000000000000000000000000000000000080010000000000000000000000000000000000000000000000000000000000210380093b63305f8470eb242aa185e6f1212ca6ed3846cc706edb8a973cbf938c0c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a229000000000000000000000000000000000000000000000000800100000000000000000000000000000000000000000000000000000000000401800114af7320264aec30b3fafd2025ac747b1733f770681a460b587298214d00018001000000000000000000000000000000000000000000000000000000000003018101000000020220870101000000000000000180010000000000000000000000000000000000000000000000000000000000210380073660440a73ddca360f5839cfece756db9080f88c2333cca9937d6234f10c0d6465706f7369745f62617463682101830020200020220600012101200741010fe04368eb040b84a52477b08dac77bfa0a8c88845b9c6de509c02673daf712b56c88bcb13523ba264cdf25d00b48771d3619f6fb8d815199a03cf438971ed360001210120074100334d7c48f72eab4fcc7868e18340d657a074495fe2d8c2e3542093cfbf32c4a11787073f8ad036c8f250d9bb2394c04e26412c4f47d8fae190cdaab0db65d20e0001210120074101964f30050b145cf3f32e89d8a407fae2b838ce491a85aea39f4efe4a3093994e466cde03e3628b8c56b7d02b3cb0781795085f35e825620298338e626b0756ae01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740523cf102570eed7cada9c616ac918f61f5341f7e7cecc3ee9869a22184edb84cf4a8df93ebcfb56a7a36b2a771819fe5aa8abb9309a2b90f52eefaa70400b80801022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740c227eae48be23995067e016aee13f57270f3d8b7d30d48d19a9795f1d4efc4be1e4077a0e57bc0bbc120660fc2655db30c224ec71e55a7e9c2eb187badafae080102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b210120074026957d8f507caea34293ac2895492996ddeacba0f1be0ebf11b5262fa75e2e233742024e97b3abf7f626ec27b47fc4ee68c6fdd4bc0978fca705efdf6389c80e22000121012007410154ced0890bb19f8a59aab7afcefa6989d644cbe69acb7abd5077b04bf5b2afa93bc7d6ab2909ea1c8c7bf9d0acd0dd6af7d77ba9733ff16eb7dec32b2bb19d78"
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
                "address": "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
              },
              "method_name": {
                "type": "String",
                "value": "withdraw"
              },
              "arguments": [
                {
                  "type": "Address",
                  "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
                "address": "component_sim1pyakxvzls3cwkfp25xz7dufp9jnw6wzxe3cxaku2ju7tlyuvusk6y9"
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
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
                "address": "resource_sim1qy227ueqye9wcv9nlt7jqfdvw3a3wvlhwp5p53sttpefsg2d5zv25c"
              }
            },
            {
              "instruction": "TAKE_FROM_WORKTOP",
              "resource_address": {
                "type": "Address",
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
                "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
                "address": "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
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
        "signature": "010fe04368eb040b84a52477b08dac77bfa0a8c88845b9c6de509c02673daf712b56c88bcb13523ba264cdf25d00b48771d3619f6fb8d815199a03cf438971ed36"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "00334d7c48f72eab4fcc7868e18340d657a074495fe2d8c2e3542093cfbf32c4a11787073f8ad036c8f250d9bb2394c04e26412c4f47d8fae190cdaab0db65d20e"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "01964f30050b145cf3f32e89d8a407fae2b838ce491a85aea39f4efe4a3093994e466cde03e3628b8c56b7d02b3cb0781795085f35e825620298338e626b0756ae"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
        "signature": "523cf102570eed7cada9c616ac918f61f5341f7e7cecc3ee9869a22184edb84cf4a8df93ebcfb56a7a36b2a771819fe5aa8abb9309a2b90f52eefaa70400b808"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
        "signature": "c227eae48be23995067e016aee13f57270f3d8b7d30d48d19a9795f1d4efc4be1e4077a0e57bc0bbc120660fc2655db30c224ec71e55a7e9c2eb187badafae08"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
        "signature": "26957d8f507caea34293ac2895492996ddeacba0f1be0ebf11b5262fa75e2e233742024e97b3abf7f626ec27b47fc4ee68c6fdd4bc0978fca705efdf6389c80e"
      }
    ]
  },
  "notary_signature": {
    "curve": "EcdsaSecp256k1",
    "signature": "0154ced0890bb19f8a59aab7afcefa6989d644cbe69acb7abd5077b04bf5b2afa93bc7d6ab2909ea1c8c7bf9d0acd0dd6af7d77ba9733ff16eb7dec32b2bb19d78"
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
  "compiled_unknown_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f5050800002102202209210380073660440a73ddca360f5839cfece756db9080f88c2333cca9937d6234f10c087769746864726177210280010000000000000000000000000000000000000000000000000000000000850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b00000000000000000000000000000000000000000000000080010000000000000000000000000000000000000000000000000000000000210380093b63305f8470eb242aa185e6f1212ca6ed3846cc706edb8a973cbf938c0c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a229000000000000000000000000000000000000000000000000800100000000000000000000000000000000000000000000000000000000000401800114af7320264aec30b3fafd2025ac747b1733f770681a460b587298214d00018001000000000000000000000000000000000000000000000000000000000003018101000000020220870101000000000000000180010000000000000000000000000000000000000000000000000000000000210380073660440a73ddca360f5839cfece756db9080f88c2333cca9937d6234f10c0d6465706f7369745f62617463682101830020200020220600012101200741010fe04368eb040b84a52477b08dac77bfa0a8c88845b9c6de509c02673daf712b56c88bcb13523ba264cdf25d00b48771d3619f6fb8d815199a03cf438971ed360001210120074100334d7c48f72eab4fcc7868e18340d657a074495fe2d8c2e3542093cfbf32c4a11787073f8ad036c8f250d9bb2394c04e26412c4f47d8fae190cdaab0db65d20e0001210120074101964f30050b145cf3f32e89d8a407fae2b838ce491a85aea39f4efe4a3093994e466cde03e3628b8c56b7d02b3cb0781795085f35e825620298338e626b0756ae01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740523cf102570eed7cada9c616ac918f61f5341f7e7cecc3ee9869a22184edb84cf4a8df93ebcfb56a7a36b2a771819fe5aa8abb9309a2b90f52eefaa70400b80801022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740c227eae48be23995067e016aee13f57270f3d8b7d30d48d19a9795f1d4efc4be1e4077a0e57bc0bbc120660fc2655db30c224ec71e55a7e9c2eb187badafae080102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b210120074026957d8f507caea34293ac2895492996ddeacba0f1be0ebf11b5262fa75e2e233742024e97b3abf7f626ec27b47fc4ee68c6fdd4bc0978fca705efdf6389c80e22000121012007410154ced0890bb19f8a59aab7afcefa6989d644cbe69acb7abd5077b04bf5b2afa93bc7d6ab2909ea1c8c7bf9d0acd0dd6af7d77ba9733ff16eb7dec32b2bb19d78"
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
                  "address": "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
                },
                "method_name": {
                  "type": "String",
                  "value": "withdraw"
                },
                "arguments": [
                  {
                    "type": "Address",
                    "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
                  "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
                  "address": "component_sim1pyakxvzls3cwkfp25xz7dufp9jnw6wzxe3cxaku2ju7tlyuvusk6y9"
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
                  "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
                  "address": "resource_sim1qy227ueqye9wcv9nlt7jqfdvw3a3wvlhwp5p53sttpefsg2d5zv25c"
                }
              },
              {
                "instruction": "TAKE_FROM_WORKTOP",
                "resource_address": {
                  "type": "Address",
                  "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
                  "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
                  "address": "account_sim1qumxq3q2w0wu5ds0tquulm882mdepq8c3s3n8n9fjd7kyd836h8xyl"
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
          "signature": "010fe04368eb040b84a52477b08dac77bfa0a8c88845b9c6de509c02673daf712b56c88bcb13523ba264cdf25d00b48771d3619f6fb8d815199a03cf438971ed36"
        },
        {
          "curve": "EcdsaSecp256k1",
          "signature": "00334d7c48f72eab4fcc7868e18340d657a074495fe2d8c2e3542093cfbf32c4a11787073f8ad036c8f250d9bb2394c04e26412c4f47d8fae190cdaab0db65d20e"
        },
        {
          "curve": "EcdsaSecp256k1",
          "signature": "01964f30050b145cf3f32e89d8a407fae2b838ce491a85aea39f4efe4a3093994e466cde03e3628b8c56b7d02b3cb0781795085f35e825620298338e626b0756ae"
        },
        {
          "curve": "EddsaEd25519",
          "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
          "signature": "523cf102570eed7cada9c616ac918f61f5341f7e7cecc3ee9869a22184edb84cf4a8df93ebcfb56a7a36b2a771819fe5aa8abb9309a2b90f52eefaa70400b808"
        },
        {
          "curve": "EddsaEd25519",
          "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
          "signature": "c227eae48be23995067e016aee13f57270f3d8b7d30d48d19a9795f1d4efc4be1e4077a0e57bc0bbc120660fc2655db30c224ec71e55a7e9c2eb187badafae08"
        },
        {
          "curve": "EddsaEd25519",
          "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
          "signature": "26957d8f507caea34293ac2895492996ddeacba0f1be0ebf11b5262fa75e2e233742024e97b3abf7f626ec27b47fc4ee68c6fdd4bc0978fca705efdf6389c80e"
        }
      ]
    },
    "notary_signature": {
      "curve": "EcdsaSecp256k1",
      "signature": "0154ced0890bb19f8a59aab7afcefa6989d644cbe69acb7abd5077b04bf5b2afa93bc7d6ab2909ea1c8c7bf9d0acd0dd6af7d77ba9733ff16eb7dec32b2bb19d78"
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
  "address_bytes": "000000000000000000000000000000000000000000000000000000000002",
  "network_id": "242"
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "address": "package_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzursw36"
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
  "address": "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3waw00"
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
  "data": "010000000000000000000000000000000000000000000000000000000000",
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
  "encoded_value": "4d2108800000000000000000000000000000000000000000000000000000000000008104000000820500000083018406060606060606060606060606060606060606060606060606060606060606068507070707070707070707070707070707070707070707070707070707070707078608080808080808080808080808080808080808080808080808080808080808080808080808080808080808080808080808080808080808080808080808080808870003616263",
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
        "address": "package_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq0x3dzh"
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
  "virtual_account_address": "account_sim1ptgge5mvjmkc7q4suyt3yddgk0c7yd5z6g662z4yc548cumwd7lcna"
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
  "virtual_identity_address": "identity_sim1pngge5mvjmkc7q4suyt3yddgk0c7yd5z6g662z4yc548cumw05fhdf"
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
  "babylon_account_address": "account_rdx1pte8u653alt59xm8ple6khu6cgce9cfx9mlza6wxf7qs3wwdazuagr",
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
  "compiled_notarized_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f5050800002102202209210380073660440a73ddca360f5839cfece756db9080f88c2333cca9937d6234f10c087769746864726177210280010000000000000000000000000000000000000000000000000000000000850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b00000000000000000000000000000000000000000000000080010000000000000000000000000000000000000000000000000000000000210380093b63305f8470eb242aa185e6f1212ca6ed3846cc706edb8a973cbf938c0c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a229000000000000000000000000000000000000000000000000800100000000000000000000000000000000000000000000000000000000000401800114af7320264aec30b3fafd2025ac747b1733f770681a460b587298214d00018001000000000000000000000000000000000000000000000000000000000003018101000000020220870101000000000000000180010000000000000000000000000000000000000000000000000000000000210380073660440a73ddca360f5839cfece756db9080f88c2333cca9937d6234f10c0d6465706f7369745f62617463682101830020200020220600012101200741010fe04368eb040b84a52477b08dac77bfa0a8c88845b9c6de509c02673daf712b56c88bcb13523ba264cdf25d00b48771d3619f6fb8d815199a03cf438971ed360001210120074100334d7c48f72eab4fcc7868e18340d657a074495fe2d8c2e3542093cfbf32c4a11787073f8ad036c8f250d9bb2394c04e26412c4f47d8fae190cdaab0db65d20e0001210120074101964f30050b145cf3f32e89d8a407fae2b838ce491a85aea39f4efe4a3093994e466cde03e3628b8c56b7d02b3cb0781795085f35e825620298338e626b0756ae01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740523cf102570eed7cada9c616ac918f61f5341f7e7cecc3ee9869a22184edb84cf4a8df93ebcfb56a7a36b2a771819fe5aa8abb9309a2b90f52eefaa70400b80801022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740c227eae48be23995067e016aee13f57270f3d8b7d30d48d19a9795f1d4efc4be1e4077a0e57bc0bbc120660fc2655db30c224ec71e55a7e9c2eb187badafae080102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b210120074026957d8f507caea34293ac2895492996ddeacba0f1be0ebf11b5262fa75e2e233742024e97b3abf7f626ec27b47fc4ee68c6fdd4bc0978fca705efdf6389c80e2200012101200741010fe04368eb040b84a52477b08dac77bfa0a8c88845b9c6de509c02673daf712b56c88bcb13523ba264cdf25d00b48771d3619f6fb8d815199a03cf438971ed36",
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
  "faucet_package_address": "package_rdx1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzq07s3y8",
  "account_package_address": "package_rdx1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq9r3clqr",
  "xrd_resource_address": "resource_rdx1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq09z0ln",
  "system_token_resource_address": "resource_rdx1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzreev97",
  "ecdsa_secp256k1_token_resource_address": "resource_rdx1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqsuc0kn",
  "eddsa_ed25519_token_resource_address": "resource_rdx1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpd2v6tp",
  "package_token_resource_address": "resource_rdx1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqr70decv",
  "epoch_manager_system_address": "epochmanager_rdx1qvqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqsu426p",
  "clock_system_address": "clock_rdx1q5qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqsxvvv0"
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
  "payload": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f5050800002102202209210380073660440a73ddca360f5839cfece756db9080f88c2333cca9937d6234f10c087769746864726177210280010000000000000000000000000000000000000000000000000000000000850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b00000000000000000000000000000000000000000000000080010000000000000000000000000000000000000000000000000000000000210380093b63305f8470eb242aa185e6f1212ca6ed3846cc706edb8a973cbf938c0c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a229000000000000000000000000000000000000000000000000800100000000000000000000000000000000000000000000000000000000000401800114af7320264aec30b3fafd2025ac747b1733f770681a460b587298214d00018001000000000000000000000000000000000000000000000000000000000003018101000000020220870101000000000000000180010000000000000000000000000000000000000000000000000000000000210380073660440a73ddca360f5839cfece756db9080f88c2333cca9937d6234f10c0d6465706f7369745f62617463682101830020200020220600012101200741010fe04368eb040b84a52477b08dac77bfa0a8c88845b9c6de509c02673daf712b56c88bcb13523ba264cdf25d00b48771d3619f6fb8d815199a03cf438971ed360001210120074100334d7c48f72eab4fcc7868e18340d657a074495fe2d8c2e3542093cfbf32c4a11787073f8ad036c8f250d9bb2394c04e26412c4f47d8fae190cdaab0db65d20e0001210120074101964f30050b145cf3f32e89d8a407fae2b838ce491a85aea39f4efe4a3093994e466cde03e3628b8c56b7d02b3cb0781795085f35e825620298338e626b0756ae01022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba292101200740523cf102570eed7cada9c616ac918f61f5341f7e7cecc3ee9869a22184edb84cf4a8df93ebcfb56a7a36b2a771819fe5aa8abb9309a2b90f52eefaa70400b80801022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe26742101200740c227eae48be23995067e016aee13f57270f3d8b7d30d48d19a9795f1d4efc4be1e4077a0e57bc0bbc120660fc2655db30c224ec71e55a7e9c2eb187badafae080102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b210120074026957d8f507caea34293ac2895492996ddeacba0f1be0ebf11b5262fa75e2e233742024e97b3abf7f626ec27b47fc4ee68c6fdd4bc0978fca705efdf6389c80e2200012101200741010fe04368eb040b84a52477b08dac77bfa0a8c88845b9c6de509c02673daf712b56c88bcb13523ba264cdf25d00b48771d3619f6fb8d815199a03cf438971ed36"
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "value": "32536bf9beb90dd22207e4a9e6942e285da3ef2ac77e68ecc5c857ea925ebc2d"
}
```
</details>
