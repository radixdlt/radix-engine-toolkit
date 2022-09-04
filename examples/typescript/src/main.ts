import TransactionService from "./transaction-service";
import fs from "fs";
import {
	ConvertManifestRequest,
	ConvertManifestResponse,
	Manifest,
	ManifestKind,
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
};

main();
