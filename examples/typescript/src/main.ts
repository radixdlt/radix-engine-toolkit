import TransactionService from "./transaction-service";
import fs from "fs";
import {
	CompileTransactionIntentRequest,
	CompileTransactionIntentResponse,
	ConvertManifestRequest,
	ConvertManifestResponse,
	DecompileTransactionIntentRequest,
	DecompileTransactionIntentResponse,
	Manifest,
	ManifestKind,
	Signature,
	TransactionHeader,
	CompileSignedTransactionIntentRequest,
	CompileSignedTransactionIntentResponse,
	DecompileSignedTransactionIntentRequest,
	DecompileSignedTransactionIntentResponse,
	CompileNotarizedTransactionIntentRequest,
	CompileNotarizedTransactionIntentResponse,
	DecompileNotarizedTransactionIntentRequest,
	DecompileNotarizedTransactionIntentResponse,
	DecompileUnknownTransactionIntentRequest,
	DecompileUnknownTransactionIntentResponse,
} from "./interfaces";
import * as CryptoJS from "crypto-js";
import * as secp256k1 from "secp256k1";
import { ValueKind } from "./value";

const main = async (): Promise<void> => {
	// Creating a new transaction service object from the transaction service WASM file path
	const path: string = "../../target/wasm32-unknown-unknown/release/transaction_library.wasm";
	const transactionService: TransactionService = await TransactionService.fromPath(path);

	// Example 1: Printing the information of the transaction service. This is essentially the
	// "Hello World" of this project. If the information of the package is printed correctly, then
	// this means that the calls to the WASM modules are happening without any issues.
	console.log("======= Example 1 =======");
	console.log(transactionService.information());
	console.log("=========================", "\n");

	// Example 2: One of the functions that are exposed by this library is one which allows clients
	// to convert manifests from one format to another. In this example, we will read the manifest
	// file in the `examples` directory and convert it to a JSON manifest through the transaction
	// library.
	let manifestString: string = fs.readFileSync("../complex.rtm", "utf-8");
	let manifest: Manifest = {
		type: ManifestKind.String,
		value: manifestString,
	};

	let manifestConversionRequest: ConvertManifestRequest = {
		transaction_version: 1,
		network_id: 0xf2,
		manifest_output_format: ManifestKind.JSON,
		manifest,
	};
	let manifestConversionResponse: ConvertManifestResponse = transactionService.convertManifest(
		manifestConversionRequest
	) as ConvertManifestResponse;
	console.log("======= Example 2 =======");
	console.log(JSON.stringify(manifestConversionResponse, null, 4));
	console.log("=========================", "\n");

	// Example 3: When signing a transaction, the compiled intent of a transaction is what gets
	// signed. Obtaining this compiled intent requires SBOR encoding the intent and therefore
	// requires an SBOR implementation. However, this library provides the ability to compile
	// transactions without needing to implement the SBOR codec at the client.
	let transactionHeader: TransactionHeader = {
		version: 0x01,
		network_id: 0xf2,
		start_epoch_inclusive: 0x00,
		end_epoch_exclusive: 0x20,
		nonce: 0x00,
		notary_public_key: "031c3796382de8e6e7a1aacb069221e43943af8be417d4c8c92dca7c4b07f93969",
		notary_as_signatory: false,
		cost_unit_limit: 0x0,
		tip_percentage: 0x0,
	};

	let compileTransactionIntentRequest: CompileTransactionIntentRequest = {
		manifest,
		header: transactionHeader,
	};
	let compileTransactionIntentResponse: CompileTransactionIntentResponse =
		transactionService.compileTransactionIntent(
			compileTransactionIntentRequest
		) as CompileTransactionIntentResponse;
	console.log("======= Example 3 =======");
	console.log(JSON.stringify(compileTransactionIntentResponse, null, 4));
	console.log("=========================", "\n");

	// Example 4: There are certain cases where you might the compiled transaction intent and you
	// wish to understand what exactly you might be signing. In this case, you would need to
	// decompile the byte-representation of the transaction intent into something that you can
	// understand (in code or as a human).
	let decompileTransactionIntentRequest: DecompileTransactionIntentRequest = {
		compiled_intent: compileTransactionIntentResponse.compiled_intent,
		manifest_output_format: ManifestKind.String,
	};
	let decompileTransactionIntentResponse: DecompileTransactionIntentResponse =
		transactionService.decompileTransactionIntent(
			decompileTransactionIntentRequest
		) as DecompileTransactionIntentResponse;
	console.log("======= Example 4 =======");
	console.log(JSON.stringify(decompileTransactionIntentResponse, null, 4));
	console.log("=========================", "\n");

	// Example 5: In example 3, we compiled a manifest down to its SBOR bytes representation, which
	// we need when signing transactions. In this example, we will sign a transaction with multiple
	// private keys and then request a compiled signed transaction intent from the transactions API.

	// The private keys that we will be using to sign the transaction.
	let privateKeyStrings: string[] = [
		"d54b4de65b9bb6b076c248e4d3d14ef29875a241e1245f54e6601b0827123fd4",
		"08724d6795c40488df15c653c5ac4831c466482ec65846723add17ee2b67c610",
		"c98b96a1263b8b8506c71590357214e2e064ed36b7bf780c40a6a81d51b80916",
		"85657258fbf0a5751c3fc89e0cff88d7ac0801d6b5216a028c37085a179e2451",
	];
	let privateKeys: Uint8Array[] = privateKeyStrings.map((privateKeyString: string) =>
		Uint8Array.from(Buffer.from(privateKeyString, "hex"))
	);

	// The compiled transaction intent that we will be signing. We will first double hash it and then sign it.
	let compiledTransactionIntent: CryptoJS.lib.WordArray = CryptoJS.enc.Hex.parse(
		compileTransactionIntentResponse.compiled_intent
	);
	let doubleIntentHash: CryptoJS.lib.WordArray = CryptoJS.SHA256(
		CryptoJS.SHA256(compiledTransactionIntent)
	);
	let doubleIntentHashBytes: Uint8Array = Uint8Array.from(
		Buffer.from(doubleIntentHash.toString(), "hex")
	);

	// Signing the compiled transaction intent.
	let signatures: Signature[] = privateKeys.map((privateKey: Uint8Array): Signature => {
		let publicKey: Uint8Array = secp256k1.publicKeyCreate(privateKey, true);
		let signature: Uint8Array = secp256k1.ecdsaSign(doubleIntentHashBytes, privateKey).signature;

		return {
			public_key: Buffer.from(publicKey).toString("hex"),
			signature: Buffer.from(signature).toString("hex"),
		};
	});

	let compileSignedTransactionIntentRequest: CompileSignedTransactionIntentRequest = {
		transaction_intent: {
			header: transactionHeader,
			manifest,
		},
		signatures,
	};
	let compileSignedTransactionIntentResponse: CompileSignedTransactionIntentResponse =
		transactionService.compileSignedTransactionIntent(
			compileSignedTransactionIntentRequest
		) as CompileSignedTransactionIntentResponse;
	console.log("======= Example 5 =======");
	console.log(JSON.stringify(compileSignedTransactionIntentResponse, null, 4));
	console.log("=========================", "\n");

	// Example 6: Just like we have done with the previous examples, anything that is compiled down
	// can be decompiled again. In this case, the compiled signed transaction intent can be
	// decompiled.
	let decompileSignedTransactionIntentRequest: DecompileSignedTransactionIntentRequest = {
		compiled_signed_intent: compileSignedTransactionIntentResponse.compiled_signed_intent,
		manifest_output_format: ManifestKind.JSON,
	};
	let decompileSignedTransactionIntentResponse: DecompileSignedTransactionIntentResponse =
		transactionService.decompileSignedTransactionIntent(
			decompileSignedTransactionIntentRequest
		) as DecompileSignedTransactionIntentResponse;
	console.log("======= Example 6 =======");
	console.log(JSON.stringify(decompileSignedTransactionIntentResponse, null, 4));
	console.log("=========================", "\n");

	// Example 7: Compiling and decompiling of notarized transactions
	let compiledSignedTransactionIntent: CryptoJS.lib.WordArray = CryptoJS.enc.Hex.parse(
		compileSignedTransactionIntentResponse.compiled_signed_intent
	);
	let doubleSignedIntentHash: CryptoJS.lib.WordArray = CryptoJS.SHA256(
		CryptoJS.SHA256(compiledSignedTransactionIntent)
	);
	let doubleSignedIntentHashBytes: Uint8Array = Uint8Array.from(
		Buffer.from(doubleSignedIntentHash.toString(), "hex")
	);

	const notaryPrivateKeyString: string =
		"0d5666def4fb894f18a5075b261845c044b7e3dd2ba8514b2614dbbb6606c622";
	let notaryPrivateKey: Uint8Array = Uint8Array.from(Buffer.from(notaryPrivateKeyString, "hex"));
	let notarySignature = secp256k1.ecdsaSign(
		doubleSignedIntentHashBytes,
		notaryPrivateKey
	).signature;

	let compileNotarizedTransactionIntentRequest: CompileNotarizedTransactionIntentRequest = {
		signed_intent: {
			transaction_intent: {
				header: transactionHeader,
				manifest,
			},
			signatures,
		},
		notary_signature: Buffer.from(notarySignature).toString("hex"),
	};
	let compileNotarizedTransactionIntentResponse: CompileNotarizedTransactionIntentResponse =
		transactionService.compileNotarizedTransactionIntent(
			compileNotarizedTransactionIntentRequest
		) as CompileNotarizedTransactionIntentResponse;
	console.log("======= Example 7 =======");
	console.log(JSON.stringify(compileNotarizedTransactionIntentResponse, null, 4));
	console.log("=========================", "\n");

	let decompileNotarizedTransactionIntentRequest: DecompileNotarizedTransactionIntentRequest = {
		manifest_output_format: ManifestKind.JSON,
		compiled_notarized_intent: compileNotarizedTransactionIntentResponse.compiled_notarized_intent,
	};
	let decompileNotarizedTransactionIntentResponse: DecompileNotarizedTransactionIntentResponse =
		transactionService.decompileNotarizedTransactionIntent(
			decompileNotarizedTransactionIntentRequest
		) as DecompileNotarizedTransactionIntentResponse;
	console.log("======= Example 7 =======");
	console.log(JSON.stringify(decompileNotarizedTransactionIntentResponse, null, 4));
	console.log("=========================", "\n");

	// Example 8: There are cases where we might have some blob which we suspect to be a transaction
	// intent of some sort. However, there is no easy way to tell whether this is an unsigned,
	// signed, or notarized transaction compiled transaction intent blob. For this specific use
	// case, this library provides a function for the decompilation of a compiled transaction intent
	// which we are not sure what type it is.
	let decompileUnknownTransactionIntentRequest: DecompileUnknownTransactionIntentRequest = {
		manifest_output_format: ManifestKind.JSON,
		compiled_unknown_intent: compileNotarizedTransactionIntentResponse.compiled_notarized_intent,
	};
	let decompileUnknownTransactionIntentResponse: DecompileUnknownTransactionIntentResponse =
		transactionService.decompileUnknownTransactionIntent(
			decompileUnknownTransactionIntentRequest
		) as DecompileUnknownTransactionIntentResponse;
	console.log("======= Example 8 =======");
	console.log(JSON.stringify(decompileUnknownTransactionIntentResponse, null, 4));
	console.log("=========================", "\n");
};

main();
