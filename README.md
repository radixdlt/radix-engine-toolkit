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
| Functionality | Clients have a need to be able to read, parse, understand, and interrogate transaction manifests to get more information on what a transactions might be doing. Transaction manifests have so far existed in one format: as strings. While the string format is very human readable, it is not easily readable by machines as a lexer and parser are needed to make sense of them; thus, it is for clients to programmatically make sense of transactions. As such, there is a need for another transaction manifest format (to supplement, **NOT** replace) which machines can easily make sense of without the need to implement a lexer and parser.</br></br>Therefore, this library introduces a JSON format for transaction manifests which clients can use when wanting to read and interrogate their transaction manifests in code. The transaction manifest JSON format has a 1:1 mapping to the string format of transaction manifests, meaning that anything which can be done in the string format of transaction manifests, can be done in the JSON format as well.</br></br>This function allows the client the convert their manifest between the two supported manifest types: string and JSON. 
| Request Type  | `ConvertManifestRequest` |
| Response Type | `ConvertManifestResponse` |

<details>
    <summary>Request Example</summary>
  
```json
{
    "transaction_version": 1,
    "network_id": 242,
    "manifest_output_format": "JSON",
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