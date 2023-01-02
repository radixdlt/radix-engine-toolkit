# Requests Examples

This document contains examples and descriptions of the different requests and responses which the Radix Engine Toolkit may provide. As long as all of the CI test pass, then you may treat this document as the canonical truth for the format of the different requests and as valid examples of the payload and responses of these requests.


## Information

| Function Name     | `information` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_information` |
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
  "package_version": "0.1.0"
}
```
</details>

## Convert Manifest

| Function Name     | `convert_manifest` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_convertManifest` |
| Functionality     | Clients have a need to be able to read, parse, understand, and interrogate transaction manifests to get more information on what a transactions might be doing. Transaction manifests have so far existed in one format: as strings. While the string format is very human readable, it is not easily readable by machines as a lexer and parser are needed to make sense of them; thus, it is for clients to programmatically make sense of transactions. As such, there is a need for another transaction manifest format (to supplement, NOT replace) which machines can easily make sense of without the need to implement a lexer and parser.</br></br>Therefore, this library introduces a JSON format for transaction manifests which clients can use when wanting to read and interrogate their transaction manifests in code. The transaction manifest JSON format has a 1:1 mapping to the string format of transaction manifests, meaning that anything which can be done in the string format of transaction manifests, can be done in the JSON format as well.</br></br>This function allows the client the convert their manifest between the two supported manifest types: string and JSON. |
| Request Type      | `ConvertManifestRequest` |
| Response Type     | `ConvertManifestResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "transaction_version": 1,
  "network_id": 242,
  "manifest_instructions_output_format": "JSON",
  "manifest": {
    "instructions": {
      "type": "String",
      "value": "CALL_METHOD ComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\") \"withdraw_by_amount\" Decimal(\"5\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nTAKE_FROM_WORKTOP_BY_AMOUNT Decimal(\"2\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\") Bucket(\"bucket1\");\nCALL_METHOD ComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\") \"buy_gumball\" Bucket(\"bucket1\");\nASSERT_WORKTOP_CONTAINS_BY_AMOUNT Decimal(\"3\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nASSERT_WORKTOP_CONTAINS ResourceAddress(\"resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6\");\nTAKE_FROM_WORKTOP ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\") Bucket(\"bucket2\");\nCREATE_PROOF_FROM_BUCKET Bucket(\"bucket2\") Proof(\"proof1\");\nCLONE_PROOF Proof(\"proof1\") Proof(\"proof2\");\nDROP_PROOF Proof(\"proof1\");\nDROP_PROOF Proof(\"proof2\");\nCALL_METHOD ComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\") \"create_proof_by_amount\" Decimal(\"5\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nPOP_FROM_AUTH_ZONE Proof(\"proof3\");\nDROP_PROOF Proof(\"proof3\");\nRETURN_TO_WORKTOP Bucket(\"bucket2\");\nTAKE_FROM_WORKTOP_BY_IDS Array<NonFungibleId>(NonFungibleId(Bytes(\"031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f\"))) ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\") Bucket(\"bucket3\");\nCREATE_RESOURCE Enum(\"Fungible\", 0u8) Array<Tuple>() Array<Tuple>() Some(Enum(\"Fungible\", Decimal(\"1\")));\nCALL_METHOD ComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");\nDROP_ALL_PROOFS;\nCALL_METHOD ComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\") \"complicated_method\" Decimal(\"1\") PreciseDecimal(\"2\");\nPUBLISH_PACKAGE_WITH_OWNER Blob(\"36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618\") Blob(\"15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d\") NonFungibleAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\", Bytes(\"031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f\"));\n"
    },
    "blobs": [
      "10020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c1000000000",
      "320c1000000000"
    ]
  }
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "instructions": {
    "type": "JSON",
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
        "amount": {
          "type": "Decimal",
          "value": "2"
        },
        "resource_address": {
          "type": "ResourceAddress",
          "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
        },
        "into_bucket": {
          "type": "Bucket",
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
            "identifier": "bucket1"
          }
        ]
      },
      {
        "instruction": "ASSERT_WORKTOP_CONTAINS_BY_AMOUNT",
        "amount": {
          "type": "Decimal",
          "value": "3"
        },
        "resource_address": {
          "type": "ResourceAddress",
          "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
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
          "identifier": "bucket2"
        }
      },
      {
        "instruction": "CREATE_PROOF_FROM_BUCKET",
        "bucket": {
          "type": "Bucket",
          "identifier": "bucket2"
        },
        "into_proof": {
          "type": "Proof",
          "identifier": "proof1"
        }
      },
      {
        "instruction": "CLONE_PROOF",
        "proof": {
          "type": "Proof",
          "identifier": "proof1"
        },
        "into_proof": {
          "type": "Proof",
          "identifier": "proof2"
        }
      },
      {
        "instruction": "DROP_PROOF",
        "proof": {
          "type": "Proof",
          "identifier": "proof1"
        }
      },
      {
        "instruction": "DROP_PROOF",
        "proof": {
          "type": "Proof",
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
          "identifier": "proof3"
        }
      },
      {
        "instruction": "DROP_PROOF",
        "proof": {
          "type": "Proof",
          "identifier": "proof3"
        }
      },
      {
        "instruction": "RETURN_TO_WORKTOP",
        "bucket": {
          "type": "Bucket",
          "identifier": "bucket2"
        }
      },
      {
        "instruction": "TAKE_FROM_WORKTOP_BY_IDS",
        "ids": [
          {
            "type": "NonFungibleId",
            "variant": "Bytes",
            "value": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
          }
        ],
        "resource_address": {
          "type": "ResourceAddress",
          "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
        },
        "into_bucket": {
          "type": "Bucket",
          "identifier": "bucket3"
        }
      },
      {
        "instruction": "CREATE_RESOURCE",
        "resource_type": {
          "type": "Enum",
          "variant": "Fungible",
          "fields": [
            {
              "type": "U8",
              "value": "0"
            }
          ]
        },
        "metadata": {
          "type": "Array",
          "element_type": "Tuple",
          "elements": []
        },
        "access_rules": {
          "type": "Array",
          "element_type": "Tuple",
          "elements": []
        },
        "mint_params": {
          "type": "Option",
          "variant": "Some",
          "field": {
            "type": "Enum",
            "variant": "Fungible",
            "fields": [
              {
                "type": "Decimal",
                "value": "1"
              }
            ]
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
          "value": "deposit_batch"
        },
        "arguments": [
          {
            "type": "Expression",
            "value": "ENTIRE_WORKTOP"
          }
        ]
      },
      {
        "instruction": "DROP_ALL_PROOFS"
      },
      {
        "instruction": "CALL_METHOD",
        "component_address": {
          "type": "ComponentAddress",
          "address": "component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum"
        },
        "method_name": {
          "type": "String",
          "value": "complicated_method"
        },
        "arguments": [
          {
            "type": "Decimal",
            "value": "1"
          },
          {
            "type": "PreciseDecimal",
            "value": "2"
          }
        ]
      },
      {
        "instruction": "PUBLISH_PACKAGE_WITH_OWNER",
        "code": {
          "type": "Blob",
          "hash": "36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618"
        },
        "abi": {
          "type": "Blob",
          "hash": "15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d"
        },
        "owner_badge": {
          "type": "NonFungibleAddress",
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          },
          "non_fungible_id": {
            "type": "NonFungibleId",
            "variant": "Bytes",
            "value": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
          }
        }
      }
    ]
  },
  "blobs": [
    "10020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c1000000000",
    "320c1000000000"
  ]
}
```
</details>

## Compile Transaction Intent

| Function Name     | `compile_transaction_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_compileTransactionIntent` |
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
      "type": "EcdsaSecp256k1",
      "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
    },
    "notary_as_signatory": true,
    "cost_unit_limit": "100000000",
    "tip_percentage": "0"
  },
  "manifest": {
    "instructions": {
      "type": "String",
      "value": "CALL_METHOD ComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\") \"withdraw_by_amount\" Decimal(\"5\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nTAKE_FROM_WORKTOP_BY_AMOUNT Decimal(\"2\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\") Bucket(\"bucket1\");\nCALL_METHOD ComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\") \"buy_gumball\" Bucket(\"bucket1\");\nASSERT_WORKTOP_CONTAINS_BY_AMOUNT Decimal(\"3\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nASSERT_WORKTOP_CONTAINS ResourceAddress(\"resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6\");\nTAKE_FROM_WORKTOP ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\") Bucket(\"bucket2\");\nCREATE_PROOF_FROM_BUCKET Bucket(\"bucket2\") Proof(\"proof1\");\nCLONE_PROOF Proof(\"proof1\") Proof(\"proof2\");\nDROP_PROOF Proof(\"proof1\");\nDROP_PROOF Proof(\"proof2\");\nCALL_METHOD ComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\") \"create_proof_by_amount\" Decimal(\"5\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nPOP_FROM_AUTH_ZONE Proof(\"proof3\");\nDROP_PROOF Proof(\"proof3\");\nRETURN_TO_WORKTOP Bucket(\"bucket2\");\nTAKE_FROM_WORKTOP_BY_IDS Array<NonFungibleId>(NonFungibleId(Bytes(\"031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f\"))) ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\") Bucket(\"bucket3\");\nCREATE_RESOURCE Enum(\"Fungible\", 0u8) Array<Tuple>() Array<Tuple>() Some(Enum(\"Fungible\", Decimal(\"1\")));\nCALL_METHOD ComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");\nDROP_ALL_PROOFS;\nCALL_METHOD ComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\") \"complicated_method\" Decimal(\"1\") PreciseDecimal(\"2\");\nPUBLISH_PACKAGE_WITH_OWNER Blob(\"36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618\") Blob(\"15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d\") NonFungibleAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\", Bytes(\"031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f\"));\n"
    },
    "blobs": [
      "10020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c1000000000",
      "320c1000000000"
    ]
  }
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "compiled_intent": "5c21022109070107f20a00020000000000000a10020000000000000a2200000000000000110e4563647361536563703235366b3101b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505070021022011140a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000041754616b6546726f6d576f726b746f704279416d6f756e7402b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040a43616c6c4d6574686f640221021106476c6f62616c01810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c210192000200001d417373657274576f726b746f70436f6e7461696e734279416d6f756e7402b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000415417373657274576f726b746f70436f6e7461696e73018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510f54616b6546726f6d576f726b746f7001820000000000000000000000000000000000000000000000000000041543726561746550726f6f6646726f6d4275636b65740109010200000a436c6f6e6550726f6f660109020200000944726f7050726f6f660109020200000944726f7050726f6f660109030200000a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040f506f7046726f6d417574685a6f6e65000944726f7050726f6f660109040200000f52657475726e546f576f726b746f700109010200001454616b6546726f6d576f726b746f7042794964730220b701255c200721031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f820000000000000000000000000000000000000000000000000000041243616c6c4e617469766546756e6374696f6e0221020c0f5265736f757263654d616e616765720c066372656174652007495c2104110846756e6769626c650107002021002021001104536f6d6501110846756e6769626c6501b5000064a7b3b6e00d0000000000000000000000000000000000000000000000000a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007135c2101a00e454e544952455f574f524b544f500d44726f70416c6c50726f6f6673000a43616c6c4d6574686f640221021106476c6f62616c01810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c12636f6d706c6963617465645f6d6574686f642007655c2102b5000064a7b3b6e00d000000000000000000000000000000000000000000000000b60000000000000000023ed47ec9da71dcda2f4fb5e9f37fd2079e3000000000000000000000000000000000000000000000000000000000000000000000000000175075626c6973685061636b616765576974684f776e657203a136dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618a115e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5da2400000000000000000000000000000000000000000000000000000045c200721031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f20200207d20110020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c10000000000707320c1000000000"
}
```
</details>

## Decompile Transaction Intent

| Function Name     | `decompile_transaction_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_decompileTransactionIntent` |
| Functionality     | This function does the opposite of the compile_transaction_intent function. It takes in a compiled transaction intent and decompiles it into its human-readable / machine-readable format. |
| Request Type      | `DecompileTransactionIntentRequest` |
| Response Type     | `DecompileTransactionIntentResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "manifest_instructions_output_format": "JSON",
  "compiled_intent": "5c21022109070107f20a00020000000000000a10020000000000000a2200000000000000110e4563647361536563703235366b3101b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505070021022011140a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000041754616b6546726f6d576f726b746f704279416d6f756e7402b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040a43616c6c4d6574686f640221021106476c6f62616c01810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c210192000200001d417373657274576f726b746f70436f6e7461696e734279416d6f756e7402b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000415417373657274576f726b746f70436f6e7461696e73018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510f54616b6546726f6d576f726b746f7001820000000000000000000000000000000000000000000000000000041543726561746550726f6f6646726f6d4275636b65740109010200000a436c6f6e6550726f6f660109020200000944726f7050726f6f660109020200000944726f7050726f6f660109030200000a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040f506f7046726f6d417574685a6f6e65000944726f7050726f6f660109040200000f52657475726e546f576f726b746f700109010200001454616b6546726f6d576f726b746f7042794964730220b701255c200721031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f820000000000000000000000000000000000000000000000000000041243616c6c4e617469766546756e6374696f6e0221020c0f5265736f757263654d616e616765720c066372656174652007495c2104110846756e6769626c650107002021002021001104536f6d6501110846756e6769626c6501b5000064a7b3b6e00d0000000000000000000000000000000000000000000000000a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007135c2101a00e454e544952455f574f524b544f500d44726f70416c6c50726f6f6673000a43616c6c4d6574686f640221021106476c6f62616c01810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c12636f6d706c6963617465645f6d6574686f642007655c2102b5000064a7b3b6e00d000000000000000000000000000000000000000000000000b60000000000000000023ed47ec9da71dcda2f4fb5e9f37fd2079e3000000000000000000000000000000000000000000000000000000000000000000000000000175075626c6973685061636b616765576974684f776e657203a136dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618a115e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5da2400000000000000000000000000000000000000000000000000000045c200721031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f20200207d20110020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c10000000000707320c1000000000"
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
      "type": "EcdsaSecp256k1",
      "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
    },
    "notary_as_signatory": true,
    "cost_unit_limit": "100000000",
    "tip_percentage": "0"
  },
  "manifest": {
    "instructions": {
      "type": "JSON",
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
          "amount": {
            "type": "Decimal",
            "value": "2"
          },
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          },
          "into_bucket": {
            "type": "Bucket",
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
              "identifier": "bucket1"
            }
          ]
        },
        {
          "instruction": "ASSERT_WORKTOP_CONTAINS_BY_AMOUNT",
          "amount": {
            "type": "Decimal",
            "value": "3"
          },
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
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
            "identifier": "bucket2"
          }
        },
        {
          "instruction": "CREATE_PROOF_FROM_BUCKET",
          "bucket": {
            "type": "Bucket",
            "identifier": "bucket2"
          },
          "into_proof": {
            "type": "Proof",
            "identifier": "proof1"
          }
        },
        {
          "instruction": "CLONE_PROOF",
          "proof": {
            "type": "Proof",
            "identifier": "proof1"
          },
          "into_proof": {
            "type": "Proof",
            "identifier": "proof2"
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "identifier": "proof1"
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
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
            "identifier": "proof3"
          }
        },
        {
          "instruction": "DROP_PROOF",
          "proof": {
            "type": "Proof",
            "identifier": "proof3"
          }
        },
        {
          "instruction": "RETURN_TO_WORKTOP",
          "bucket": {
            "type": "Bucket",
            "identifier": "bucket2"
          }
        },
        {
          "instruction": "TAKE_FROM_WORKTOP_BY_IDS",
          "ids": [
            {
              "type": "NonFungibleId",
              "variant": "Bytes",
              "value": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
            }
          ],
          "resource_address": {
            "type": "ResourceAddress",
            "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
          },
          "into_bucket": {
            "type": "Bucket",
            "identifier": "bucket3"
          }
        },
        {
          "instruction": "CREATE_RESOURCE",
          "resource_type": {
            "type": "Enum",
            "variant": "Fungible",
            "fields": [
              {
                "type": "U8",
                "value": "0"
              }
            ]
          },
          "metadata": {
            "type": "Array",
            "element_type": "Tuple",
            "elements": []
          },
          "access_rules": {
            "type": "Array",
            "element_type": "Tuple",
            "elements": []
          },
          "mint_params": {
            "type": "Option",
            "variant": "Some",
            "field": {
              "type": "Enum",
              "variant": "Fungible",
              "fields": [
                {
                  "type": "Decimal",
                  "value": "1"
                }
              ]
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
            "value": "deposit_batch"
          },
          "arguments": [
            {
              "type": "Expression",
              "value": "ENTIRE_WORKTOP"
            }
          ]
        },
        {
          "instruction": "DROP_ALL_PROOFS"
        },
        {
          "instruction": "CALL_METHOD",
          "component_address": {
            "type": "ComponentAddress",
            "address": "component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum"
          },
          "method_name": {
            "type": "String",
            "value": "complicated_method"
          },
          "arguments": [
            {
              "type": "Decimal",
              "value": "1"
            },
            {
              "type": "PreciseDecimal",
              "value": "2"
            }
          ]
        },
        {
          "instruction": "PUBLISH_PACKAGE_WITH_OWNER",
          "code": {
            "type": "Blob",
            "hash": "36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618"
          },
          "abi": {
            "type": "Blob",
            "hash": "15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d"
          },
          "owner_badge": {
            "type": "NonFungibleAddress",
            "resource_address": {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            },
            "non_fungible_id": {
              "type": "NonFungibleId",
              "variant": "Bytes",
              "value": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
            }
          }
        }
      ]
    },
    "blobs": [
      "10020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c1000000000",
      "320c1000000000"
    ]
  }
}
```
</details>

## Compile Signed Transaction Intent

| Function Name     | `compile_signed_transaction_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_compileSignedTransactionIntent` |
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
        "type": "EcdsaSecp256k1",
        "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
      },
      "notary_as_signatory": true,
      "cost_unit_limit": "100000000",
      "tip_percentage": "0"
    },
    "manifest": {
      "instructions": {
        "type": "String",
        "value": "CALL_METHOD ComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\") \"withdraw_by_amount\" Decimal(\"5\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nTAKE_FROM_WORKTOP_BY_AMOUNT Decimal(\"2\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\") Bucket(\"bucket1\");\nCALL_METHOD ComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\") \"buy_gumball\" Bucket(\"bucket1\");\nASSERT_WORKTOP_CONTAINS_BY_AMOUNT Decimal(\"3\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nASSERT_WORKTOP_CONTAINS ResourceAddress(\"resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6\");\nTAKE_FROM_WORKTOP ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\") Bucket(\"bucket2\");\nCREATE_PROOF_FROM_BUCKET Bucket(\"bucket2\") Proof(\"proof1\");\nCLONE_PROOF Proof(\"proof1\") Proof(\"proof2\");\nDROP_PROOF Proof(\"proof1\");\nDROP_PROOF Proof(\"proof2\");\nCALL_METHOD ComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\") \"create_proof_by_amount\" Decimal(\"5\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nPOP_FROM_AUTH_ZONE Proof(\"proof3\");\nDROP_PROOF Proof(\"proof3\");\nRETURN_TO_WORKTOP Bucket(\"bucket2\");\nTAKE_FROM_WORKTOP_BY_IDS Array<NonFungibleId>(NonFungibleId(Bytes(\"031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f\"))) ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\") Bucket(\"bucket3\");\nCREATE_RESOURCE Enum(\"Fungible\", 0u8) Array<Tuple>() Array<Tuple>() Some(Enum(\"Fungible\", Decimal(\"1\")));\nCALL_METHOD ComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");\nDROP_ALL_PROOFS;\nCALL_METHOD ComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\") \"complicated_method\" Decimal(\"1\") PreciseDecimal(\"2\");\nPUBLISH_PACKAGE_WITH_OWNER Blob(\"36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618\") Blob(\"15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d\") NonFungibleAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\", Bytes(\"031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f\"));\n"
      },
      "blobs": [
        "10020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c1000000000",
        "320c1000000000"
      ]
    }
  },
  "intent_signatures": [
    {
      "type": "EcdsaSecp256k1",
      "signature": "01f74e7ec78fc77d1d7bae5ca9c3aa3596680d5ac883f188742bc29ef335b3e7de4902806c811ccad3c75230bbd22a1645aa3825c492c3e834ecbbab3eb966209f"
    },
    {
      "type": "EcdsaSecp256k1",
      "signature": "019db1de979763680b1d82fb2bc377c2d22aae44b68758acd1add4b3fe2a703788369562549f3ac2074777d086ed2cb70154d45a5ed004fcd015b9e5613415775f"
    },
    {
      "type": "EcdsaSecp256k1",
      "signature": "018a49c93437ad33f2cc169d30c01c3a5bcc09c800a221922f5d7e1e16f9f9c64e33bebe61d07373d15ed48e5193d2d1cbbbab5e89f80e0bdaf8fc57f08700ddff"
    },
    {
      "type": "EddsaEd25519",
      "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
      "signature": "afa51548f560636d80b8d16fba359cd3a2d0d214584adc137e53e9570c28deb4982a2a255912f7062ef4746965f196637294a119e43f5715e8c77bcaf8554a01"
    },
    {
      "type": "EddsaEd25519",
      "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
      "signature": "6e1bdd9ae346c4c44c5b38e797d07c31003b59fabffa1c5dfcf150bb2d3d4ea7f225401928c855261617dc49ef0fba13097b96e463a6a78976d0c95067e8830a"
    },
    {
      "type": "EddsaEd25519",
      "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
      "signature": "089d5005906640a2f56348deb8442be2e1ce344267c11f089dff08c96fdeb71a06fdb456cdd25b2a8eb0a6bc0f0d9e7948dcbf842a74ddced6af54be26035705"
    }
  ]
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "compiled_signed_intent": "5c210221022109070107f20a00020000000000000a10020000000000000a2200000000000000110e4563647361536563703235366b3101b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505070021022011140a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000041754616b6546726f6d576f726b746f704279416d6f756e7402b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040a43616c6c4d6574686f640221021106476c6f62616c01810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c210192000200001d417373657274576f726b746f70436f6e7461696e734279416d6f756e7402b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000415417373657274576f726b746f70436f6e7461696e73018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510f54616b6546726f6d576f726b746f7001820000000000000000000000000000000000000000000000000000041543726561746550726f6f6646726f6d4275636b65740109010200000a436c6f6e6550726f6f660109020200000944726f7050726f6f660109020200000944726f7050726f6f660109030200000a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040f506f7046726f6d417574685a6f6e65000944726f7050726f6f660109040200000f52657475726e546f576f726b746f700109010200001454616b6546726f6d576f726b746f7042794964730220b701255c200721031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f820000000000000000000000000000000000000000000000000000041243616c6c4e617469766546756e6374696f6e0221020c0f5265736f757263654d616e616765720c066372656174652007495c2104110846756e6769626c650107002021002021001104536f6d6501110846756e6769626c6501b5000064a7b3b6e00d0000000000000000000000000000000000000000000000000a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007135c2101a00e454e544952455f574f524b544f500d44726f70416c6c50726f6f6673000a43616c6c4d6574686f640221021106476c6f62616c01810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c12636f6d706c6963617465645f6d6574686f642007655c2102b5000064a7b3b6e00d000000000000000000000000000000000000000000000000b60000000000000000023ed47ec9da71dcda2f4fb5e9f37fd2079e3000000000000000000000000000000000000000000000000000000000000000000000000000175075626c6973685061636b616765576974684f776e657203a136dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618a115e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5da2400000000000000000000000000000000000000000000000000000045c200721031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f20200207d20110020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c10000000000707320c10000000002011060e4563647361536563703235366b3101b201f74e7ec78fc77d1d7bae5ca9c3aa3596680d5ac883f188742bc29ef335b3e7de4902806c811ccad3c75230bbd22a1645aa3825c492c3e834ecbbab3eb966209f0e4563647361536563703235366b3101b2019db1de979763680b1d82fb2bc377c2d22aae44b68758acd1add4b3fe2a703788369562549f3ac2074777d086ed2cb70154d45a5ed004fcd015b9e5613415775f0e4563647361536563703235366b3101b2018a49c93437ad33f2cc169d30c01c3a5bcc09c800a221922f5d7e1e16f9f9c64e33bebe61d07373d15ed48e5193d2d1cbbbab5e89f80e0bdaf8fc57f08700ddff0c45646473614564323535313902b34cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29b4afa51548f560636d80b8d16fba359cd3a2d0d214584adc137e53e9570c28deb4982a2a255912f7062ef4746965f196637294a119e43f5715e8c77bcaf8554a010c45646473614564323535313902b37422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674b46e1bdd9ae346c4c44c5b38e797d07c31003b59fabffa1c5dfcf150bb2d3d4ea7f225401928c855261617dc49ef0fba13097b96e463a6a78976d0c95067e8830a0c45646473614564323535313902b3f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54bb4089d5005906640a2f56348deb8442be2e1ce344267c11f089dff08c96fdeb71a06fdb456cdd25b2a8eb0a6bc0f0d9e7948dcbf842a74ddced6af54be26035705"
}
```
</details>

## Decompile Signed Transaction Intent

| Function Name     | `decompile_signed_transaction_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_decompileSignedTransactionIntent` |
| Functionality     | This function does the opposite of the compile_signed_transaction_intent function. This function takes in a compiled signed transaction intent and decompiles it into its transaction intent and signatures. |
| Request Type      | `DecompileSignedTransactionIntentRequest` |
| Response Type     | `DecompileSignedTransactionIntentResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "manifest_instructions_output_format": "JSON",
  "compiled_signed_intent": "5c210221022109070107f20a00020000000000000a10020000000000000a2200000000000000110e4563647361536563703235366b3101b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505070021022011140a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000041754616b6546726f6d576f726b746f704279416d6f756e7402b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040a43616c6c4d6574686f640221021106476c6f62616c01810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c210192000200001d417373657274576f726b746f70436f6e7461696e734279416d6f756e7402b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000415417373657274576f726b746f70436f6e7461696e73018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510f54616b6546726f6d576f726b746f7001820000000000000000000000000000000000000000000000000000041543726561746550726f6f6646726f6d4275636b65740109010200000a436c6f6e6550726f6f660109020200000944726f7050726f6f660109020200000944726f7050726f6f660109030200000a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040f506f7046726f6d417574685a6f6e65000944726f7050726f6f660109040200000f52657475726e546f576f726b746f700109010200001454616b6546726f6d576f726b746f7042794964730220b701255c200721031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f820000000000000000000000000000000000000000000000000000041243616c6c4e617469766546756e6374696f6e0221020c0f5265736f757263654d616e616765720c066372656174652007495c2104110846756e6769626c650107002021002021001104536f6d6501110846756e6769626c6501b5000064a7b3b6e00d0000000000000000000000000000000000000000000000000a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007135c2101a00e454e544952455f574f524b544f500d44726f70416c6c50726f6f6673000a43616c6c4d6574686f640221021106476c6f62616c01810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c12636f6d706c6963617465645f6d6574686f642007655c2102b5000064a7b3b6e00d000000000000000000000000000000000000000000000000b60000000000000000023ed47ec9da71dcda2f4fb5e9f37fd2079e3000000000000000000000000000000000000000000000000000000000000000000000000000175075626c6973685061636b616765576974684f776e657203a136dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618a115e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5da2400000000000000000000000000000000000000000000000000000045c200721031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f20200207d20110020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c10000000000707320c10000000002011060e4563647361536563703235366b3101b201f74e7ec78fc77d1d7bae5ca9c3aa3596680d5ac883f188742bc29ef335b3e7de4902806c811ccad3c75230bbd22a1645aa3825c492c3e834ecbbab3eb966209f0e4563647361536563703235366b3101b2019db1de979763680b1d82fb2bc377c2d22aae44b68758acd1add4b3fe2a703788369562549f3ac2074777d086ed2cb70154d45a5ed004fcd015b9e5613415775f0e4563647361536563703235366b3101b2018a49c93437ad33f2cc169d30c01c3a5bcc09c800a221922f5d7e1e16f9f9c64e33bebe61d07373d15ed48e5193d2d1cbbbab5e89f80e0bdaf8fc57f08700ddff0c45646473614564323535313902b34cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29b4afa51548f560636d80b8d16fba359cd3a2d0d214584adc137e53e9570c28deb4982a2a255912f7062ef4746965f196637294a119e43f5715e8c77bcaf8554a010c45646473614564323535313902b37422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674b46e1bdd9ae346c4c44c5b38e797d07c31003b59fabffa1c5dfcf150bb2d3d4ea7f225401928c855261617dc49ef0fba13097b96e463a6a78976d0c95067e8830a0c45646473614564323535313902b3f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54bb4089d5005906640a2f56348deb8442be2e1ce344267c11f089dff08c96fdeb71a06fdb456cdd25b2a8eb0a6bc0f0d9e7948dcbf842a74ddced6af54be26035705"
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
        "type": "EcdsaSecp256k1",
        "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
      },
      "notary_as_signatory": true,
      "cost_unit_limit": "100000000",
      "tip_percentage": "0"
    },
    "manifest": {
      "instructions": {
        "type": "JSON",
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
            "amount": {
              "type": "Decimal",
              "value": "2"
            },
            "resource_address": {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            },
            "into_bucket": {
              "type": "Bucket",
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
                "identifier": "bucket1"
              }
            ]
          },
          {
            "instruction": "ASSERT_WORKTOP_CONTAINS_BY_AMOUNT",
            "amount": {
              "type": "Decimal",
              "value": "3"
            },
            "resource_address": {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
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
              "identifier": "bucket2"
            }
          },
          {
            "instruction": "CREATE_PROOF_FROM_BUCKET",
            "bucket": {
              "type": "Bucket",
              "identifier": "bucket2"
            },
            "into_proof": {
              "type": "Proof",
              "identifier": "proof1"
            }
          },
          {
            "instruction": "CLONE_PROOF",
            "proof": {
              "type": "Proof",
              "identifier": "proof1"
            },
            "into_proof": {
              "type": "Proof",
              "identifier": "proof2"
            }
          },
          {
            "instruction": "DROP_PROOF",
            "proof": {
              "type": "Proof",
              "identifier": "proof1"
            }
          },
          {
            "instruction": "DROP_PROOF",
            "proof": {
              "type": "Proof",
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
              "identifier": "proof3"
            }
          },
          {
            "instruction": "DROP_PROOF",
            "proof": {
              "type": "Proof",
              "identifier": "proof3"
            }
          },
          {
            "instruction": "RETURN_TO_WORKTOP",
            "bucket": {
              "type": "Bucket",
              "identifier": "bucket2"
            }
          },
          {
            "instruction": "TAKE_FROM_WORKTOP_BY_IDS",
            "ids": [
              {
                "type": "NonFungibleId",
                "variant": "Bytes",
                "value": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
              }
            ],
            "resource_address": {
              "type": "ResourceAddress",
              "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            },
            "into_bucket": {
              "type": "Bucket",
              "identifier": "bucket3"
            }
          },
          {
            "instruction": "CREATE_RESOURCE",
            "resource_type": {
              "type": "Enum",
              "variant": "Fungible",
              "fields": [
                {
                  "type": "U8",
                  "value": "0"
                }
              ]
            },
            "metadata": {
              "type": "Array",
              "element_type": "Tuple",
              "elements": []
            },
            "access_rules": {
              "type": "Array",
              "element_type": "Tuple",
              "elements": []
            },
            "mint_params": {
              "type": "Option",
              "variant": "Some",
              "field": {
                "type": "Enum",
                "variant": "Fungible",
                "fields": [
                  {
                    "type": "Decimal",
                    "value": "1"
                  }
                ]
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
              "value": "deposit_batch"
            },
            "arguments": [
              {
                "type": "Expression",
                "value": "ENTIRE_WORKTOP"
              }
            ]
          },
          {
            "instruction": "DROP_ALL_PROOFS"
          },
          {
            "instruction": "CALL_METHOD",
            "component_address": {
              "type": "ComponentAddress",
              "address": "component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum"
            },
            "method_name": {
              "type": "String",
              "value": "complicated_method"
            },
            "arguments": [
              {
                "type": "Decimal",
                "value": "1"
              },
              {
                "type": "PreciseDecimal",
                "value": "2"
              }
            ]
          },
          {
            "instruction": "PUBLISH_PACKAGE_WITH_OWNER",
            "code": {
              "type": "Blob",
              "hash": "36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618"
            },
            "abi": {
              "type": "Blob",
              "hash": "15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d"
            },
            "owner_badge": {
              "type": "NonFungibleAddress",
              "resource_address": {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              },
              "non_fungible_id": {
                "type": "NonFungibleId",
                "variant": "Bytes",
                "value": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
              }
            }
          }
        ]
      },
      "blobs": [
        "10020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c1000000000",
        "320c1000000000"
      ]
    }
  },
  "intent_signatures": [
    {
      "type": "EcdsaSecp256k1",
      "signature": "01f74e7ec78fc77d1d7bae5ca9c3aa3596680d5ac883f188742bc29ef335b3e7de4902806c811ccad3c75230bbd22a1645aa3825c492c3e834ecbbab3eb966209f"
    },
    {
      "type": "EcdsaSecp256k1",
      "signature": "019db1de979763680b1d82fb2bc377c2d22aae44b68758acd1add4b3fe2a703788369562549f3ac2074777d086ed2cb70154d45a5ed004fcd015b9e5613415775f"
    },
    {
      "type": "EcdsaSecp256k1",
      "signature": "018a49c93437ad33f2cc169d30c01c3a5bcc09c800a221922f5d7e1e16f9f9c64e33bebe61d07373d15ed48e5193d2d1cbbbab5e89f80e0bdaf8fc57f08700ddff"
    },
    {
      "type": "EddsaEd25519",
      "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
      "signature": "afa51548f560636d80b8d16fba359cd3a2d0d214584adc137e53e9570c28deb4982a2a255912f7062ef4746965f196637294a119e43f5715e8c77bcaf8554a01"
    },
    {
      "type": "EddsaEd25519",
      "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
      "signature": "6e1bdd9ae346c4c44c5b38e797d07c31003b59fabffa1c5dfcf150bb2d3d4ea7f225401928c855261617dc49ef0fba13097b96e463a6a78976d0c95067e8830a"
    },
    {
      "type": "EddsaEd25519",
      "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
      "signature": "089d5005906640a2f56348deb8442be2e1ce344267c11f089dff08c96fdeb71a06fdb456cdd25b2a8eb0a6bc0f0d9e7948dcbf842a74ddced6af54be26035705"
    }
  ]
}
```
</details>

## Compile Notarized Transaction Intent

| Function Name     | `compile_notarized_transaction_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_compileNotarizedTransactionIntent` |
| Functionality     | This function takes in a raw signed transaction intent as well as the notary signature and compiles it. This is useful when we wish to submit a transaction to the Gateway API |
| Request Type      | `CompileNotarizedTransactionIntentRequest` |
| Response Type     | `CompileNotarizedTransactionIntentResponse` |

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
          "type": "EcdsaSecp256k1",
          "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
        },
        "notary_as_signatory": true,
        "cost_unit_limit": "100000000",
        "tip_percentage": "0"
      },
      "manifest": {
        "instructions": {
          "type": "String",
          "value": "CALL_METHOD ComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\") \"withdraw_by_amount\" Decimal(\"5\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nTAKE_FROM_WORKTOP_BY_AMOUNT Decimal(\"2\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\") Bucket(\"bucket1\");\nCALL_METHOD ComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\") \"buy_gumball\" Bucket(\"bucket1\");\nASSERT_WORKTOP_CONTAINS_BY_AMOUNT Decimal(\"3\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nASSERT_WORKTOP_CONTAINS ResourceAddress(\"resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6\");\nTAKE_FROM_WORKTOP ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\") Bucket(\"bucket2\");\nCREATE_PROOF_FROM_BUCKET Bucket(\"bucket2\") Proof(\"proof1\");\nCLONE_PROOF Proof(\"proof1\") Proof(\"proof2\");\nDROP_PROOF Proof(\"proof1\");\nDROP_PROOF Proof(\"proof2\");\nCALL_METHOD ComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\") \"create_proof_by_amount\" Decimal(\"5\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nPOP_FROM_AUTH_ZONE Proof(\"proof3\");\nDROP_PROOF Proof(\"proof3\");\nRETURN_TO_WORKTOP Bucket(\"bucket2\");\nTAKE_FROM_WORKTOP_BY_IDS Array<NonFungibleId>(NonFungibleId(Bytes(\"031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f\"))) ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\") Bucket(\"bucket3\");\nCREATE_RESOURCE Enum(\"Fungible\", 0u8) Array<Tuple>() Array<Tuple>() Some(Enum(\"Fungible\", Decimal(\"1\")));\nCALL_METHOD ComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");\nDROP_ALL_PROOFS;\nCALL_METHOD ComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\") \"complicated_method\" Decimal(\"1\") PreciseDecimal(\"2\");\nPUBLISH_PACKAGE_WITH_OWNER Blob(\"36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618\") Blob(\"15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d\") NonFungibleAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\", Bytes(\"031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f\"));\n"
        },
        "blobs": [
          "10020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c1000000000",
          "320c1000000000"
        ]
      }
    },
    "intent_signatures": [
      {
        "type": "EcdsaSecp256k1",
        "signature": "01f74e7ec78fc77d1d7bae5ca9c3aa3596680d5ac883f188742bc29ef335b3e7de4902806c811ccad3c75230bbd22a1645aa3825c492c3e834ecbbab3eb966209f"
      },
      {
        "type": "EcdsaSecp256k1",
        "signature": "019db1de979763680b1d82fb2bc377c2d22aae44b68758acd1add4b3fe2a703788369562549f3ac2074777d086ed2cb70154d45a5ed004fcd015b9e5613415775f"
      },
      {
        "type": "EcdsaSecp256k1",
        "signature": "018a49c93437ad33f2cc169d30c01c3a5bcc09c800a221922f5d7e1e16f9f9c64e33bebe61d07373d15ed48e5193d2d1cbbbab5e89f80e0bdaf8fc57f08700ddff"
      },
      {
        "type": "EddsaEd25519",
        "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
        "signature": "afa51548f560636d80b8d16fba359cd3a2d0d214584adc137e53e9570c28deb4982a2a255912f7062ef4746965f196637294a119e43f5715e8c77bcaf8554a01"
      },
      {
        "type": "EddsaEd25519",
        "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
        "signature": "6e1bdd9ae346c4c44c5b38e797d07c31003b59fabffa1c5dfcf150bb2d3d4ea7f225401928c855261617dc49ef0fba13097b96e463a6a78976d0c95067e8830a"
      },
      {
        "type": "EddsaEd25519",
        "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
        "signature": "089d5005906640a2f56348deb8442be2e1ce344267c11f089dff08c96fdeb71a06fdb456cdd25b2a8eb0a6bc0f0d9e7948dcbf842a74ddced6af54be26035705"
      }
    ]
  },
  "notary_signature": {
    "type": "EcdsaSecp256k1",
    "signature": "002e13bbb2ad772720c5dadb4e9f8dd4144af8400fd625752b31c517f08cabb30e1df32c000e0c9f04fa03dc83756d23f813cdc7219f5d7c2119bdd58341da5be5"
  }
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "compiled_notarized_intent": "5c2102210221022109070107f20a00020000000000000a10020000000000000a2200000000000000110e4563647361536563703235366b3101b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505070021022011140a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000041754616b6546726f6d576f726b746f704279416d6f756e7402b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040a43616c6c4d6574686f640221021106476c6f62616c01810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c210192000200001d417373657274576f726b746f70436f6e7461696e734279416d6f756e7402b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000415417373657274576f726b746f70436f6e7461696e73018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510f54616b6546726f6d576f726b746f7001820000000000000000000000000000000000000000000000000000041543726561746550726f6f6646726f6d4275636b65740109010200000a436c6f6e6550726f6f660109020200000944726f7050726f6f660109020200000944726f7050726f6f660109030200000a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040f506f7046726f6d417574685a6f6e65000944726f7050726f6f660109040200000f52657475726e546f576f726b746f700109010200001454616b6546726f6d576f726b746f7042794964730220b701255c200721031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f820000000000000000000000000000000000000000000000000000041243616c6c4e617469766546756e6374696f6e0221020c0f5265736f757263654d616e616765720c066372656174652007495c2104110846756e6769626c650107002021002021001104536f6d6501110846756e6769626c6501b5000064a7b3b6e00d0000000000000000000000000000000000000000000000000a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007135c2101a00e454e544952455f574f524b544f500d44726f70416c6c50726f6f6673000a43616c6c4d6574686f640221021106476c6f62616c01810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c12636f6d706c6963617465645f6d6574686f642007655c2102b5000064a7b3b6e00d000000000000000000000000000000000000000000000000b60000000000000000023ed47ec9da71dcda2f4fb5e9f37fd2079e3000000000000000000000000000000000000000000000000000000000000000000000000000175075626c6973685061636b616765576974684f776e657203a136dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618a115e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5da2400000000000000000000000000000000000000000000000000000045c200721031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f20200207d20110020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c10000000000707320c10000000002011060e4563647361536563703235366b3101b201f74e7ec78fc77d1d7bae5ca9c3aa3596680d5ac883f188742bc29ef335b3e7de4902806c811ccad3c75230bbd22a1645aa3825c492c3e834ecbbab3eb966209f0e4563647361536563703235366b3101b2019db1de979763680b1d82fb2bc377c2d22aae44b68758acd1add4b3fe2a703788369562549f3ac2074777d086ed2cb70154d45a5ed004fcd015b9e5613415775f0e4563647361536563703235366b3101b2018a49c93437ad33f2cc169d30c01c3a5bcc09c800a221922f5d7e1e16f9f9c64e33bebe61d07373d15ed48e5193d2d1cbbbab5e89f80e0bdaf8fc57f08700ddff0c45646473614564323535313902b34cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29b4afa51548f560636d80b8d16fba359cd3a2d0d214584adc137e53e9570c28deb4982a2a255912f7062ef4746965f196637294a119e43f5715e8c77bcaf8554a010c45646473614564323535313902b37422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674b46e1bdd9ae346c4c44c5b38e797d07c31003b59fabffa1c5dfcf150bb2d3d4ea7f225401928c855261617dc49ef0fba13097b96e463a6a78976d0c95067e8830a0c45646473614564323535313902b3f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54bb4089d5005906640a2f56348deb8442be2e1ce344267c11f089dff08c96fdeb71a06fdb456cdd25b2a8eb0a6bc0f0d9e7948dcbf842a74ddced6af54be26035705110e4563647361536563703235366b3101b2002e13bbb2ad772720c5dadb4e9f8dd4144af8400fd625752b31c517f08cabb30e1df32c000e0c9f04fa03dc83756d23f813cdc7219f5d7c2119bdd58341da5be5"
}
```
</details>

## Decompile Notarized Transaction Intent

| Function Name     | `decompile_notarized_transaction_intent` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_decompileNotarizedTransactionIntent` |
| Functionality     | This function does the opposite of the compile_notarized_transaction_intent function. This function takes in a compiled notarized transaction intent and decompiles it into its signed transaction intent and notary signature. |
| Request Type      | `DecompileNotarizedTransactionIntentRequest` |
| Response Type     | `DecompileNotarizedTransactionIntentResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "manifest_instructions_output_format": "JSON",
  "compiled_notarized_intent": "5c2102210221022109070107f20a00020000000000000a10020000000000000a2200000000000000110e4563647361536563703235366b3101b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505070021022011140a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000041754616b6546726f6d576f726b746f704279416d6f756e7402b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040a43616c6c4d6574686f640221021106476c6f62616c01810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c210192000200001d417373657274576f726b746f70436f6e7461696e734279416d6f756e7402b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000415417373657274576f726b746f70436f6e7461696e73018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510f54616b6546726f6d576f726b746f7001820000000000000000000000000000000000000000000000000000041543726561746550726f6f6646726f6d4275636b65740109010200000a436c6f6e6550726f6f660109020200000944726f7050726f6f660109020200000944726f7050726f6f660109030200000a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040f506f7046726f6d417574685a6f6e65000944726f7050726f6f660109040200000f52657475726e546f576f726b746f700109010200001454616b6546726f6d576f726b746f7042794964730220b701255c200721031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f820000000000000000000000000000000000000000000000000000041243616c6c4e617469766546756e6374696f6e0221020c0f5265736f757263654d616e616765720c066372656174652007495c2104110846756e6769626c650107002021002021001104536f6d6501110846756e6769626c6501b5000064a7b3b6e00d0000000000000000000000000000000000000000000000000a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007135c2101a00e454e544952455f574f524b544f500d44726f70416c6c50726f6f6673000a43616c6c4d6574686f640221021106476c6f62616c01810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c12636f6d706c6963617465645f6d6574686f642007655c2102b5000064a7b3b6e00d000000000000000000000000000000000000000000000000b60000000000000000023ed47ec9da71dcda2f4fb5e9f37fd2079e3000000000000000000000000000000000000000000000000000000000000000000000000000175075626c6973685061636b616765576974684f776e657203a136dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618a115e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5da2400000000000000000000000000000000000000000000000000000045c200721031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f20200207d20110020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c10000000000707320c10000000002011060e4563647361536563703235366b3101b201f74e7ec78fc77d1d7bae5ca9c3aa3596680d5ac883f188742bc29ef335b3e7de4902806c811ccad3c75230bbd22a1645aa3825c492c3e834ecbbab3eb966209f0e4563647361536563703235366b3101b2019db1de979763680b1d82fb2bc377c2d22aae44b68758acd1add4b3fe2a703788369562549f3ac2074777d086ed2cb70154d45a5ed004fcd015b9e5613415775f0e4563647361536563703235366b3101b2018a49c93437ad33f2cc169d30c01c3a5bcc09c800a221922f5d7e1e16f9f9c64e33bebe61d07373d15ed48e5193d2d1cbbbab5e89f80e0bdaf8fc57f08700ddff0c45646473614564323535313902b34cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29b4afa51548f560636d80b8d16fba359cd3a2d0d214584adc137e53e9570c28deb4982a2a255912f7062ef4746965f196637294a119e43f5715e8c77bcaf8554a010c45646473614564323535313902b37422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674b46e1bdd9ae346c4c44c5b38e797d07c31003b59fabffa1c5dfcf150bb2d3d4ea7f225401928c855261617dc49ef0fba13097b96e463a6a78976d0c95067e8830a0c45646473614564323535313902b3f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54bb4089d5005906640a2f56348deb8442be2e1ce344267c11f089dff08c96fdeb71a06fdb456cdd25b2a8eb0a6bc0f0d9e7948dcbf842a74ddced6af54be26035705110e4563647361536563703235366b3101b2002e13bbb2ad772720c5dadb4e9f8dd4144af8400fd625752b31c517f08cabb30e1df32c000e0c9f04fa03dc83756d23f813cdc7219f5d7c2119bdd58341da5be5"
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
          "type": "EcdsaSecp256k1",
          "public_key": "03c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa21"
        },
        "notary_as_signatory": true,
        "cost_unit_limit": "100000000",
        "tip_percentage": "0"
      },
      "manifest": {
        "instructions": {
          "type": "JSON",
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
              "amount": {
                "type": "Decimal",
                "value": "2"
              },
              "resource_address": {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              },
              "into_bucket": {
                "type": "Bucket",
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
                  "identifier": "bucket1"
                }
              ]
            },
            {
              "instruction": "ASSERT_WORKTOP_CONTAINS_BY_AMOUNT",
              "amount": {
                "type": "Decimal",
                "value": "3"
              },
              "resource_address": {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
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
                "identifier": "bucket2"
              }
            },
            {
              "instruction": "CREATE_PROOF_FROM_BUCKET",
              "bucket": {
                "type": "Bucket",
                "identifier": "bucket2"
              },
              "into_proof": {
                "type": "Proof",
                "identifier": "proof1"
              }
            },
            {
              "instruction": "CLONE_PROOF",
              "proof": {
                "type": "Proof",
                "identifier": "proof1"
              },
              "into_proof": {
                "type": "Proof",
                "identifier": "proof2"
              }
            },
            {
              "instruction": "DROP_PROOF",
              "proof": {
                "type": "Proof",
                "identifier": "proof1"
              }
            },
            {
              "instruction": "DROP_PROOF",
              "proof": {
                "type": "Proof",
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
                "identifier": "proof3"
              }
            },
            {
              "instruction": "DROP_PROOF",
              "proof": {
                "type": "Proof",
                "identifier": "proof3"
              }
            },
            {
              "instruction": "RETURN_TO_WORKTOP",
              "bucket": {
                "type": "Bucket",
                "identifier": "bucket2"
              }
            },
            {
              "instruction": "TAKE_FROM_WORKTOP_BY_IDS",
              "ids": [
                {
                  "type": "NonFungibleId",
                  "variant": "Bytes",
                  "value": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
                }
              ],
              "resource_address": {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
              },
              "into_bucket": {
                "type": "Bucket",
                "identifier": "bucket3"
              }
            },
            {
              "instruction": "CREATE_RESOURCE",
              "resource_type": {
                "type": "Enum",
                "variant": "Fungible",
                "fields": [
                  {
                    "type": "U8",
                    "value": "0"
                  }
                ]
              },
              "metadata": {
                "type": "Array",
                "element_type": "Tuple",
                "elements": []
              },
              "access_rules": {
                "type": "Array",
                "element_type": "Tuple",
                "elements": []
              },
              "mint_params": {
                "type": "Option",
                "variant": "Some",
                "field": {
                  "type": "Enum",
                  "variant": "Fungible",
                  "fields": [
                    {
                      "type": "Decimal",
                      "value": "1"
                    }
                  ]
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
                "value": "deposit_batch"
              },
              "arguments": [
                {
                  "type": "Expression",
                  "value": "ENTIRE_WORKTOP"
                }
              ]
            },
            {
              "instruction": "DROP_ALL_PROOFS"
            },
            {
              "instruction": "CALL_METHOD",
              "component_address": {
                "type": "ComponentAddress",
                "address": "component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum"
              },
              "method_name": {
                "type": "String",
                "value": "complicated_method"
              },
              "arguments": [
                {
                  "type": "Decimal",
                  "value": "1"
                },
                {
                  "type": "PreciseDecimal",
                  "value": "2"
                }
              ]
            },
            {
              "instruction": "PUBLISH_PACKAGE_WITH_OWNER",
              "code": {
                "type": "Blob",
                "hash": "36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618"
              },
              "abi": {
                "type": "Blob",
                "hash": "15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d"
              },
              "owner_badge": {
                "type": "NonFungibleAddress",
                "resource_address": {
                  "type": "ResourceAddress",
                  "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
                },
                "non_fungible_id": {
                  "type": "NonFungibleId",
                  "variant": "Bytes",
                  "value": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
                }
              }
            }
          ]
        },
        "blobs": [
          "10020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c1000000000",
          "320c1000000000"
        ]
      }
    },
    "intent_signatures": [
      {
        "type": "EcdsaSecp256k1",
        "signature": "01f74e7ec78fc77d1d7bae5ca9c3aa3596680d5ac883f188742bc29ef335b3e7de4902806c811ccad3c75230bbd22a1645aa3825c492c3e834ecbbab3eb966209f"
      },
      {
        "type": "EcdsaSecp256k1",
        "signature": "019db1de979763680b1d82fb2bc377c2d22aae44b68758acd1add4b3fe2a703788369562549f3ac2074777d086ed2cb70154d45a5ed004fcd015b9e5613415775f"
      },
      {
        "type": "EcdsaSecp256k1",
        "signature": "018a49c93437ad33f2cc169d30c01c3a5bcc09c800a221922f5d7e1e16f9f9c64e33bebe61d07373d15ed48e5193d2d1cbbbab5e89f80e0bdaf8fc57f08700ddff"
      },
      {
        "type": "EddsaEd25519",
        "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29",
        "signature": "afa51548f560636d80b8d16fba359cd3a2d0d214584adc137e53e9570c28deb4982a2a255912f7062ef4746965f196637294a119e43f5715e8c77bcaf8554a01"
      },
      {
        "type": "EddsaEd25519",
        "public_key": "7422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674",
        "signature": "6e1bdd9ae346c4c44c5b38e797d07c31003b59fabffa1c5dfcf150bb2d3d4ea7f225401928c855261617dc49ef0fba13097b96e463a6a78976d0c95067e8830a"
      },
      {
        "type": "EddsaEd25519",
        "public_key": "f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54b",
        "signature": "089d5005906640a2f56348deb8442be2e1ce344267c11f089dff08c96fdeb71a06fdb456cdd25b2a8eb0a6bc0f0d9e7948dcbf842a74ddced6af54be26035705"
      }
    ]
  },
  "notary_signature": {
    "type": "EcdsaSecp256k1",
    "signature": "002e13bbb2ad772720c5dadb4e9f8dd4144af8400fd625752b31c517f08cabb30e1df32c000e0c9f04fa03dc83756d23f813cdc7219f5d7c2119bdd58341da5be5"
  }
}
```
</details>

## Encode Address

| Function Name     | `encode_address` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_encodeAddress` |
| Functionality     | This function can be used when we have a byte array which we wish to do Bech32m encoding on. In this case, the HRP to use will be determined through the entity byte of the passed address hex string. |
| Request Type      | `EncodeAddressRequest` |
| Response Type     | `EncodeAddressResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "address_bytes": "000000000000000000000000000000000000000000000000000002",
  "network_id": 242
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
| JNI Function Name | `Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_decodeAddress` |
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
  "network_id": 242,
  "network_name": "simulator",
  "entity_type": "Resource",
  "data": "000000000000000000000000000000000000000000000000000002",
  "hrp": "resource_sim",
  "address": {
    "type": "ResourceAddress",
    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpqd60rqz"
  }
}
```
</details>

## Sbor Encode

| Function Name     | `sbor_encode` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_sborEncode` |
| Functionality     | This function takes in a Value and encodes it in SBOR. |
| Request Type      | `SBOREncodeRequest` |
| Response Type     | `SBOREncodeResponse` |

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
                  "element_type": "Decimal",
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
| JNI Function Name | `Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_sborDecode` |
| Functionality     | This function takes in a hex string and attemps to decode it into a Value. |
| Request Type      | `SBORDecodeRequest` |
| Response Type     | `SBORDecodeResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "encoded_value": "5c2104b50000e8890423c78a000000000000000000000000000000000000000000000000b600000000000000000a36257aef45394e46ef8b8a90c37f1c2716f3000000000000000000000000000000000000000000000000000000000000000000000000000c0c48656c6c6f20576f726c64212104b50000e8890423c78a000000000000000000000000000000000000000000000000b600000000000000000a36257aef45394e46ef8b8a90c37f1c2716f3000000000000000000000000000000000000000000000000000000000000000000000000000c0c48656c6c6f20576f726c64212104b50000e8890423c78a000000000000000000000000000000000000000000000000b600000000000000000a36257aef45394e46ef8b8a90c37f1c2716f3000000000000000000000000000000000000000000000000000000000000000000000000000c0c48656c6c6f20576f726c64212104b50000e8890423c78a000000000000000000000000000000000000000000000000b600000000000000000a36257aef45394e46ef8b8a90c37f1c2716f3000000000000000000000000000000000000000000000000000000000000000000000000000c0c48656c6c6f20576f726c642120b5030000d01309468e15010000000000000000000000000000000000000000000000000010632d5ec76b05000000000000000000000000000000000000000000000000005f13195ed66c0a0000000000000000000000000000000000000000000000",
  "network_id": 242
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
                  "element_type": "Decimal",
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
| JNI Function Name | `Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_deriveVirtualAccountAddress` |
| Functionality     | Derives the virtual account component address given a public key and a network id. |
| Request Type      | `DeriveVirtualAccountAddressRequest` |
| Response Type     | `DeriveVirtualAccountAddressResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "network_id": 242,
  "public_key": {
    "type": "EcdsaSecp256k1",
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

## Known Entity Addresses

| Function Name     | `known_entity_addresses` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_knownEntityAddresses` |
| Functionality     | Given a network id, this function derives the Bech32m-encoded addresses of the set of known addresses.</br>        </br>        As an example, this function allows users to derive the XRD resource address, faucet component address, or account package address on any network (given that they know its network id). |
| Request Type      | `KnownEntityAddressesRequest` |
| Response Type     | `KnownEntityAddressesResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "network_id": 1
}
```
</details>

<details>
    <summary>Response Example</summary>
    
```json
{
  "faucet_component_address": {
    "type": "ComponentAddress",
    "address": "component_rdx1qftacppvmr9ezmekxqpq58en0nk954x0a7jv2zz0hc7qp7wydu"
  },
  "faucet_package_address": {
    "type": "PackageAddress",
    "address": "package_rdx1qyqzcexvnyg60z7lnlwauh66nhzg3m8tch2j8wc0e70qsxg0vu"
  },
  "account_package_address": {
    "type": "PackageAddress",
    "address": "package_rdx1qy4hrp8a9apxldp5cazvxgwdj80cxad4u8cpkaqqnhlsmn6s2x"
  },
  "xrd_resource_address": {
    "type": "ResourceAddress",
    "address": "resource_rdx1qzkcyv5dwq3r6kawy6pxpvcythx8rh8ntum6ws62p95sxshc9u"
  },
  "system_token_resource_address": {
    "type": "ResourceAddress",
    "address": "resource_rdx1qzfzxp4x6ya0vwrxk6yvjyphr8gyk9xqvz7y3xdxzw6sfg0scu"
  },
  "ecdsa_secp256k1_token_resource_address": {
    "type": "ResourceAddress",
    "address": "resource_rdx1qzu3wdlw3fx7t82fmt2qme2kpet4g3n2epx02sew49wszqm9ur"
  },
  "eddsa_ed25519_token_resource_address": {
    "type": "ResourceAddress",
    "address": "resource_rdx1qq8cays25704xdyap2vhgmshkkfyr023uxdtk59ddd4qk9a6ln"
  },
  "epoch_manager_system_address": {
    "type": "SystemAddress",
    "address": "system_rdx1qne8qu4seyvzfgd94p3z8rjcdl3v0nfhv84judpum2lqcysr6t"
  },
  "clock_system_address": {
    "type": "SystemAddress",
    "address": "system_rdx1qhrvq0wjqnnzcwwm8jhzxw2ctd3t4aqql0a56a9mu5nsq4wurk"
  }
}
```
</details>

## Statically Validate Transaction

| Function Name     | `statically_validate_transaction` |
| ----------------- | :----------------- |
| JNI Function Name | `Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_staticallyValidateTransaction` |
| Functionality     | Performs static validation on the given notarized transaction. |
| Request Type      | `StaticallyValidateTransactionRequest` |
| Response Type     | `StaticallyValidateTransactionResponse` |

<details>
    <summary>Request Example</summary>
    
```json
{
  "compiled_notarized_intent": "5c2102210221022109070107f20a00020000000000000a10020000000000000a2200000000000000110e4563647361536563703235366b3101b103c32f9761dd3f961a3d12747e54db6b821bd022ef92b9ebf591bfe186885baa2101010900e1f505070021022011140a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1277697468647261775f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000041754616b6546726f6d576f726b746f704279416d6f756e7402b50000c84e676dc11b000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040a43616c6c4d6574686f640221021106476c6f62616c01810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b6275795f67756d62616c6c2007085c210192000200001d417373657274576f726b746f70436f6e7461696e734279416d6f756e7402b500002cf61a24a2290000000000000000000000000000000000000000000000008200000000000000000000000000000000000000000000000000000415417373657274576f726b746f70436f6e7461696e73018200aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510f54616b6546726f6d576f726b746f7001820000000000000000000000000000000000000000000000000000041543726561746550726f6f6646726f6d4275636b65740109010200000a436c6f6e6550726f6f660109020200000944726f7050726f6f660109020200000944726f7050726f6f660109030200000a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c166372656174655f70726f6f665f62795f616d6f756e742007405c2102b50000f44482916345000000000000000000000000000000000000000000000000820000000000000000000000000000000000000000000000000000040f506f7046726f6d417574685a6f6e65000944726f7050726f6f660109040200000f52657475726e546f576f726b746f700109010200001454616b6546726f6d576f726b746f7042794964730220b701255c200721031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f820000000000000000000000000000000000000000000000000000041243616c6c4e617469766546756e6374696f6e0221020c0f5265736f757263654d616e616765720c066372656174652007495c2104110846756e6769626c650107002021002021001104536f6d6501110846756e6769626c6501b5000064a7b3b6e00d0000000000000000000000000000000000000000000000000a43616c6c4d6574686f640221021106476c6f62616c018103d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d6465706f7369745f62617463682007135c2101a00e454e544952455f574f524b544f500d44726f70416c6c50726f6f6673000a43616c6c4d6574686f640221021106476c6f62616c01810292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c12636f6d706c6963617465645f6d6574686f642007655c2102b5000064a7b3b6e00d000000000000000000000000000000000000000000000000b60000000000000000023ed47ec9da71dcda2f4fb5e9f37fd2079e3000000000000000000000000000000000000000000000000000000000000000000000000000175075626c6973685061636b616765576974684f776e657203a136dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618a115e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5da2400000000000000000000000000000000000000000000000000000045c200721031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f20200207d20110020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c10000000000707320c10000000002011060e4563647361536563703235366b3101b201f74e7ec78fc77d1d7bae5ca9c3aa3596680d5ac883f188742bc29ef335b3e7de4902806c811ccad3c75230bbd22a1645aa3825c492c3e834ecbbab3eb966209f0e4563647361536563703235366b3101b2019db1de979763680b1d82fb2bc377c2d22aae44b68758acd1add4b3fe2a703788369562549f3ac2074777d086ed2cb70154d45a5ed004fcd015b9e5613415775f0e4563647361536563703235366b3101b2018a49c93437ad33f2cc169d30c01c3a5bcc09c800a221922f5d7e1e16f9f9c64e33bebe61d07373d15ed48e5193d2d1cbbbab5e89f80e0bdaf8fc57f08700ddff0c45646473614564323535313902b34cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29b4afa51548f560636d80b8d16fba359cd3a2d0d214584adc137e53e9570c28deb4982a2a255912f7062ef4746965f196637294a119e43f5715e8c77bcaf8554a010c45646473614564323535313902b37422b9887598068e32c4448a949adb290d0f4e35b9e01b0ee5f1a1e600fe2674b46e1bdd9ae346c4c44c5b38e797d07c31003b59fabffa1c5dfcf150bb2d3d4ea7f225401928c855261617dc49ef0fba13097b96e463a6a78976d0c95067e8830a0c45646473614564323535313902b3f381626e41e7027ea431bfe3009e94bdd25a746beec468948d6c3c7c5dc9a54bb4089d5005906640a2f56348deb8442be2e1ce344267c11f089dff08c96fdeb71a06fdb456cdd25b2a8eb0a6bc0f0d9e7948dcbf842a74ddced6af54be26035705110e4563647361536563703235366b3101b201f74e7ec78fc77d1d7bae5ca9c3aa3596680d5ac883f188742bc29ef335b3e7de4902806c811ccad3c75230bbd22a1645aa3825c492c3e834ecbbab3eb966209f",
  "validation_config": {
    "network_id": "242",
    "min_cost_unit_limit": "1000000",
    "max_cost_unit_limit": "100000000",
    "min_tip_percentage": "0",
    "max_tip_percentage": "255",
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
