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
          "instruction": "CALL_METHOD",
          "component_address": {
            "type": "Address",
            "address": "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
          },
          "method_name": {
            "type": "String",
            "value": "withdraw"
          },
          "arguments": [
            {
              "type": "Address",
              "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
            "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
            "address": "component_sim1pyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqdxh44"
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
            "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
            "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
          }
        },
        {
          "instruction": "TAKE_FROM_WORKTOP",
          "resource_address": {
            "type": "Address",
            "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
            "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
            "address": "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
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
          "address": "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
        },
        "method_name": {
          "type": "String",
          "value": "withdraw"
        },
        "arguments": [
          {
            "type": "Address",
            "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
          "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
          "address": "component_sim1pyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqdxh44"
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
          "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
          "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
        }
      },
      {
        "instruction": "TAKE_FROM_WORKTOP",
        "resource_address": {
          "type": "Address",
          "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
          "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
          "address": "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
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
      "value": "CALL_METHOD\n    Address(\"account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt\")\n    \"withdraw\"\n    Address(\"resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp\")\n    Decimal(\"5\");\nTAKE_FROM_WORKTOP_BY_AMOUNT\n    Decimal(\"2\")\n    Address(\"resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp\")\n    Bucket(\"bucket1\");\nCALL_METHOD\n    Address(\"component_sim1pyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqdxh44\")\n    \"buy_gumball\"\n    Bucket(\"bucket1\");\nASSERT_WORKTOP_CONTAINS_BY_AMOUNT\n    Decimal(\"3\")\n    Address(\"resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp\");\nASSERT_WORKTOP_CONTAINS\n    Address(\"resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp\");\nTAKE_FROM_WORKTOP\n    Address(\"resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp\")\n    Bucket(\"bucket2\");\nRETURN_TO_WORKTOP\n    Bucket(\"bucket2\");\nTAKE_FROM_WORKTOP_BY_IDS\n    Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"))\n    Address(\"resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp\")\n    Bucket(\"bucket3\");\nCALL_METHOD\n    Address(\"account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt\")\n    \"deposit_batch\"\n    Expression(\"ENTIRE_WORKTOP\");\n"
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
    "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt",
    "component_sim1pyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqdxh44"
  ],
  "resource_addresses": [
    "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
  ],
  "account_addresses": [
    "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
  ],
  "accounts_requiring_auth": [
    "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
  ],
  "accounts_withdrawn_from": [
    "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
  ],
  "accounts_deposited_into": [
    "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
  ]
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
          "instruction": "CALL_METHOD",
          "component_address": {
            "type": "Address",
            "address": "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
          },
          "method_name": {
            "type": "String",
            "value": "withdraw"
          },
          "arguments": [
            {
              "type": "Address",
              "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
            "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
            "address": "component_sim1pyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqdxh44"
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
            "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
            "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
          }
        },
        {
          "instruction": "TAKE_FROM_WORKTOP",
          "resource_address": {
            "type": "Address",
            "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
            "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
            "address": "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
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
  "compiled_intent": "4d21022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220220921038007dea02f0a5efd1bd1a4f9d03ba8273267840d3f5f512dfbadfa23968df90c08776974686472617721028002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b0000000000000000000000000000000000000000000000008002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f2103800900000000000000000000000000000000000000000000000000000000000c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a2290000000000000000000000000000000000000000000000008002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f04018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f00018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f0301810100000002022087010100000000000000018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f21038007dea02f0a5efd1bd1a4f9d03ba8273267840d3f5f512dfbadfa23968df90c0d6465706f7369745f626174636821018300202000"
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
  "compiled_intent": "4d21022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220220921038007dea02f0a5efd1bd1a4f9d03ba8273267840d3f5f512dfbadfa23968df90c08776974686472617721028002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b0000000000000000000000000000000000000000000000008002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f2103800900000000000000000000000000000000000000000000000000000000000c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a2290000000000000000000000000000000000000000000000008002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f04018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f00018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f0301810100000002022087010100000000000000018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f21038007dea02f0a5efd1bd1a4f9d03ba8273267840d3f5f512dfbadfa23968df90c0d6465706f7369745f626174636821018300202000"
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
            "address": "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
          },
          "method_name": {
            "type": "String",
            "value": "withdraw"
          },
          "arguments": [
            {
              "type": "Address",
              "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
            "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
            "address": "component_sim1pyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqdxh44"
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
            "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
            "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
          }
        },
        {
          "instruction": "TAKE_FROM_WORKTOP",
          "resource_address": {
            "type": "Address",
            "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
            "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
            "address": "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
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
            "instruction": "CALL_METHOD",
            "component_address": {
              "type": "Address",
              "address": "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
            },
            "method_name": {
              "type": "String",
              "value": "withdraw"
            },
            "arguments": [
              {
                "type": "Address",
                "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
              "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
              "address": "component_sim1pyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqdxh44"
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
              "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
              "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
            }
          },
          {
            "instruction": "TAKE_FROM_WORKTOP",
            "resource_address": {
              "type": "Address",
              "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
              "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
              "address": "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
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
      "signature": "0001262622d14f533bc2a0ad79852080db6dc12ad1e6aee5d96e29d2ee8443d36649dad839ffa7682bf018ade1bdca6cdc65b5050a7a8fcaa78d0d8d3b4c4c6126"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "00762da58eee9bce70ae534c5fffdf0b9e7d480bbe86ec4e54143936010d4e1d241e78598d1db1a76e47519b1482bce849efce6ed5e1b5e626ce2ef70bd58b36d5"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "01b9b898f8cf80c879ccf0b77a1a4ddf2abbd7294ecce10868ca75ec1a067df8081e80afc4d1543325d5e6e7810e908443fccd0a6447782ec228b835756feb7ee2"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
      "signature": "48587e7b41231f7a1740d010038e4f66ad09124f3be2aa94abe528a05bec9d58089a7e24505b7ce027877748e2e3c52a08b15c9b84bb59509ae81212ef375e03"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
      "signature": "29df463d52222cb7a3aba84ab7718451cacf9a827e9eef8aa15d044b79bd2853d9fe2e8bc4b639ec3f479b4729c39cf34e2f526e082d495dfbf2b2c174c3eb0f"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
      "signature": "b10242b96e9218e0921fc85f1ee66098092747d53cefff4a9839f9d2824d59f1ba504a52fd14f9d34a0e908e8501cb8c2d3325fa152bb954746d69b0b9787908"
    }
  ]
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "compiled_intent": "4d210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220220921038007dea02f0a5efd1bd1a4f9d03ba8273267840d3f5f512dfbadfa23968df90c08776974686472617721028002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b0000000000000000000000000000000000000000000000008002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f2103800900000000000000000000000000000000000000000000000000000000000c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a2290000000000000000000000000000000000000000000000008002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f04018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f00018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f0301810100000002022087010100000000000000018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f21038007dea02f0a5efd1bd1a4f9d03ba8273267840d3f5f512dfbadfa23968df90c0d6465706f7369745f626174636821018300202000202206000121012007410001262622d14f533bc2a0ad79852080db6dc12ad1e6aee5d96e29d2ee8443d36649dad839ffa7682bf018ade1bdca6cdc65b5050a7a8fcaa78d0d8d3b4c4c61260001210120074100762da58eee9bce70ae534c5fffdf0b9e7d480bbe86ec4e54143936010d4e1d241e78598d1db1a76e47519b1482bce849efce6ed5e1b5e626ce2ef70bd58b36d50001210120074101b9b898f8cf80c879ccf0b77a1a4ddf2abbd7294ecce10868ca75ec1a067df8081e80afc4d1543325d5e6e7810e908443fccd0a6447782ec228b835756feb7ee201022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29210120074048587e7b41231f7a1740d010038e4f66ad09124f3be2aa94abe528a05bec9d58089a7e24505b7ce027877748e2e3c52a08b15c9b84bb59509ae81212ef375e0301022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674210120074029df463d52222cb7a3aba84ab7718451cacf9a827e9eef8aa15d044b79bd2853d9fe2e8bc4b639ec3f479b4729c39cf34e2f526e082d495dfbf2b2c174c3eb0f0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740b10242b96e9218e0921fc85f1ee66098092747d53cefff4a9839f9d2824d59f1ba504a52fd14f9d34a0e908e8501cb8c2d3325fa152bb954746d69b0b9787908"
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
  "compiled_signed_intent": "4d210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220220921038007dea02f0a5efd1bd1a4f9d03ba8273267840d3f5f512dfbadfa23968df90c08776974686472617721028002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b0000000000000000000000000000000000000000000000008002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f2103800900000000000000000000000000000000000000000000000000000000000c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a2290000000000000000000000000000000000000000000000008002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f04018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f00018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f0301810100000002022087010100000000000000018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f21038007dea02f0a5efd1bd1a4f9d03ba8273267840d3f5f512dfbadfa23968df90c0d6465706f7369745f626174636821018300202000202206000121012007410001262622d14f533bc2a0ad79852080db6dc12ad1e6aee5d96e29d2ee8443d36649dad839ffa7682bf018ade1bdca6cdc65b5050a7a8fcaa78d0d8d3b4c4c61260001210120074100762da58eee9bce70ae534c5fffdf0b9e7d480bbe86ec4e54143936010d4e1d241e78598d1db1a76e47519b1482bce849efce6ed5e1b5e626ce2ef70bd58b36d50001210120074101b9b898f8cf80c879ccf0b77a1a4ddf2abbd7294ecce10868ca75ec1a067df8081e80afc4d1543325d5e6e7810e908443fccd0a6447782ec228b835756feb7ee201022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29210120074048587e7b41231f7a1740d010038e4f66ad09124f3be2aa94abe528a05bec9d58089a7e24505b7ce027877748e2e3c52a08b15c9b84bb59509ae81212ef375e0301022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674210120074029df463d52222cb7a3aba84ab7718451cacf9a827e9eef8aa15d044b79bd2853d9fe2e8bc4b639ec3f479b4729c39cf34e2f526e082d495dfbf2b2c174c3eb0f0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740b10242b96e9218e0921fc85f1ee66098092747d53cefff4a9839f9d2824d59f1ba504a52fd14f9d34a0e908e8501cb8c2d3325fa152bb954746d69b0b9787908"
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
              "address": "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
            },
            "method_name": {
              "type": "String",
              "value": "withdraw"
            },
            "arguments": [
              {
                "type": "Address",
                "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
              "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
              "address": "component_sim1pyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqdxh44"
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
              "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
              "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
            }
          },
          {
            "instruction": "TAKE_FROM_WORKTOP",
            "resource_address": {
              "type": "Address",
              "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
              "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
              "address": "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
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
      "signature": "0001262622d14f533bc2a0ad79852080db6dc12ad1e6aee5d96e29d2ee8443d36649dad839ffa7682bf018ade1bdca6cdc65b5050a7a8fcaa78d0d8d3b4c4c6126"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "00762da58eee9bce70ae534c5fffdf0b9e7d480bbe86ec4e54143936010d4e1d241e78598d1db1a76e47519b1482bce849efce6ed5e1b5e626ce2ef70bd58b36d5"
    },
    {
      "curve": "EcdsaSecp256k1",
      "signature": "01b9b898f8cf80c879ccf0b77a1a4ddf2abbd7294ecce10868ca75ec1a067df8081e80afc4d1543325d5e6e7810e908443fccd0a6447782ec228b835756feb7ee2"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
      "signature": "48587e7b41231f7a1740d010038e4f66ad09124f3be2aa94abe528a05bec9d58089a7e24505b7ce027877748e2e3c52a08b15c9b84bb59509ae81212ef375e03"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
      "signature": "29df463d52222cb7a3aba84ab7718451cacf9a827e9eef8aa15d044b79bd2853d9fe2e8bc4b639ec3f479b4729c39cf34e2f526e082d495dfbf2b2c174c3eb0f"
    },
    {
      "curve": "EddsaEd25519",
      "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
      "signature": "b10242b96e9218e0921fc85f1ee66098092747d53cefff4a9839f9d2824d59f1ba504a52fd14f9d34a0e908e8501cb8c2d3325fa152bb954746d69b0b9787908"
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
              "instruction": "CALL_METHOD",
              "component_address": {
                "type": "Address",
                "address": "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
              },
              "method_name": {
                "type": "String",
                "value": "withdraw"
              },
              "arguments": [
                {
                  "type": "Address",
                  "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
                "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
                "address": "component_sim1pyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqdxh44"
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
                "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
                "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
              }
            },
            {
              "instruction": "TAKE_FROM_WORKTOP",
              "resource_address": {
                "type": "Address",
                "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
                "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
                "address": "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
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
        "signature": "0001262622d14f533bc2a0ad79852080db6dc12ad1e6aee5d96e29d2ee8443d36649dad839ffa7682bf018ade1bdca6cdc65b5050a7a8fcaa78d0d8d3b4c4c6126"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "00762da58eee9bce70ae534c5fffdf0b9e7d480bbe86ec4e54143936010d4e1d241e78598d1db1a76e47519b1482bce849efce6ed5e1b5e626ce2ef70bd58b36d5"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "01b9b898f8cf80c879ccf0b77a1a4ddf2abbd7294ecce10868ca75ec1a067df8081e80afc4d1543325d5e6e7810e908443fccd0a6447782ec228b835756feb7ee2"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
        "signature": "48587e7b41231f7a1740d010038e4f66ad09124f3be2aa94abe528a05bec9d58089a7e24505b7ce027877748e2e3c52a08b15c9b84bb59509ae81212ef375e03"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
        "signature": "29df463d52222cb7a3aba84ab7718451cacf9a827e9eef8aa15d044b79bd2853d9fe2e8bc4b639ec3f479b4729c39cf34e2f526e082d495dfbf2b2c174c3eb0f"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
        "signature": "b10242b96e9218e0921fc85f1ee66098092747d53cefff4a9839f9d2824d59f1ba504a52fd14f9d34a0e908e8501cb8c2d3325fa152bb954746d69b0b9787908"
      }
    ]
  },
  "notary_signature": {
    "curve": "EcdsaSecp256k1",
    "signature": "0118da0031ad49dff51696bbd49d5edf782cfdb79b9f7b51c8cb9b398d0c87caf753c96e4dd53db7bd0f6c059eb5a7eaf77a628515fd6affb819073fefda2e7323"
  }
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "compiled_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220220921038007dea02f0a5efd1bd1a4f9d03ba8273267840d3f5f512dfbadfa23968df90c08776974686472617721028002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b0000000000000000000000000000000000000000000000008002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f2103800900000000000000000000000000000000000000000000000000000000000c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a2290000000000000000000000000000000000000000000000008002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f04018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f00018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f0301810100000002022087010100000000000000018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f21038007dea02f0a5efd1bd1a4f9d03ba8273267840d3f5f512dfbadfa23968df90c0d6465706f7369745f626174636821018300202000202206000121012007410001262622d14f533bc2a0ad79852080db6dc12ad1e6aee5d96e29d2ee8443d36649dad839ffa7682bf018ade1bdca6cdc65b5050a7a8fcaa78d0d8d3b4c4c61260001210120074100762da58eee9bce70ae534c5fffdf0b9e7d480bbe86ec4e54143936010d4e1d241e78598d1db1a76e47519b1482bce849efce6ed5e1b5e626ce2ef70bd58b36d50001210120074101b9b898f8cf80c879ccf0b77a1a4ddf2abbd7294ecce10868ca75ec1a067df8081e80afc4d1543325d5e6e7810e908443fccd0a6447782ec228b835756feb7ee201022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29210120074048587e7b41231f7a1740d010038e4f66ad09124f3be2aa94abe528a05bec9d58089a7e24505b7ce027877748e2e3c52a08b15c9b84bb59509ae81212ef375e0301022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674210120074029df463d52222cb7a3aba84ab7718451cacf9a827e9eef8aa15d044b79bd2853d9fe2e8bc4b639ec3f479b4729c39cf34e2f526e082d495dfbf2b2c174c3eb0f0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740b10242b96e9218e0921fc85f1ee66098092747d53cefff4a9839f9d2824d59f1ba504a52fd14f9d34a0e908e8501cb8c2d3325fa152bb954746d69b0b978790822000121012007410118da0031ad49dff51696bbd49d5edf782cfdb79b9f7b51c8cb9b398d0c87caf753c96e4dd53db7bd0f6c059eb5a7eaf77a628515fd6affb819073fefda2e7323"
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
  "compiled_notarized_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220220921038007dea02f0a5efd1bd1a4f9d03ba8273267840d3f5f512dfbadfa23968df90c08776974686472617721028002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b0000000000000000000000000000000000000000000000008002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f2103800900000000000000000000000000000000000000000000000000000000000c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a2290000000000000000000000000000000000000000000000008002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f04018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f00018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f0301810100000002022087010100000000000000018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f21038007dea02f0a5efd1bd1a4f9d03ba8273267840d3f5f512dfbadfa23968df90c0d6465706f7369745f626174636821018300202000202206000121012007410001262622d14f533bc2a0ad79852080db6dc12ad1e6aee5d96e29d2ee8443d36649dad839ffa7682bf018ade1bdca6cdc65b5050a7a8fcaa78d0d8d3b4c4c61260001210120074100762da58eee9bce70ae534c5fffdf0b9e7d480bbe86ec4e54143936010d4e1d241e78598d1db1a76e47519b1482bce849efce6ed5e1b5e626ce2ef70bd58b36d50001210120074101b9b898f8cf80c879ccf0b77a1a4ddf2abbd7294ecce10868ca75ec1a067df8081e80afc4d1543325d5e6e7810e908443fccd0a6447782ec228b835756feb7ee201022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29210120074048587e7b41231f7a1740d010038e4f66ad09124f3be2aa94abe528a05bec9d58089a7e24505b7ce027877748e2e3c52a08b15c9b84bb59509ae81212ef375e0301022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674210120074029df463d52222cb7a3aba84ab7718451cacf9a827e9eef8aa15d044b79bd2853d9fe2e8bc4b639ec3f479b4729c39cf34e2f526e082d495dfbf2b2c174c3eb0f0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740b10242b96e9218e0921fc85f1ee66098092747d53cefff4a9839f9d2824d59f1ba504a52fd14f9d34a0e908e8501cb8c2d3325fa152bb954746d69b0b978790822000121012007410118da0031ad49dff51696bbd49d5edf782cfdb79b9f7b51c8cb9b398d0c87caf753c96e4dd53db7bd0f6c059eb5a7eaf77a628515fd6affb819073fefda2e7323"
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
                "address": "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
              },
              "method_name": {
                "type": "String",
                "value": "withdraw"
              },
              "arguments": [
                {
                  "type": "Address",
                  "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
                "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
                "address": "component_sim1pyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqdxh44"
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
                "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
                "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
              }
            },
            {
              "instruction": "TAKE_FROM_WORKTOP",
              "resource_address": {
                "type": "Address",
                "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
                "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
                "address": "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
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
        "signature": "0001262622d14f533bc2a0ad79852080db6dc12ad1e6aee5d96e29d2ee8443d36649dad839ffa7682bf018ade1bdca6cdc65b5050a7a8fcaa78d0d8d3b4c4c6126"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "00762da58eee9bce70ae534c5fffdf0b9e7d480bbe86ec4e54143936010d4e1d241e78598d1db1a76e47519b1482bce849efce6ed5e1b5e626ce2ef70bd58b36d5"
      },
      {
        "curve": "EcdsaSecp256k1",
        "signature": "01b9b898f8cf80c879ccf0b77a1a4ddf2abbd7294ecce10868ca75ec1a067df8081e80afc4d1543325d5e6e7810e908443fccd0a6447782ec228b835756feb7ee2"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
        "signature": "48587e7b41231f7a1740d010038e4f66ad09124f3be2aa94abe528a05bec9d58089a7e24505b7ce027877748e2e3c52a08b15c9b84bb59509ae81212ef375e03"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
        "signature": "29df463d52222cb7a3aba84ab7718451cacf9a827e9eef8aa15d044b79bd2853d9fe2e8bc4b639ec3f479b4729c39cf34e2f526e082d495dfbf2b2c174c3eb0f"
      },
      {
        "curve": "EddsaEd25519",
        "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
        "signature": "b10242b96e9218e0921fc85f1ee66098092747d53cefff4a9839f9d2824d59f1ba504a52fd14f9d34a0e908e8501cb8c2d3325fa152bb954746d69b0b9787908"
      }
    ]
  },
  "notary_signature": {
    "curve": "EcdsaSecp256k1",
    "signature": "0118da0031ad49dff51696bbd49d5edf782cfdb79b9f7b51c8cb9b398d0c87caf753c96e4dd53db7bd0f6c059eb5a7eaf77a628515fd6affb819073fefda2e7323"
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
  "compiled_unknown_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220220921038007dea02f0a5efd1bd1a4f9d03ba8273267840d3f5f512dfbadfa23968df90c08776974686472617721028002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b0000000000000000000000000000000000000000000000008002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f2103800900000000000000000000000000000000000000000000000000000000000c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a2290000000000000000000000000000000000000000000000008002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f04018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f00018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f0301810100000002022087010100000000000000018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f21038007dea02f0a5efd1bd1a4f9d03ba8273267840d3f5f512dfbadfa23968df90c0d6465706f7369745f626174636821018300202000202206000121012007410001262622d14f533bc2a0ad79852080db6dc12ad1e6aee5d96e29d2ee8443d36649dad839ffa7682bf018ade1bdca6cdc65b5050a7a8fcaa78d0d8d3b4c4c61260001210120074100762da58eee9bce70ae534c5fffdf0b9e7d480bbe86ec4e54143936010d4e1d241e78598d1db1a76e47519b1482bce849efce6ed5e1b5e626ce2ef70bd58b36d50001210120074101b9b898f8cf80c879ccf0b77a1a4ddf2abbd7294ecce10868ca75ec1a067df8081e80afc4d1543325d5e6e7810e908443fccd0a6447782ec228b835756feb7ee201022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29210120074048587e7b41231f7a1740d010038e4f66ad09124f3be2aa94abe528a05bec9d58089a7e24505b7ce027877748e2e3c52a08b15c9b84bb59509ae81212ef375e0301022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674210120074029df463d52222cb7a3aba84ab7718451cacf9a827e9eef8aa15d044b79bd2853d9fe2e8bc4b639ec3f479b4729c39cf34e2f526e082d495dfbf2b2c174c3eb0f0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740b10242b96e9218e0921fc85f1ee66098092747d53cefff4a9839f9d2824d59f1ba504a52fd14f9d34a0e908e8501cb8c2d3325fa152bb954746d69b0b978790822000121012007410118da0031ad49dff51696bbd49d5edf782cfdb79b9f7b51c8cb9b398d0c87caf753c96e4dd53db7bd0f6c059eb5a7eaf77a628515fd6affb819073fefda2e7323"
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
                  "address": "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
                },
                "method_name": {
                  "type": "String",
                  "value": "withdraw"
                },
                "arguments": [
                  {
                    "type": "Address",
                    "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
                  "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
                  "address": "component_sim1pyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqdxh44"
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
                  "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
                  "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
                }
              },
              {
                "instruction": "TAKE_FROM_WORKTOP",
                "resource_address": {
                  "type": "Address",
                  "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
                  "address": "resource_sim1q2atsr8kvzrkdpqe7h94jp9vleraasdw348gn8zg9g6n6g50t6hwlp"
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
                  "address": "account_sim1ql02qtc2tm73h5dyl8grh2p8xfncgrfltagjm7adlg3edr0ejjmpvt"
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
          "signature": "0001262622d14f533bc2a0ad79852080db6dc12ad1e6aee5d96e29d2ee8443d36649dad839ffa7682bf018ade1bdca6cdc65b5050a7a8fcaa78d0d8d3b4c4c6126"
        },
        {
          "curve": "EcdsaSecp256k1",
          "signature": "00762da58eee9bce70ae534c5fffdf0b9e7d480bbe86ec4e54143936010d4e1d241e78598d1db1a76e47519b1482bce849efce6ed5e1b5e626ce2ef70bd58b36d5"
        },
        {
          "curve": "EcdsaSecp256k1",
          "signature": "01b9b898f8cf80c879ccf0b77a1a4ddf2abbd7294ecce10868ca75ec1a067df8081e80afc4d1543325d5e6e7810e908443fccd0a6447782ec228b835756feb7ee2"
        },
        {
          "curve": "EddsaEd25519",
          "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
          "signature": "48587e7b41231f7a1740d010038e4f66ad09124f3be2aa94abe528a05bec9d58089a7e24505b7ce027877748e2e3c52a08b15c9b84bb59509ae81212ef375e03"
        },
        {
          "curve": "EddsaEd25519",
          "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
          "signature": "29df463d52222cb7a3aba84ab7718451cacf9a827e9eef8aa15d044b79bd2853d9fe2e8bc4b639ec3f479b4729c39cf34e2f526e082d495dfbf2b2c174c3eb0f"
        },
        {
          "curve": "EddsaEd25519",
          "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
          "signature": "b10242b96e9218e0921fc85f1ee66098092747d53cefff4a9839f9d2824d59f1ba504a52fd14f9d34a0e908e8501cb8c2d3325fa152bb954746d69b0b9787908"
        }
      ]
    },
    "notary_signature": {
      "curve": "EcdsaSecp256k1",
      "signature": "0118da0031ad49dff51696bbd49d5edf782cfdb79b9f7b51c8cb9b398d0c87caf753c96e4dd53db7bd0f6c059eb5a7eaf77a628515fd6affb819073fefda2e7323"
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
| Functionality     | This function can be used to decode a Bech32m encoded address string into its</br>equivalent hrp and data. In addition to that, this function provides other useful information on</br>the address such as the network id and name that it is used for, and the entity type of the</br>address.   |
| Required Features | default   |

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
  "data": "010000000000000000000000000000000000000000000000000000000000"
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
| Functionality     | This function takes in a hex string and attempts to decode it into a</br>ScryptoSborValue.   |
| Required Features | default   |

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
  "compiled_notarized_intent": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220220921038007dea02f0a5efd1bd1a4f9d03ba8273267840d3f5f512dfbadfa23968df90c08776974686472617721028002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b0000000000000000000000000000000000000000000000008002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f2103800900000000000000000000000000000000000000000000000000000000000c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a2290000000000000000000000000000000000000000000000008002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f04018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f00018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f0301810100000002022087010100000000000000018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f21038007dea02f0a5efd1bd1a4f9d03ba8273267840d3f5f512dfbadfa23968df90c0d6465706f7369745f626174636821018300202000202206000121012007410001262622d14f533bc2a0ad79852080db6dc12ad1e6aee5d96e29d2ee8443d36649dad839ffa7682bf018ade1bdca6cdc65b5050a7a8fcaa78d0d8d3b4c4c61260001210120074100762da58eee9bce70ae534c5fffdf0b9e7d480bbe86ec4e54143936010d4e1d241e78598d1db1a76e47519b1482bce849efce6ed5e1b5e626ce2ef70bd58b36d50001210120074101b9b898f8cf80c879ccf0b77a1a4ddf2abbd7294ecce10868ca75ec1a067df8081e80afc4d1543325d5e6e7810e908443fccd0a6447782ec228b835756feb7ee201022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29210120074048587e7b41231f7a1740d010038e4f66ad09124f3be2aa94abe528a05bec9d58089a7e24505b7ce027877748e2e3c52a08b15c9b84bb59509ae81212ef375e0301022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674210120074029df463d52222cb7a3aba84ab7718451cacf9a827e9eef8aa15d044b79bd2853d9fe2e8bc4b639ec3f479b4729c39cf34e2f526e082d495dfbf2b2c174c3eb0f0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740b10242b96e9218e0921fc85f1ee66098092747d53cefff4a9839f9d2824d59f1ba504a52fd14f9d34a0e908e8501cb8c2d3325fa152bb954746d69b0b978790822000121012007410001262622d14f533bc2a0ad79852080db6dc12ad1e6aee5d96e29d2ee8443d36649dad839ffa7682bf018ade1bdca6cdc65b5050a7a8fcaa78d0d8d3b4c4c6126",
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
| Functionality     | Hashes some payload through the hashing algorithm used in Scrypto and the Radix</br>Engine.   |
| Required Features | default   |

<details>
    <summary>Request Example</summary>

```json
{
  "payload": "4d2102210221022109070107f20a00020000000000000a10020000000000000a220000000000000022000120072103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505080000210220220921038007dea02f0a5efd1bd1a4f9d03ba8273267840d3f5f512dfbadfa23968df90c08776974686472617721028002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f850000f444829163450000000000000000000000000000000000000000000000000102850000c84e676dc11b0000000000000000000000000000000000000000000000008002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f2103800900000000000000000000000000000000000000000000000000000000000c0b6275795f67756d62616c6c2101810000000005028500002cf61a24a2290000000000000000000000000000000000000000000000008002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f04018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f00018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f0301810100000002022087010100000000000000018002bab80cf66087668419f5cb5904acfe47dec1ae8d4e899c482a353d228f21038007dea02f0a5efd1bd1a4f9d03ba8273267840d3f5f512dfbadfa23968df90c0d6465706f7369745f626174636821018300202000202206000121012007410001262622d14f533bc2a0ad79852080db6dc12ad1e6aee5d96e29d2ee8443d36649dad839ffa7682bf018ade1bdca6cdc65b5050a7a8fcaa78d0d8d3b4c4c61260001210120074100762da58eee9bce70ae534c5fffdf0b9e7d480bbe86ec4e54143936010d4e1d241e78598d1db1a76e47519b1482bce849efce6ed5e1b5e626ce2ef70bd58b36d50001210120074101b9b898f8cf80c879ccf0b77a1a4ddf2abbd7294ecce10868ca75ec1a067df8081e80afc4d1543325d5e6e7810e908443fccd0a6447782ec228b835756feb7ee201022007204cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29210120074048587e7b41231f7a1740d010038e4f66ad09124f3be2aa94abe528a05bec9d58089a7e24505b7ce027877748e2e3c52a08b15c9b84bb59509ae81212ef375e0301022007207422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674210120074029df463d52222cb7a3aba84ab7718451cacf9a827e9eef8aa15d044b79bd2853d9fe2e8bc4b639ec3f479b4729c39cf34e2f526e082d495dfbf2b2c174c3eb0f0102200720f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b2101200740b10242b96e9218e0921fc85f1ee66098092747d53cefff4a9839f9d2824d59f1ba504a52fd14f9d34a0e908e8501cb8c2d3325fa152bb954746d69b0b978790822000121012007410001262622d14f533bc2a0ad79852080db6dc12ad1e6aee5d96e29d2ee8443d36649dad839ffa7682bf018ade1bdca6cdc65b5050a7a8fcaa78d0d8d3b4c4c6126"
}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{
  "value": "71d77b068a408d49747eb2f44216a7fa4dc953c6d61b1d41adbb93ad3f4a7baa"
}
```
</details>
