# Babylon Transaction Library

This is a WebAssembly (WASM) library built using Rust to provide language-agnostic support for the following:

1. The compilation and decompilation of unsigned, signed, and notarized transaction intents.
1. The ability to convert manifests between their string and JSON representations.
1. The ability to Bech32 encode and decode addresses as needed. 
1. The ability to encode and decode SBOR values as needed. 

A client written in any language can leverage the abilities of this library as long as it has a WASM runtime library. [Here](https://github.com/wasmerio/wasmer#-language-integrations) is a list of all of the available wasmer runtimes, [here](https://github.com/bytecodealliance/wasmtime) is a list of the languages with a wasmtime runtime, and [here](https://github.com/appcypher/awesome-wasm-runtimes) is a list of the different WASM runtimes available and their supported language. Any WASM runtime should be suitable for this library.

## Background

Babylon transactions are composed of a number of different parts, but their building blocks essentially boil down to: a transaction header, a manifest, and signatures which might either be intent signatures or a notary signature depending on what exactly was signed. The diagram below illustrates the relationship and names of the different parts of a transaction. 

![image](./images/v0.5.0-transaction-model.png)

When we have a transaction intent, getting to a signed transaction intent requires that the transaction intent is "compiled" and then that the double hash of the compiled transaction intent is signed. In this context, compiling something simply refers to encoding it in SBOR. Once all of the signers have signed the double hash of the compiled transaction intent, we have all we need to for a signed transaction intent. 

Similarity, when the notary wishes to notarize a signed transaction intent, they compile the signed transaction intent (as explained above, they encode the signed transaction intent in SBOR), sign the double hash of that, and with that they have the notary signature required to form a complete transaction. 

## Motivation

As can be seen in the [background section](#background), the process of creating a transaction requires that a client is able to encode different parts of a transaction in SBOR to later allow for these parts to be signed or notarized. This means that a client would need to, at a minimum, have an SBOR encoder for compiling transactions, and perhaps a decoder if the decompilation of transactions is desirable. 

The main implementation of SBOR is written in Rust. That being said, clients wishing to integrate their services with Radix (or just build and send programmatic transactions) could have clients written in a any programming languages. It is unrealistic to expect that clients would write, test, and maintain their own SBOR implementation in their client language as this is a high implementation burden on the client.

In addition to the building of transaction requiring SBOR for compilation, certain clients might wish to decompile transactions to figure out what the transaction intent is. Much like compilation, without an SBOR implementation available to the client, the decompilation of transactions would be impossible. 

Therefore, there is a strong need to **not** rewrite SBOR in different languages, instead, to reuse and reutilize the original SBOR implementation for clients written in any language. This library achieves that by being fully written in rust; thus leveraging the ability to use the original SBOR implementation, and being compilable to different targets with WASM and native iOS being the two main compilation targets for this library. WASM being a main compilation target means that any programming language with a WASM runtime can leverage and make use of this library. 

The choice of making the transaction library compilable to WASM comes with a number of advantages, the first of which is that certain clients may wish to build their transactions, SBOR encode and decode their data, or Bech32 encode or decode their addresses in a trustless fashion without having to rely on a REST API where a non-negligible chance of tampering exists. Having this library as a WASM module allows such clients to do all they need without worrying about trusting an external server.

In addition to that, using WASM instead of a docker-based solution to this problem means that the solution is very lightweight and also compatible with platforms where Docker is no an option (mostly smart phones.), thus making this library available to the largest amount of clients without any compromises. 

Therefore, this library provides the following features to any client language that has a WASM runtime:

1. The compilation and decompilation of unsigned, signed, and notarized transaction intents.
1. The ability to convert manifests between their string and JSON representations.
1. The ability to Bech32 encode and decode addresses as needed. 
1. The ability to encode and decode SBOR values as needed. 

## Features and Functions

This section lists all of the functions available in this library, what they are used for, what their arguments and returns are, and examples of their request and return. The types mentioned in this section are all provided in the [OpenAPI specification](./spec/transaction-api-spec.yaml) of this library, so, not much description will be given for these types here. 

### Convert Manifest

| Function Name | `convert_manifest` |
| ------------- | :----------------- |
| Functionality | Clients have a need to be able to read, parse, understand, and interrogate transaction manifests to get more information on what a transactions might be doing. Transaction manifests have so far existed in one format: as strings. While the string format is very human readable, it is not easily readable by machines as a lexer and parser are needed to make sense of them; thus, it is for clients to programmatically make sense of transactions. As such, there is a need for another transaction manifest format (to supplement, **NOT** replace) which machines can easily make sense of without the need to implement a lexer and parser.</br></br>Therefore, this library introduces a JSON format for transaction manifests which clients can use when wanting to read and interrogate their transaction manifests in code. The transaction manifest JSON format has a 1:1 mapping to the string format of transaction manifests, meaning that anything which can be done in the string format of transaction manifests, can be done in the JSON format as well.</br></br>This function allows the client the convert their manifest between the two supported manifest types: string and JSON. |
| Request Type  | `ConvertManifestRequest` |
| Response Type | `ConvertManifestResponse` |

<details>
    <summary>Request Example</summary>
  
```json
{
    "transaction_version": 1,
    "network_id": 242,
    "manifest_instructions_output_format": "JSON",
    "manifest": {
        "type": "String",
        "value": "# Withdraw XRD from account\nCALL_METHOD\n\tComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\")\n\t\"withdraw_by_amount\"\n\tDecimal(\"5.0\")\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\n\n# Buy GUM with XRD\nTAKE_FROM_WORKTOP_BY_AMOUNT\n\tDecimal(\"2.0\")\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n\tBucket(\"xrd\");\nCALL_METHOD\n\tComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\")\n\t\"buy_gumball\"\n\tBucket(\"xrd\");\nASSERT_WORKTOP_CONTAINS_BY_AMOUNT\n\tDecimal(\"3.0\")\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nASSERT_WORKTOP_CONTAINS\n\tResourceAddress(\"resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6\");\n\n# Create a proof from bucket, clone it and drop both\nTAKE_FROM_WORKTOP\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n\tBucket(\"1u32\");\nCREATE_PROOF_FROM_BUCKET\n\tBucket(\"1u32\")\n\tProof(\"proof1\");\nCLONE_PROOF\n\tProof(\"proof1\")\n\tProof(\"proof2\");\nDROP_PROOF\n\tProof(\"proof1\");\nDROP_PROOF\n\tProof(\"proof2\");\n\n# Create a proof from account and drop it\nCALL_METHOD\n\tComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\")\n\t\"create_proof_by_amount\"\n\tDecimal(\"5.0\")\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nPOP_FROM_AUTH_ZONE\n\tProof(\"proof3\");\nDROP_PROOF\n\tProof(\"proof3\");\n\n# Return a bucket to worktop\nRETURN_TO_WORKTOP\n\tBucket(\"1u32\");\nTAKE_FROM_WORKTOP_BY_IDS\n\tSet<NonFungibleId>(NonFungibleId(\"0905000000\"),NonFungibleId(\"0907000000\"))\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n\tBucket(\"nfts\");\n\n# Cancel all buckets and move resources to account\nCALL_METHOD_WITH_ALL_RESOURCES\n\tComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\")\n\t\"deposit_batch\";\n\n# Drop all proofs\nDROP_ALL_PROOFS;\n\n# Two ways of publishing package through manifest\nPUBLISH_PACKAGE\n\tBytes(\"10020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c1000000000\");\n\n# Complicated method that takes all of the number types\nCALL_METHOD\n\tComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\")\n\t\"complicated_method\"\n\tDecimal(\"1\")\n\tPreciseDecimal(\"2\");"
    }
}
```
</details>

<details>
    <summary>Response Example</summary>
  
```json
{
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
                "identifier": "xrd"
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
                    "identifier": "xrd"
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
                "identifier": "1u32"
            }
        },
        {
            "instruction": "CREATE_PROOF_FROM_BUCKET",
            "bucket": {
                "type": "Bucket",
                "identifier": "1u32"
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
                "identifier": "1u32"
            }
        },
        {
            "instruction": "TAKE_FROM_WORKTOP_BY_IDS",
            "ids": [
                {
                    "type": "NonFungibleId",
                    "value": "0905000000"
                },
                {
                    "type": "NonFungibleId",
                    "value": "0907000000"
                }
            ],
            "resource_address": {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
            },
            "into_bucket": {
                "type": "Bucket",
                "identifier": "nfts"
            }
        },
        {
            "instruction": "CALL_METHOD_WITH_ALL_RESOURCES",
            "component_address": {
                "type": "ComponentAddress",
                "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
            },
            "method": {
                "type": "String",
                "value": "deposit_batch"
            }
        },
        {
            "instruction": "DROP_ALL_PROOFS"
        },
        {
            "instruction": "PUBLISH_PACKAGE",
            "package": {
                "type": "Bytes",
                "value": "10020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c1000000000"
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
        }
    ]
}
```
</details>

### Compile Transaction Intent

| Function Name | `compile_transaction_intent` |
| ------------- | :----------------- |
| Functionality | Takes a transaction intent and compiles it by SBOR encoding it and returning it back to the caller. This is mainly useful when creating a transaction. |
| Request Type  | `CompileTransactionIntentRequest` |
| Response Type | `CompileTransactionIntentResponse` |

<details>
    <summary>Request Example</summary>
  
```json
{
    "header": {
        "version": 1,
        "network_id": 242,
        "start_epoch_inclusive": 0,
        "end_epoch_exclusive": 32,
        "nonce": 0,
        "notary_public_key": "031c3796382de8e6e7a1aacb069221e43943af8be417d4c8c92dca7c4b07f93969",
        "notary_as_signatory": false,
        "cost_unit_limit": 0,
        "tip_percentage": 0
    },
    "manifest": {
        "type": "String",
        "value": "# Withdraw XRD from account\nCALL_METHOD\n\tComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\")\n\t\"withdraw_by_amount\"\n\tDecimal(\"5.0\")\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\n\n# Buy GUM with XRD\nTAKE_FROM_WORKTOP_BY_AMOUNT\n\tDecimal(\"2.0\")\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n\tBucket(\"xrd\");\nCALL_METHOD\n\tComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\")\n\t\"buy_gumball\"\n\tBucket(\"xrd\");\nASSERT_WORKTOP_CONTAINS_BY_AMOUNT\n\tDecimal(\"3.0\")\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nASSERT_WORKTOP_CONTAINS\n\tResourceAddress(\"resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6\");\n\n# Create a proof from bucket, clone it and drop both\nTAKE_FROM_WORKTOP\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n\tBucket(\"1u32\");\nCREATE_PROOF_FROM_BUCKET\n\tBucket(\"1u32\")\n\tProof(\"proof1\");\nCLONE_PROOF\n\tProof(\"proof1\")\n\tProof(\"proof2\");\nDROP_PROOF\n\tProof(\"proof1\");\nDROP_PROOF\n\tProof(\"proof2\");\n\n# Create a proof from account and drop it\nCALL_METHOD\n\tComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\")\n\t\"create_proof_by_amount\"\n\tDecimal(\"5.0\")\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nPOP_FROM_AUTH_ZONE\n\tProof(\"proof3\");\nDROP_PROOF\n\tProof(\"proof3\");\n\n# Return a bucket to worktop\nRETURN_TO_WORKTOP\n\tBucket(\"1u32\");\nTAKE_FROM_WORKTOP_BY_IDS\n\tSet<NonFungibleId>(NonFungibleId(\"0905000000\"),NonFungibleId(\"0907000000\"))\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n\tBucket(\"nfts\");\n\n# Cancel all buckets and move resources to account\nCALL_METHOD_WITH_ALL_RESOURCES\n\tComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\")\n\t\"deposit_batch\";\n\n# Drop all proofs\nDROP_ALL_PROOFS;\n\n# Two ways of publishing package through manifest\nPUBLISH_PACKAGE\n\tBytes(\"10020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c1000000000\");\n\n# Complicated method that takes all of the number types\nCALL_METHOD\n\tComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\")\n\t\"complicated_method\"\n\tDecimal(\"1\")\n\tPreciseDecimal(\"2\");"
    }
}
```
</details>

<details>
    <summary>Response Example</summary>
  
```json
{
    "compiled_intent": "10020000001009000000070107f20a00000000000000000a20000000000000000a00000000000000009121000000031c3796382de8e6e7a1aacb069221e43943af8be417d4c8c92dca7c4b07f9396901000900000000090000000010010000003011130000000a00000043616c6c4d6574686f6403000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1200000077697468647261775f62795f616d6f756e7430074a0000001002000000a1200000000000f44482916345000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000041700000054616b6546726f6d576f726b746f704279416d6f756e7402000000a1200000000000c84e676dc11b000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000040a00000043616c6c4d6574686f6403000000811b0000000292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b0000006275795f67756d62616c6c30070e0000001001000000b104000000000200001d000000417373657274576f726b746f70436f6e7461696e734279416d6f756e7402000000a12000000000002cf61a24a229000000000000000000000000000000000000000000000000b61b00000000000000000000000000000000000000000000000000000000000415000000417373657274576f726b746f70436f6e7461696e7301000000b61b00000000aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510f00000054616b6546726f6d576f726b746f7001000000b61b0000000000000000000000000000000000000000000000000000000000041500000043726561746550726f6f6646726f6d4275636b65740100000009010200000a000000436c6f6e6550726f6f660100000009020200000900000044726f7050726f6f660100000009020200000900000044726f7050726f6f660100000009030200000a00000043616c6c4d6574686f6403000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c160000006372656174655f70726f6f665f62795f616d6f756e7430074a0000001002000000a1200000000000f44482916345000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000040f000000506f7046726f6d417574685a6f6e65000000000900000044726f7050726f6f660100000009040200000f00000052657475726e546f576f726b746f700100000009010200001400000054616b6546726f6d576f726b746f7042794964730200000031b402000000050000000905000000050000000907000000b61b0000000000000000000000000000000000000000000000000000000000041a00000043616c6c4d6574686f6457697468416c6c5265736f757263657302000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d0000006465706f7369745f62617463680d00000044726f70416c6c50726f6f6673000000000e0000005075626c6973685061636b616765010000003007d200000010020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c10000000000a00000043616c6c4d6574686f6403000000811b0000000292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c12000000636f6d706c6963617465645f6d6574686f6430076f0000001002000000a120000000000064a7b3b6e00d000000000000000000000000000000000000000000000000a2400000000000000000000000023ed47ec9da71dcda2f4fb5e9f37fd2079e3000000000000000000000000000000000000000000000000000000000000000000000000000"
}
```
</details>

### Decompile Transaction Intent

| Function Name | `decompile_transaction_intent` |
| ------------- | :----------------- |
| Functionality | This function does the opposite of the [`compile_transaction_intent`](#compile-transaction-intent) function. It takes in a compiled transaction intent and decompiles it into its human-readable / machine-readable format. |
| Request Type  | `DecompileTransactionIntentRequest` |
| Response Type | `DecompileTransactionIntentResponse` |

<details>
    <summary>Request Example</summary>
  
```json
{
    "manifest_instructions_output_format": "String",
    "compiled_intent": "10020000001009000000070107f20a00000000000000000a20000000000000000a00000000000000009121000000031c3796382de8e6e7a1aacb069221e43943af8be417d4c8c92dca7c4b07f9396901000900000000090000000010010000003011130000000a00000043616c6c4d6574686f6403000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1200000077697468647261775f62795f616d6f756e7430074a0000001002000000a1200000000000f44482916345000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000041700000054616b6546726f6d576f726b746f704279416d6f756e7402000000a1200000000000c84e676dc11b000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000040a00000043616c6c4d6574686f6403000000811b0000000292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b0000006275795f67756d62616c6c30070e0000001001000000b104000000000200001d000000417373657274576f726b746f70436f6e7461696e734279416d6f756e7402000000a12000000000002cf61a24a229000000000000000000000000000000000000000000000000b61b00000000000000000000000000000000000000000000000000000000000415000000417373657274576f726b746f70436f6e7461696e7301000000b61b00000000aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510f00000054616b6546726f6d576f726b746f7001000000b61b0000000000000000000000000000000000000000000000000000000000041500000043726561746550726f6f6646726f6d4275636b65740100000009010200000a000000436c6f6e6550726f6f660100000009020200000900000044726f7050726f6f660100000009020200000900000044726f7050726f6f660100000009030200000a00000043616c6c4d6574686f6403000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c160000006372656174655f70726f6f665f62795f616d6f756e7430074a0000001002000000a1200000000000f44482916345000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000040f000000506f7046726f6d417574685a6f6e65000000000900000044726f7050726f6f660100000009040200000f00000052657475726e546f576f726b746f700100000009010200001400000054616b6546726f6d576f726b746f7042794964730200000031b402000000050000000905000000050000000907000000b61b0000000000000000000000000000000000000000000000000000000000041a00000043616c6c4d6574686f6457697468416c6c5265736f757263657302000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d0000006465706f7369745f62617463680d00000044726f70416c6c50726f6f6673000000000e0000005075626c6973685061636b616765010000003007d200000010020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c10000000000a00000043616c6c4d6574686f6403000000811b0000000292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c12000000636f6d706c6963617465645f6d6574686f6430076f0000001002000000a120000000000064a7b3b6e00d000000000000000000000000000000000000000000000000a2400000000000000000000000023ed47ec9da71dcda2f4fb5e9f37fd2079e3000000000000000000000000000000000000000000000000000000000000000000000000000"
}
```
</details>

<details>
    <summary>Response Example</summary>
  
```json
{
    "header": {
        "version": 1,
        "network_id": 242,
        "start_epoch_inclusive": 0,
        "end_epoch_exclusive": 32,
        "nonce": 0,
        "notary_public_key": "031c3796382de8e6e7a1aacb069221e43943af8be417d4c8c92dca7c4b07f93969",
        "notary_as_signatory": false,
        "cost_unit_limit": 0,
        "tip_percentage": 0
    },
    "manifest": {
        "type": "String",
        "value": "CALL_METHOD ComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\") \"withdraw_by_amount\" Decimal(\"5\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nTAKE_FROM_WORKTOP_BY_AMOUNT Decimal(\"2\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\") Bucket(\"bucket1\");\nCALL_METHOD ComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\") \"buy_gumball\" Bucket(\"bucket1\");\nASSERT_WORKTOP_CONTAINS_BY_AMOUNT Decimal(\"3\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nASSERT_WORKTOP_CONTAINS ResourceAddress(\"resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6\");\nTAKE_FROM_WORKTOP ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\") Bucket(\"bucket2\");\nCREATE_PROOF_FROM_BUCKET Bucket(\"bucket2\") Proof(\"proof1\");\nCLONE_PROOF Proof(\"proof1\") Proof(\"proof2\");\nDROP_PROOF Proof(\"proof1\");\nDROP_PROOF Proof(\"proof2\");\nCALL_METHOD ComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\") \"create_proof_by_amount\" Decimal(\"5\") ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nPOP_FROM_AUTH_ZONE Proof(\"proof3\");\nDROP_PROOF Proof(\"proof3\");\nRETURN_TO_WORKTOP Bucket(\"bucket2\");\nTAKE_FROM_WORKTOP_BY_IDS Set<NonFungibleId>(NonFungibleId(\"0905000000\"), NonFungibleId(\"0907000000\")) ResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\") Bucket(\"bucket3\");\nCALL_METHOD_WITH_ALL_RESOURCES ComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\") \"deposit_batch\";\nDROP_ALL_PROOFS;\nPUBLISH_PACKAGE Bytes(\"10020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c1000000000\");\nCALL_METHOD ComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\") \"complicated_method\" Decimal(\"1\") PreciseDecimal(\"2\");\n"
    }
}
```
</details>

### Compile Signed Transaction Intent

| Function Name | `compile_signed_transaction_intent` |
| ------------- | :----------------- |
| Functionality | This function takes in a raw transaction intent as well as its signatures and compiles it. This is useful when a notary wishes to notarize a signed transaction intent. |
| Request Type  | `CompileSignedTransactionIntentRequest` |
| Response Type | `CompileSignedTransactionIntentResponse` |

<details>
    <summary>Request Example</summary>
  
```json
{
    "transaction_intent": {
        "header": {
            "version": 1,
            "network_id": 242,
            "start_epoch_inclusive": 0,
            "end_epoch_exclusive": 32,
            "nonce": 0,
            "notary_public_key": "031c3796382de8e6e7a1aacb069221e43943af8be417d4c8c92dca7c4b07f93969",
            "notary_as_signatory": false,
            "cost_unit_limit": 0,
            "tip_percentage": 0
        },
        "manifest": {
            "type": "String",
            "value": "# Withdraw XRD from account\nCALL_METHOD\n\tComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\")\n\t\"withdraw_by_amount\"\n\tDecimal(\"5.0\")\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\n\n# Buy GUM with XRD\nTAKE_FROM_WORKTOP_BY_AMOUNT\n\tDecimal(\"2.0\")\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n\tBucket(\"xrd\");\nCALL_METHOD\n\tComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\")\n\t\"buy_gumball\"\n\tBucket(\"xrd\");\nASSERT_WORKTOP_CONTAINS_BY_AMOUNT\n\tDecimal(\"3.0\")\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nASSERT_WORKTOP_CONTAINS\n\tResourceAddress(\"resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6\");\n\n# Create a proof from bucket, clone it and drop both\nTAKE_FROM_WORKTOP\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n\tBucket(\"1u32\");\nCREATE_PROOF_FROM_BUCKET\n\tBucket(\"1u32\")\n\tProof(\"proof1\");\nCLONE_PROOF\n\tProof(\"proof1\")\n\tProof(\"proof2\");\nDROP_PROOF\n\tProof(\"proof1\");\nDROP_PROOF\n\tProof(\"proof2\");\n\n# Create a proof from account and drop it\nCALL_METHOD\n\tComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\")\n\t\"create_proof_by_amount\"\n\tDecimal(\"5.0\")\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nPOP_FROM_AUTH_ZONE\n\tProof(\"proof3\");\nDROP_PROOF\n\tProof(\"proof3\");\n\n# Return a bucket to worktop\nRETURN_TO_WORKTOP\n\tBucket(\"1u32\");\nTAKE_FROM_WORKTOP_BY_IDS\n\tSet<NonFungibleId>(NonFungibleId(\"0905000000\"),NonFungibleId(\"0907000000\"))\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n\tBucket(\"nfts\");\n\n# Cancel all buckets and move resources to account\nCALL_METHOD_WITH_ALL_RESOURCES\n\tComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\")\n\t\"deposit_batch\";\n\n# Drop all proofs\nDROP_ALL_PROOFS;\n\n# Two ways of publishing package through manifest\nPUBLISH_PACKAGE\n\tBytes(\"10020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c1000000000\");\n\n# Complicated method that takes all of the number types\nCALL_METHOD\n\tComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\")\n\t\"complicated_method\"\n\tDecimal(\"1\")\n\tPreciseDecimal(\"2\");"
        }
    },
    "signatures": [
        {
            "public_key": "02e6dc9dc1d1b418aab0a30089623f2dd4bab8f1ee9262a3ac0340cddb98b94437",
            "signature": "437849b37df54d58e49b37e0004fb92a7133491efdc5332e7ace2d69f6f73df5636bb4f65943bdb59facd400dc06e2b1386a19327c8e85ea5e404ef502ef6fbb"
        },
        {
            "public_key": "02b2bb8940c7acb5a221c4efab35b1c77afaddc611106ff4625e31d45cc17c4225",
            "signature": "c215331bb6319fd26dee491824345375e9d8936cd69d6645c8e06222bcdae4b061305e640781b84b63cb23c719eb0edb43fd2e3df04b5990434cff44a02a85a4"
        },
        {
            "public_key": "03e0366ffa0c2db669c4a7b671109309a25eb1511350aa34bf22415bc03d57fc12",
            "signature": "8c6b942f5ef8d81116120102fb4f22796c00a1aea556082d1af1d8529733e7124582b49c2bdc1545cb9d90bbdbedcf8185d16d6df5d76584cdf12688e07d919b"
        },
        {
            "public_key": "0362d5d7de2ddb98e35ac84e59c4767e1d024a95f68893af469c361f74ea08fa49",
            "signature": "ba40502e403bd49db22369a9b93cbd08bf9ab10b643b237cf9ee40bb931c12d927faad0c6a5ea6a334b1432b7047f5f8293f06fc37d229b560f50f71b84b9768"
        }
    ]
}
```
</details>

<details>
    <summary>Response Example</summary>
  
```json
{
    "compiled_signed_intent": "100200000010020000001009000000070107f20a00000000000000000a20000000000000000a00000000000000009121000000031c3796382de8e6e7a1aacb069221e43943af8be417d4c8c92dca7c4b07f9396901000900000000090000000010010000003011130000000a00000043616c6c4d6574686f6403000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1200000077697468647261775f62795f616d6f756e7430074a0000001002000000a1200000000000f44482916345000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000041700000054616b6546726f6d576f726b746f704279416d6f756e7402000000a1200000000000c84e676dc11b000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000040a00000043616c6c4d6574686f6403000000811b0000000292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b0000006275795f67756d62616c6c30070e0000001001000000b104000000000200001d000000417373657274576f726b746f70436f6e7461696e734279416d6f756e7402000000a12000000000002cf61a24a229000000000000000000000000000000000000000000000000b61b00000000000000000000000000000000000000000000000000000000000415000000417373657274576f726b746f70436f6e7461696e7301000000b61b00000000aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510f00000054616b6546726f6d576f726b746f7001000000b61b0000000000000000000000000000000000000000000000000000000000041500000043726561746550726f6f6646726f6d4275636b65740100000009010200000a000000436c6f6e6550726f6f660100000009020200000900000044726f7050726f6f660100000009020200000900000044726f7050726f6f660100000009030200000a00000043616c6c4d6574686f6403000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c160000006372656174655f70726f6f665f62795f616d6f756e7430074a0000001002000000a1200000000000f44482916345000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000040f000000506f7046726f6d417574685a6f6e65000000000900000044726f7050726f6f660100000009040200000f00000052657475726e546f576f726b746f700100000009010200001400000054616b6546726f6d576f726b746f7042794964730200000031b402000000050000000905000000050000000907000000b61b0000000000000000000000000000000000000000000000000000000000041a00000043616c6c4d6574686f6457697468416c6c5265736f757263657302000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d0000006465706f7369745f62617463680d00000044726f70416c6c50726f6f6673000000000e0000005075626c6973685061636b616765010000003007d200000010020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c10000000000a00000043616c6c4d6574686f6403000000811b0000000292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c12000000636f6d706c6963617465645f6d6574686f6430076f0000001002000000a120000000000064a7b3b6e00d000000000000000000000000000000000000000000000000a2400000000000000000000000023ed47ec9da71dcda2f4fb5e9f37fd2079e300000000000000000000000000000000000000000000000000000000000000000000000000030210400000002000000912100000002e6dc9dc1d1b418aab0a30089623f2dd4bab8f1ee9262a3ac0340cddb98b944379240000000437849b37df54d58e49b37e0004fb92a7133491efdc5332e7ace2d69f6f73df5636bb4f65943bdb59facd400dc06e2b1386a19327c8e85ea5e404ef502ef6fbb02000000912100000002b2bb8940c7acb5a221c4efab35b1c77afaddc611106ff4625e31d45cc17c42259240000000c215331bb6319fd26dee491824345375e9d8936cd69d6645c8e06222bcdae4b061305e640781b84b63cb23c719eb0edb43fd2e3df04b5990434cff44a02a85a402000000912100000003e0366ffa0c2db669c4a7b671109309a25eb1511350aa34bf22415bc03d57fc1292400000008c6b942f5ef8d81116120102fb4f22796c00a1aea556082d1af1d8529733e7124582b49c2bdc1545cb9d90bbdbedcf8185d16d6df5d76584cdf12688e07d919b0200000091210000000362d5d7de2ddb98e35ac84e59c4767e1d024a95f68893af469c361f74ea08fa499240000000ba40502e403bd49db22369a9b93cbd08bf9ab10b643b237cf9ee40bb931c12d927faad0c6a5ea6a334b1432b7047f5f8293f06fc37d229b560f50f71b84b9768"
}
```
</details>

### Decompile Signed Transaction Intent

| Function Name | `decompile_signed_transaction_intent` |
| ------------- | :----------------- |
| Functionality | This function does the opposite of the [`compile_signed_transaction_intent`](#compile-signed-transaction-intent) function. This function takes in a compiled signed transaction intent and decompiles it into its transaction intent and signatures. |
| Request Type  | `CompileSignedTransactionIntentRequest` |
| Response Type | `CompileSignedTransactionIntentResponse` |

<details>
    <summary>Request Example</summary>
  
```json
{
    "manifest_instructions_output_format": "JSON",
    "compiled_signed_intent": "100200000010020000001009000000070107f20a00000000000000000a20000000000000000a00000000000000009121000000031c3796382de8e6e7a1aacb069221e43943af8be417d4c8c92dca7c4b07f9396901000900000000090000000010010000003011130000000a00000043616c6c4d6574686f6403000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1200000077697468647261775f62795f616d6f756e7430074a0000001002000000a1200000000000f44482916345000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000041700000054616b6546726f6d576f726b746f704279416d6f756e7402000000a1200000000000c84e676dc11b000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000040a00000043616c6c4d6574686f6403000000811b0000000292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b0000006275795f67756d62616c6c30070e0000001001000000b104000000000200001d000000417373657274576f726b746f70436f6e7461696e734279416d6f756e7402000000a12000000000002cf61a24a229000000000000000000000000000000000000000000000000b61b00000000000000000000000000000000000000000000000000000000000415000000417373657274576f726b746f70436f6e7461696e7301000000b61b00000000aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510f00000054616b6546726f6d576f726b746f7001000000b61b0000000000000000000000000000000000000000000000000000000000041500000043726561746550726f6f6646726f6d4275636b65740100000009010200000a000000436c6f6e6550726f6f660100000009020200000900000044726f7050726f6f660100000009020200000900000044726f7050726f6f660100000009030200000a00000043616c6c4d6574686f6403000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c160000006372656174655f70726f6f665f62795f616d6f756e7430074a0000001002000000a1200000000000f44482916345000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000040f000000506f7046726f6d417574685a6f6e65000000000900000044726f7050726f6f660100000009040200000f00000052657475726e546f576f726b746f700100000009010200001400000054616b6546726f6d576f726b746f7042794964730200000031b402000000050000000905000000050000000907000000b61b0000000000000000000000000000000000000000000000000000000000041a00000043616c6c4d6574686f6457697468416c6c5265736f757263657302000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d0000006465706f7369745f62617463680d00000044726f70416c6c50726f6f6673000000000e0000005075626c6973685061636b616765010000003007d200000010020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c10000000000a00000043616c6c4d6574686f6403000000811b0000000292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c12000000636f6d706c6963617465645f6d6574686f6430076f0000001002000000a120000000000064a7b3b6e00d000000000000000000000000000000000000000000000000a2400000000000000000000000023ed47ec9da71dcda2f4fb5e9f37fd2079e300000000000000000000000000000000000000000000000000000000000000000000000000030210400000002000000912100000002e6dc9dc1d1b418aab0a30089623f2dd4bab8f1ee9262a3ac0340cddb98b944379240000000437849b37df54d58e49b37e0004fb92a7133491efdc5332e7ace2d69f6f73df5636bb4f65943bdb59facd400dc06e2b1386a19327c8e85ea5e404ef502ef6fbb02000000912100000002b2bb8940c7acb5a221c4efab35b1c77afaddc611106ff4625e31d45cc17c42259240000000c215331bb6319fd26dee491824345375e9d8936cd69d6645c8e06222bcdae4b061305e640781b84b63cb23c719eb0edb43fd2e3df04b5990434cff44a02a85a402000000912100000003e0366ffa0c2db669c4a7b671109309a25eb1511350aa34bf22415bc03d57fc1292400000008c6b942f5ef8d81116120102fb4f22796c00a1aea556082d1af1d8529733e7124582b49c2bdc1545cb9d90bbdbedcf8185d16d6df5d76584cdf12688e07d919b0200000091210000000362d5d7de2ddb98e35ac84e59c4767e1d024a95f68893af469c361f74ea08fa499240000000ba40502e403bd49db22369a9b93cbd08bf9ab10b643b237cf9ee40bb931c12d927faad0c6a5ea6a334b1432b7047f5f8293f06fc37d229b560f50f71b84b9768"
}
```
</details>

<details>
    <summary>Response Example</summary>
  
```json
{
    "transaction_intent": {
        "header": {
            "version": 1,
            "network_id": 242,
            "start_epoch_inclusive": 0,
            "end_epoch_exclusive": 32,
            "nonce": 0,
            "notary_public_key": "031c3796382de8e6e7a1aacb069221e43943af8be417d4c8c92dca7c4b07f93969",
            "notary_as_signatory": false,
            "cost_unit_limit": 0,
            "tip_percentage": 0
        },
        "manifest": {
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
                            "value": "0905000000"
                        },
                        {
                            "type": "NonFungibleId",
                            "value": "0907000000"
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
                    "instruction": "CALL_METHOD_WITH_ALL_RESOURCES",
                    "component_address": {
                        "type": "ComponentAddress",
                        "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
                    },
                    "method": {
                        "type": "String",
                        "value": "deposit_batch"
                    }
                },
                {
                    "instruction": "DROP_ALL_PROOFS"
                },
                {
                    "instruction": "PUBLISH_PACKAGE",
                    "package": {
                        "type": "Bytes",
                        "value": "10020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c1000000000"
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
                }
            ]
        }
    },
    "signatures": [
        {
            "public_key": "02e6dc9dc1d1b418aab0a30089623f2dd4bab8f1ee9262a3ac0340cddb98b94437",
            "signature": "437849b37df54d58e49b37e0004fb92a7133491efdc5332e7ace2d69f6f73df5636bb4f65943bdb59facd400dc06e2b1386a19327c8e85ea5e404ef502ef6fbb"
        },
        {
            "public_key": "02b2bb8940c7acb5a221c4efab35b1c77afaddc611106ff4625e31d45cc17c4225",
            "signature": "c215331bb6319fd26dee491824345375e9d8936cd69d6645c8e06222bcdae4b061305e640781b84b63cb23c719eb0edb43fd2e3df04b5990434cff44a02a85a4"
        },
        {
            "public_key": "03e0366ffa0c2db669c4a7b671109309a25eb1511350aa34bf22415bc03d57fc12",
            "signature": "8c6b942f5ef8d81116120102fb4f22796c00a1aea556082d1af1d8529733e7124582b49c2bdc1545cb9d90bbdbedcf8185d16d6df5d76584cdf12688e07d919b"
        },
        {
            "public_key": "0362d5d7de2ddb98e35ac84e59c4767e1d024a95f68893af469c361f74ea08fa49",
            "signature": "ba40502e403bd49db22369a9b93cbd08bf9ab10b643b237cf9ee40bb931c12d927faad0c6a5ea6a334b1432b7047f5f8293f06fc37d229b560f50f71b84b9768"
        }
    ]
}
```
</details>

### Compile Notarized Transaction Intent

| Function Name | `compile_notarized_transaction_intent` |
| ------------- | :----------------- |
| Functionality | This function takes in a raw signed transaction intent as well as the notary signature and compiles it. This is useful when we wish to submit a transaction to the Gateway API |
| Request Type  | `CompileNotarizedTransactionIntentRequest` |
| Response Type | `CompileNotarizedTransactionIntentResponse` |

<details>
    <summary>Request Example</summary>
  
```json
{
    "signed_intent": {
        "transaction_intent": {
            "header": {
                "version": 1,
                "network_id": 242,
                "start_epoch_inclusive": 0,
                "end_epoch_exclusive": 32,
                "nonce": 0,
                "notary_public_key": "031c3796382de8e6e7a1aacb069221e43943af8be417d4c8c92dca7c4b07f93969",
                "notary_as_signatory": false,
                "cost_unit_limit": 0,
                "tip_percentage": 0
            },
            "manifest": {
                "type": "String",
                "value": "# Withdraw XRD from account\nCALL_METHOD\n\tComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\")\n\t\"withdraw_by_amount\"\n\tDecimal(\"5.0\")\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\n\n# Buy GUM with XRD\nTAKE_FROM_WORKTOP_BY_AMOUNT\n\tDecimal(\"2.0\")\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n\tBucket(\"xrd\");\nCALL_METHOD\n\tComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\")\n\t\"buy_gumball\"\n\tBucket(\"xrd\");\nASSERT_WORKTOP_CONTAINS_BY_AMOUNT\n\tDecimal(\"3.0\")\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nASSERT_WORKTOP_CONTAINS\n\tResourceAddress(\"resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6\");\n\n# Create a proof from bucket, clone it and drop both\nTAKE_FROM_WORKTOP\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n\tBucket(\"1u32\");\nCREATE_PROOF_FROM_BUCKET\n\tBucket(\"1u32\")\n\tProof(\"proof1\");\nCLONE_PROOF\n\tProof(\"proof1\")\n\tProof(\"proof2\");\nDROP_PROOF\n\tProof(\"proof1\");\nDROP_PROOF\n\tProof(\"proof2\");\n\n# Create a proof from account and drop it\nCALL_METHOD\n\tComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\")\n\t\"create_proof_by_amount\"\n\tDecimal(\"5.0\")\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\");\nPOP_FROM_AUTH_ZONE\n\tProof(\"proof3\");\nDROP_PROOF\n\tProof(\"proof3\");\n\n# Return a bucket to worktop\nRETURN_TO_WORKTOP\n\tBucket(\"1u32\");\nTAKE_FROM_WORKTOP_BY_IDS\n\tSet<NonFungibleId>(NonFungibleId(\"0905000000\"),NonFungibleId(\"0907000000\"))\n\tResourceAddress(\"resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag\")\n\tBucket(\"nfts\");\n\n# Cancel all buckets and move resources to account\nCALL_METHOD_WITH_ALL_RESOURCES\n\tComponentAddress(\"account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064\")\n\t\"deposit_batch\";\n\n# Drop all proofs\nDROP_ALL_PROOFS;\n\n# Two ways of publishing package through manifest\nPUBLISH_PACKAGE\n\tBytes(\"10020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c1000000000\");\n\n# Complicated method that takes all of the number types\nCALL_METHOD\n\tComponentAddress(\"component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum\")\n\t\"complicated_method\"\n\tDecimal(\"1\")\n\tPreciseDecimal(\"2\");"
            }
        },
        "signatures": [
            {
                "public_key": "02e6dc9dc1d1b418aab0a30089623f2dd4bab8f1ee9262a3ac0340cddb98b94437",
                "signature": "437849b37df54d58e49b37e0004fb92a7133491efdc5332e7ace2d69f6f73df5636bb4f65943bdb59facd400dc06e2b1386a19327c8e85ea5e404ef502ef6fbb"
            },
            {
                "public_key": "02b2bb8940c7acb5a221c4efab35b1c77afaddc611106ff4625e31d45cc17c4225",
                "signature": "c215331bb6319fd26dee491824345375e9d8936cd69d6645c8e06222bcdae4b061305e640781b84b63cb23c719eb0edb43fd2e3df04b5990434cff44a02a85a4"
            },
            {
                "public_key": "03e0366ffa0c2db669c4a7b671109309a25eb1511350aa34bf22415bc03d57fc12",
                "signature": "8c6b942f5ef8d81116120102fb4f22796c00a1aea556082d1af1d8529733e7124582b49c2bdc1545cb9d90bbdbedcf8185d16d6df5d76584cdf12688e07d919b"
            },
            {
                "public_key": "0362d5d7de2ddb98e35ac84e59c4767e1d024a95f68893af469c361f74ea08fa49",
                "signature": "ba40502e403bd49db22369a9b93cbd08bf9ab10b643b237cf9ee40bb931c12d927faad0c6a5ea6a334b1432b7047f5f8293f06fc37d229b560f50f71b84b9768"
            }
        ]
    },
    "notary_signature": "e56f71f9730f0f5a4bed57b4ccb8f943aa156b3bb4fe6ec84972cecc0adaafd84bc71f92e251d1e484eb8af442d6234cfcc098b62f9770ed419a45d1cc272f6c"
}
```
</details>

<details>
    <summary>Response Example</summary>
  
```json
{
    "compiled_notarized_intent": "1002000000100200000010020000001009000000070107f20a00000000000000000a20000000000000000a00000000000000009121000000031c3796382de8e6e7a1aacb069221e43943af8be417d4c8c92dca7c4b07f9396901000900000000090000000010010000003011130000000a00000043616c6c4d6574686f6403000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1200000077697468647261775f62795f616d6f756e7430074a0000001002000000a1200000000000f44482916345000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000041700000054616b6546726f6d576f726b746f704279416d6f756e7402000000a1200000000000c84e676dc11b000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000040a00000043616c6c4d6574686f6403000000811b0000000292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b0000006275795f67756d62616c6c30070e0000001001000000b104000000000200001d000000417373657274576f726b746f70436f6e7461696e734279416d6f756e7402000000a12000000000002cf61a24a229000000000000000000000000000000000000000000000000b61b00000000000000000000000000000000000000000000000000000000000415000000417373657274576f726b746f70436f6e7461696e7301000000b61b00000000aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510f00000054616b6546726f6d576f726b746f7001000000b61b0000000000000000000000000000000000000000000000000000000000041500000043726561746550726f6f6646726f6d4275636b65740100000009010200000a000000436c6f6e6550726f6f660100000009020200000900000044726f7050726f6f660100000009020200000900000044726f7050726f6f660100000009030200000a00000043616c6c4d6574686f6403000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c160000006372656174655f70726f6f665f62795f616d6f756e7430074a0000001002000000a1200000000000f44482916345000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000040f000000506f7046726f6d417574685a6f6e65000000000900000044726f7050726f6f660100000009040200000f00000052657475726e546f576f726b746f700100000009010200001400000054616b6546726f6d576f726b746f7042794964730200000031b402000000050000000905000000050000000907000000b61b0000000000000000000000000000000000000000000000000000000000041a00000043616c6c4d6574686f6457697468416c6c5265736f757263657302000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d0000006465706f7369745f62617463680d00000044726f70416c6c50726f6f6673000000000e0000005075626c6973685061636b616765010000003007d200000010020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c10000000000a00000043616c6c4d6574686f6403000000811b0000000292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c12000000636f6d706c6963617465645f6d6574686f6430076f0000001002000000a120000000000064a7b3b6e00d000000000000000000000000000000000000000000000000a2400000000000000000000000023ed47ec9da71dcda2f4fb5e9f37fd2079e300000000000000000000000000000000000000000000000000000000000000000000000000030210400000002000000912100000002e6dc9dc1d1b418aab0a30089623f2dd4bab8f1ee9262a3ac0340cddb98b944379240000000437849b37df54d58e49b37e0004fb92a7133491efdc5332e7ace2d69f6f73df5636bb4f65943bdb59facd400dc06e2b1386a19327c8e85ea5e404ef502ef6fbb02000000912100000002b2bb8940c7acb5a221c4efab35b1c77afaddc611106ff4625e31d45cc17c42259240000000c215331bb6319fd26dee491824345375e9d8936cd69d6645c8e06222bcdae4b061305e640781b84b63cb23c719eb0edb43fd2e3df04b5990434cff44a02a85a402000000912100000003e0366ffa0c2db669c4a7b671109309a25eb1511350aa34bf22415bc03d57fc1292400000008c6b942f5ef8d81116120102fb4f22796c00a1aea556082d1af1d8529733e7124582b49c2bdc1545cb9d90bbdbedcf8185d16d6df5d76584cdf12688e07d919b0200000091210000000362d5d7de2ddb98e35ac84e59c4767e1d024a95f68893af469c361f74ea08fa499240000000ba40502e403bd49db22369a9b93cbd08bf9ab10b643b237cf9ee40bb931c12d927faad0c6a5ea6a334b1432b7047f5f8293f06fc37d229b560f50f71b84b97689240000000e56f71f9730f0f5a4bed57b4ccb8f943aa156b3bb4fe6ec84972cecc0adaafd84bc71f92e251d1e484eb8af442d6234cfcc098b62f9770ed419a45d1cc272f6c"
}
```
</details>

### Decompile Notarized Transaction Intent

| Function Name | `decompile_notarized_transaction_intent` |
| ------------- | :----------------- |
| Functionality | This function does the opposite of the [`compile_notarized_transaction_intent`](#compile-notarized-transaction-intent) function. This function takes in a compiled notarized transaction intent and decompiles it into its signed transaction intent and notary signature. |
| Request Type  | `DecompileNotarizedTransactionIntentRequest` |
| Response Type | `DecompileNotarizedTransactionIntentResponse` |

<details>
    <summary>Request Example</summary>
  
```json
{
    "manifest_instructions_output_format": "JSON",
    "compiled_notarized_intent": "1002000000100200000010020000001009000000070107f20a00000000000000000a20000000000000000a00000000000000009121000000031c3796382de8e6e7a1aacb069221e43943af8be417d4c8c92dca7c4b07f9396901000900000000090000000010010000003011130000000a00000043616c6c4d6574686f6403000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1200000077697468647261775f62795f616d6f756e7430074a0000001002000000a1200000000000f44482916345000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000041700000054616b6546726f6d576f726b746f704279416d6f756e7402000000a1200000000000c84e676dc11b000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000040a00000043616c6c4d6574686f6403000000811b0000000292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b0000006275795f67756d62616c6c30070e0000001001000000b104000000000200001d000000417373657274576f726b746f70436f6e7461696e734279416d6f756e7402000000a12000000000002cf61a24a229000000000000000000000000000000000000000000000000b61b00000000000000000000000000000000000000000000000000000000000415000000417373657274576f726b746f70436f6e7461696e7301000000b61b00000000aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510f00000054616b6546726f6d576f726b746f7001000000b61b0000000000000000000000000000000000000000000000000000000000041500000043726561746550726f6f6646726f6d4275636b65740100000009010200000a000000436c6f6e6550726f6f660100000009020200000900000044726f7050726f6f660100000009020200000900000044726f7050726f6f660100000009030200000a00000043616c6c4d6574686f6403000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c160000006372656174655f70726f6f665f62795f616d6f756e7430074a0000001002000000a1200000000000f44482916345000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000040f000000506f7046726f6d417574685a6f6e65000000000900000044726f7050726f6f660100000009040200000f00000052657475726e546f576f726b746f700100000009010200001400000054616b6546726f6d576f726b746f7042794964730200000031b402000000050000000905000000050000000907000000b61b0000000000000000000000000000000000000000000000000000000000041a00000043616c6c4d6574686f6457697468416c6c5265736f757263657302000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d0000006465706f7369745f62617463680d00000044726f70416c6c50726f6f6673000000000e0000005075626c6973685061636b616765010000003007d200000010020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c10000000000a00000043616c6c4d6574686f6403000000811b0000000292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c12000000636f6d706c6963617465645f6d6574686f6430076f0000001002000000a120000000000064a7b3b6e00d000000000000000000000000000000000000000000000000a2400000000000000000000000023ed47ec9da71dcda2f4fb5e9f37fd2079e300000000000000000000000000000000000000000000000000000000000000000000000000030210400000002000000912100000002e6dc9dc1d1b418aab0a30089623f2dd4bab8f1ee9262a3ac0340cddb98b944379240000000437849b37df54d58e49b37e0004fb92a7133491efdc5332e7ace2d69f6f73df5636bb4f65943bdb59facd400dc06e2b1386a19327c8e85ea5e404ef502ef6fbb02000000912100000002b2bb8940c7acb5a221c4efab35b1c77afaddc611106ff4625e31d45cc17c42259240000000c215331bb6319fd26dee491824345375e9d8936cd69d6645c8e06222bcdae4b061305e640781b84b63cb23c719eb0edb43fd2e3df04b5990434cff44a02a85a402000000912100000003e0366ffa0c2db669c4a7b671109309a25eb1511350aa34bf22415bc03d57fc1292400000008c6b942f5ef8d81116120102fb4f22796c00a1aea556082d1af1d8529733e7124582b49c2bdc1545cb9d90bbdbedcf8185d16d6df5d76584cdf12688e07d919b0200000091210000000362d5d7de2ddb98e35ac84e59c4767e1d024a95f68893af469c361f74ea08fa499240000000ba40502e403bd49db22369a9b93cbd08bf9ab10b643b237cf9ee40bb931c12d927faad0c6a5ea6a334b1432b7047f5f8293f06fc37d229b560f50f71b84b97689240000000e56f71f9730f0f5a4bed57b4ccb8f943aa156b3bb4fe6ec84972cecc0adaafd84bc71f92e251d1e484eb8af442d6234cfcc098b62f9770ed419a45d1cc272f6c"
}
```
</details>

<details>
    <summary>Response Example</summary>
  
```json
{
    "signed_intent": {
        "transaction_intent": {
            "header": {
                "version": 1,
                "network_id": 242,
                "start_epoch_inclusive": 0,
                "end_epoch_exclusive": 32,
                "nonce": 0,
                "notary_public_key": "031c3796382de8e6e7a1aacb069221e43943af8be417d4c8c92dca7c4b07f93969",
                "notary_as_signatory": false,
                "cost_unit_limit": 0,
                "tip_percentage": 0
            },
            "manifest": {
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
                                "value": "0907000000"
                            },
                            {
                                "type": "NonFungibleId",
                                "value": "0905000000"
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
                        "instruction": "CALL_METHOD_WITH_ALL_RESOURCES",
                        "component_address": {
                            "type": "ComponentAddress",
                            "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
                        },
                        "method": {
                            "type": "String",
                            "value": "deposit_batch"
                        }
                    },
                    {
                        "instruction": "DROP_ALL_PROOFS"
                    },
                    {
                        "instruction": "PUBLISH_PACKAGE",
                        "package": {
                            "type": "Bytes",
                            "value": "10020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c1000000000"
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
                    }
                ]
            }
        },
        "signatures": [
            {
                "public_key": "02e6dc9dc1d1b418aab0a30089623f2dd4bab8f1ee9262a3ac0340cddb98b94437",
                "signature": "437849b37df54d58e49b37e0004fb92a7133491efdc5332e7ace2d69f6f73df5636bb4f65943bdb59facd400dc06e2b1386a19327c8e85ea5e404ef502ef6fbb"
            },
            {
                "public_key": "02b2bb8940c7acb5a221c4efab35b1c77afaddc611106ff4625e31d45cc17c4225",
                "signature": "c215331bb6319fd26dee491824345375e9d8936cd69d6645c8e06222bcdae4b061305e640781b84b63cb23c719eb0edb43fd2e3df04b5990434cff44a02a85a4"
            },
            {
                "public_key": "03e0366ffa0c2db669c4a7b671109309a25eb1511350aa34bf22415bc03d57fc12",
                "signature": "8c6b942f5ef8d81116120102fb4f22796c00a1aea556082d1af1d8529733e7124582b49c2bdc1545cb9d90bbdbedcf8185d16d6df5d76584cdf12688e07d919b"
            },
            {
                "public_key": "0362d5d7de2ddb98e35ac84e59c4767e1d024a95f68893af469c361f74ea08fa49",
                "signature": "ba40502e403bd49db22369a9b93cbd08bf9ab10b643b237cf9ee40bb931c12d927faad0c6a5ea6a334b1432b7047f5f8293f06fc37d229b560f50f71b84b9768"
            }
        ]
    },
    "notary_signature": "e56f71f9730f0f5a4bed57b4ccb8f943aa156b3bb4fe6ec84972cecc0adaafd84bc71f92e251d1e484eb8af442d6234cfcc098b62f9770ed419a45d1cc272f6c"
}
```
</details>

### Decompile Unknown Transaction Intent

| Function Name | `decompile_unknown_transaction_intent` |
| ------------- | :----------------- |
| Functionality | There are certain cases where we might have some blob which we suspect is a transaction intent but we have no way of verifying whether that is true or not. Looking at the type id byte of the blob does not help either as it's a generic `Struct` type which is not too telling. For this specific use case, this library provides this function which attempts to decompile a transaction intent of an unknown type.  |
| Request Type  | `DecompileUnknownTransactionIntentRequest` |
| Response Type | `DecompileUnknownTransactionIntentResponse` |

<details>
    <summary>Request Example</summary>
  
```json
{
    "manifest_instructions_output_format": "JSON",
    "compiled_notarized_intent": "1002000000100200000010020000001009000000070107f20a00000000000000000a20000000000000000a00000000000000009121000000031c3796382de8e6e7a1aacb069221e43943af8be417d4c8c92dca7c4b07f9396901000900000000090000000010010000003011130000000a00000043616c6c4d6574686f6403000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c1200000077697468647261775f62795f616d6f756e7430074a0000001002000000a1200000000000f44482916345000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000041700000054616b6546726f6d576f726b746f704279416d6f756e7402000000a1200000000000c84e676dc11b000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000040a00000043616c6c4d6574686f6403000000811b0000000292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c0b0000006275795f67756d62616c6c30070e0000001001000000b104000000000200001d000000417373657274576f726b746f70436f6e7461696e734279416d6f756e7402000000a12000000000002cf61a24a229000000000000000000000000000000000000000000000000b61b00000000000000000000000000000000000000000000000000000000000415000000417373657274576f726b746f70436f6e7461696e7301000000b61b00000000aedb7960d1f87dc25138f4cd101da6c98d57323478d53c5fb9510f00000054616b6546726f6d576f726b746f7001000000b61b0000000000000000000000000000000000000000000000000000000000041500000043726561746550726f6f6646726f6d4275636b65740100000009010200000a000000436c6f6e6550726f6f660100000009020200000900000044726f7050726f6f660100000009020200000900000044726f7050726f6f660100000009030200000a00000043616c6c4d6574686f6403000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c160000006372656174655f70726f6f665f62795f616d6f756e7430074a0000001002000000a1200000000000f44482916345000000000000000000000000000000000000000000000000b61b0000000000000000000000000000000000000000000000000000000000040f000000506f7046726f6d417574685a6f6e65000000000900000044726f7050726f6f660100000009040200000f00000052657475726e546f576f726b746f700100000009010200001400000054616b6546726f6d576f726b746f7042794964730200000031b402000000050000000905000000050000000907000000b61b0000000000000000000000000000000000000000000000000000000000041a00000043616c6c4d6574686f6457697468416c6c5265736f757263657302000000811b00000003d43f479e9b2beb9df98bc3888344fc25eda181e8f710ce1bf1de0c0d0000006465706f7369745f62617463680d00000044726f70416c6c50726f6f6673000000000e0000005075626c6973685061636b616765010000003007d200000010020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c10000000000a00000043616c6c4d6574686f6403000000811b0000000292566c83de7fd6b04fcc92b5e04b03228ccff040785673278ef10c12000000636f6d706c6963617465645f6d6574686f6430076f0000001002000000a120000000000064a7b3b6e00d000000000000000000000000000000000000000000000000a2400000000000000000000000023ed47ec9da71dcda2f4fb5e9f37fd2079e300000000000000000000000000000000000000000000000000000000000000000000000000030210400000002000000912100000002e6dc9dc1d1b418aab0a30089623f2dd4bab8f1ee9262a3ac0340cddb98b944379240000000437849b37df54d58e49b37e0004fb92a7133491efdc5332e7ace2d69f6f73df5636bb4f65943bdb59facd400dc06e2b1386a19327c8e85ea5e404ef502ef6fbb02000000912100000002b2bb8940c7acb5a221c4efab35b1c77afaddc611106ff4625e31d45cc17c42259240000000c215331bb6319fd26dee491824345375e9d8936cd69d6645c8e06222bcdae4b061305e640781b84b63cb23c719eb0edb43fd2e3df04b5990434cff44a02a85a402000000912100000003e0366ffa0c2db669c4a7b671109309a25eb1511350aa34bf22415bc03d57fc1292400000008c6b942f5ef8d81116120102fb4f22796c00a1aea556082d1af1d8529733e7124582b49c2bdc1545cb9d90bbdbedcf8185d16d6df5d76584cdf12688e07d919b0200000091210000000362d5d7de2ddb98e35ac84e59c4767e1d024a95f68893af469c361f74ea08fa499240000000ba40502e403bd49db22369a9b93cbd08bf9ab10b643b237cf9ee40bb931c12d927faad0c6a5ea6a334b1432b7047f5f8293f06fc37d229b560f50f71b84b97689240000000e56f71f9730f0f5a4bed57b4ccb8f943aa156b3bb4fe6ec84972cecc0adaafd84bc71f92e251d1e484eb8af442d6234cfcc098b62f9770ed419a45d1cc272f6c"
}
```
</details>

<details>
    <summary>Response Example</summary>
  
```json
{
    "signed_intent": {
        "transaction_intent": {
            "header": {
                "version": 1,
                "network_id": 242,
                "start_epoch_inclusive": 0,
                "end_epoch_exclusive": 32,
                "nonce": 0,
                "notary_public_key": "031c3796382de8e6e7a1aacb069221e43943af8be417d4c8c92dca7c4b07f93969",
                "notary_as_signatory": false,
                "cost_unit_limit": 0,
                "tip_percentage": 0
            },
            "manifest": {
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
                                "value": "0907000000"
                            },
                            {
                                "type": "NonFungibleId",
                                "value": "0905000000"
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
                        "instruction": "CALL_METHOD_WITH_ALL_RESOURCES",
                        "component_address": {
                            "type": "ComponentAddress",
                            "address": "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"
                        },
                        "method": {
                            "type": "String",
                            "value": "deposit_batch"
                        }
                    },
                    {
                        "instruction": "DROP_ALL_PROOFS"
                    },
                    {
                        "instruction": "PUBLISH_PACKAGE",
                        "package": {
                            "type": "Bytes",
                            "value": "10020000003007c00000000061736d010000000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072503066d656d6f727902000a5f5f646174615f656e6403010b5f5f686561705f6261736503020019046e616d65071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e35392e30202839643162323130366520323032322d30322d323329320c1000000000"
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
                    }
                ]
            }
        },
        "signatures": [
            {
                "public_key": "02e6dc9dc1d1b418aab0a30089623f2dd4bab8f1ee9262a3ac0340cddb98b94437",
                "signature": "437849b37df54d58e49b37e0004fb92a7133491efdc5332e7ace2d69f6f73df5636bb4f65943bdb59facd400dc06e2b1386a19327c8e85ea5e404ef502ef6fbb"
            },
            {
                "public_key": "02b2bb8940c7acb5a221c4efab35b1c77afaddc611106ff4625e31d45cc17c4225",
                "signature": "c215331bb6319fd26dee491824345375e9d8936cd69d6645c8e06222bcdae4b061305e640781b84b63cb23c719eb0edb43fd2e3df04b5990434cff44a02a85a4"
            },
            {
                "public_key": "03e0366ffa0c2db669c4a7b671109309a25eb1511350aa34bf22415bc03d57fc12",
                "signature": "8c6b942f5ef8d81116120102fb4f22796c00a1aea556082d1af1d8529733e7124582b49c2bdc1545cb9d90bbdbedcf8185d16d6df5d76584cdf12688e07d919b"
            },
            {
                "public_key": "0362d5d7de2ddb98e35ac84e59c4767e1d024a95f68893af469c361f74ea08fa49",
                "signature": "ba40502e403bd49db22369a9b93cbd08bf9ab10b643b237cf9ee40bb931c12d927faad0c6a5ea6a334b1432b7047f5f8293f06fc37d229b560f50f71b84b9768"
            }
        ]
    },
    "notary_signature": "e56f71f9730f0f5a4bed57b4ccb8f943aa156b3bb4fe6ec84972cecc0adaafd84bc71f92e251d1e484eb8af442d6234cfcc098b62f9770ed419a45d1cc272f6c"
}
```
</details>


### Bech32m Encode Address

| Function Name | `encode_address` |
| ------------- | :----------------- |
| Functionality | This function can be used when we have a byte array which we wish to do Bech32m encoding on. In this case, the HRP to use will be determined through the entity byte of the passed address hex string. |
| Request Type  | `EncodeAddressRequest` |
| Response Type | `EncodeAddressResponse` |

<details>
    <summary>Request Example</summary>
  
```json
{
    "network_id": 1,
    "address": "01ce8be5eb98800c15e6942d27e373599c18a19e08a53378341d2a"
}
```
</details>

<details>
    <summary>Response Example</summary>
  
```json
{
    "type": "PackageAddress",
    "address": "package_rdx1q88ghe0tnzqqc90xjskj0cmntxwp3gv7pzjnx7p5r54q5e29qu"
}
```
</details>


### Bech32m Decode Address

| Function Name | `decode_address` |
| ------------- | :----------------- |
| Functionality | This function can be used to decode a Bech32m encoded address string into its equivalent hrp and data. In addition to that, this function provides other useful information on the address such as the network id and name that it is used for, and the entity type of the address. |
| Request Type  | `DecodeAddressRequest` |
| Response Type | `DecodeAddressResponse` |

<details>
    <summary>Request Example</summary>
  
```json
{
    "address": "package_rdx1q88ghe0tnzqqc90xjskj0cmntxwp3gv7pzjnx7p5r54q5e29qu"
}
```
</details>

<details>
    <summary>Response Example</summary>
  
```json
{
    "network_id": 1,
    "network_name": "mainnet",
    "entity_type": "Package",
    "data": "01ce8be5eb98800c15e6942d27e373599c18a19e08a53378341d2a",
    "hrp": "package_rdx",
    "address": {
        "type": "PackageAddress",
        "address": "package_rdx1q88ghe0tnzqqc90xjskj0cmntxwp3gv7pzjnx7p5r54q5e29qu"
    }
}
```
</details>

### SBOR Encode Value

| Function Name | `sbor_encode` |
| ------------- | :----------------- |
| Functionality | This function takes in a `Value` and encodes it in SBOR. |
| Request Type  | `SBOREncodeRequest` |
| Response Type | `SBOREncodeResponse` |

<details>
    <summary>Request Example</summary>
  
```json
{
    "type": "Map",
    "key_type": "String",
    "value_type": "Decimal",
    "elements": [
        {
            "type": "String",
            "value": "Toyota Camry"
        },
        {
            "type": "Decimal",
            "value": "80000"
        },
        
        {
            "type": "String",
            "value": "Ford Raptor"
        },
        {
            "type": "Decimal",
            "value": "170000"
        }
    ]
}
```
</details>

<details>
    <summary>Response Example</summary>
  
```json
{
    "encoded_value": "320ca1020000000c000000546f796f74612043616d72792000000000000092d54d06cff010000000000000000000000000000000000000000000000b000000466f726420526170746f7220000000000040d66565edb7ff2300000000000000000000000000000000000000000000"
}
```
</details>

### SBOR Decode Value

| Function Name | `sbor_decode` |
| ------------- | :----------------- |
| Functionality | This function takes in a hex string and attemps to decode it into a `Value`. |
| Request Type  | `SBORDecodeRequest` |
| Response Type | `SBORDecodeResponse` |

<details>
    <summary>Request Example</summary>
  
```json
{
    "encoded_value": "300710000000eb34e880acbd6ba9a67d0ed58d7122fa",
    "network_id": 242
}
```
</details>

<details>
    <summary>Response Example</summary>
  
```json
{
    "type": "List",
    "element_type": "U8",
    "elements": [
        {
            "type": "U8",
            "value": "235"
        },
        {
            "type": "U8",
            "value": "52"
        },
        {
            "type": "U8",
            "value": "232"
        },
        {
            "type": "U8",
            "value": "128"
        },
        {
            "type": "U8",
            "value": "172"
        },
        {
            "type": "U8",
            "value": "189"
        },
        {
            "type": "U8",
            "value": "107"
        },
        {
            "type": "U8",
            "value": "169"
        },
        {
            "type": "U8",
            "value": "166"
        },
        {
            "type": "U8",
            "value": "125"
        },
        {
            "type": "U8",
            "value": "14"
        },
        {
            "type": "U8",
            "value": "213"
        },
        {
            "type": "U8",
            "value": "141"
        },
        {
            "type": "U8",
            "value": "113"
        },
        {
            "type": "U8",
            "value": "34"
        },
        {
            "type": "U8",
            "value": "250"
        }
    ]
}
```
</details>

## Building from Source

The transaction library comes with a `build.sh` script which builds the library from source assuming that all of the required dependencies are installed. Currently, certain parts of building the transaction library require that the OS is a MacOS. More specifically, the `lipo` and `xcodebuild` tools require a MacOS operating system and are not available on other platforms. 

There are a number of dependencies required to be able to build the transaction library:

1. Make sure that you have the Rust toolchain installed and up to date

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    rustup update
    ```

2. cbindgen requires some nightly features to correctly function when expanding macros, therefore, make sure that Rust's nightly channel is installed

    ```
    rustup toolchain install nightly
    ```

3. The transaction library is build for different targets. Rustup requires that the configuration for all of these targets is installed priod to building

    ```
    rustup target add \
        wasm32-unknown-unknown \
        x86_64-apple-ios \
        aarch64-apple-ios \
        aarch64-apple-ios-sim
    ```

4. The build script relies on `cbindgen` to generate the required c-header of the transaction library. 

    ```
    cargo install cargo-lipo
    ```

5. Some of the libraries that the trasnaction library and Scrypto depend upon (mainly the Secp256k1 library) require that `llvm` and `clang` are installed and added to the path

    ```
    brew install llvm
    ```

    Relating to this point, you might need to also add the archiver's path and clang's path to your PATH. This is done through the following commands:

    ```
    export AR="/opt/homebrew/opt/llvm/bin/llvm-ar"
    export CC="/opt/homebrew/opt/llvm/bin/clang"
    ```

After the above steps, it should now be possible to build the transaction library from source. To do that, run the build script:

```
./build.sh
```