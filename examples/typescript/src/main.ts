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
	TransactionHeader,
} from "./interfaces";

const main = async (): Promise<void> => {
	// Creating a new transaction service object from the transaction service WASM file path
	const path: string = "../../target/wasm32-unknown-unknown/release/transaction_service.wasm";
	const transactionService: TransactionService = await TransactionService.fromPath(path);

	// Example 1: Printing the information of the transaction service. This is essentially the
	// "Hello World" of this project. If the information of the package is printed correctly, then
	// this means that the calls to the WASM modules are happening without any issues.
	console.log(transactionService.information());

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
	console.log(JSON.stringify(manifestConversionResponse, null, 4));

	// Example 3: When signing a transaction, the compiled intent of a transaction is what gets
	// signed. Obtaining this compiled intent requires SBOR encoding the intent and therefore
	// requires an SBOR implementation. However, this library provides the ability to compile
	// transactions without needing to implement the SBOR codec at the client.
	let transactionHeader: TransactionHeader = {
		version: 0x01,
		network_id: 0xf2,
		start_epoch_inclusive: 0x00,
		end_epoch_exclusive: 0x00,
		nonce: 0x00,
		notary_public_key: "03e53e0f0c9c934efc7b81eb1a9c339615aeadbefba4dc94db5a196c76fcbdd8b1",
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
	console.log(JSON.stringify(compileTransactionIntentResponse, null, 4));

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
	console.log(JSON.stringify(decompileTransactionIntentResponse, null, 4));
};

main();
