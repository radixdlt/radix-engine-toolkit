"use strict";

import {
	CompileTransactionIntentRequest,
	CompileTransactionIntentResponse,
	ConvertManifestRequest,
	ConvertManifestResponse,
	DecompileTransactionIntentRequest,
	DecompileTransactionIntentResponse,
	InformationRequest,
	InformationResponse,
	Request,
	Response,
	TransactionServiceInterface,
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
	AddressInformationRequest,
	AddressInformationResponse
} from "./interfaces";
import { Error } from "./error";
import fs from "fs";

export default class TransactionAPI {
	private wasmModule: WebAssembly.WebAssemblyInstantiatedSource;
	private internal_service: TransactionServiceInterface;

	// =============
	// Constructors
	// =============

	constructor(wasmModule: WebAssembly.WebAssemblyInstantiatedSource) {
		this.wasmModule = wasmModule;
		this.internal_service = wasmModule.instance.exports as unknown as TransactionServiceInterface;
	}

	static async fromPath(path: string): Promise<TransactionAPI> {
		let contents: Uint8Array = await fs.readFileSync(path);
		return TransactionAPI.fromWasmModuleBuffer(contents);
	}

	static async fromWasmModuleBuffer(buffer: Uint8Array): Promise<TransactionAPI> {
		let wasmImports: WebAssembly.Imports = {
			env: {
				memory: new WebAssembly.Memory({
					initial: 1,
				}),
			},
		};
		let wasmModule: WebAssembly.WebAssemblyInstantiatedSource = await WebAssembly.instantiate(
			buffer,
			wasmImports
		);
		return new TransactionAPI(wasmModule);
	}

	// =================
	// Exported Methods
	// =================

	information(): InformationResponse | Error {
		return this.callWasmFunction({} as InformationRequest, this.internal_service.information) as
			| InformationResponse
			| Error;
	}

	convertManifest(request: ConvertManifestRequest): ConvertManifestResponse | Error {
		return this.callWasmFunction(request, this.internal_service.convert_manifest) as
			| ConvertManifestResponse
			| Error;
	}

	compileTransactionIntent(
		request: CompileTransactionIntentRequest
	): CompileTransactionIntentResponse | Error {
		return this.callWasmFunction(request, this.internal_service.compile_transaction_intent) as
			| CompileTransactionIntentResponse
			| Error;
	}

	decompileTransactionIntent(
		request: DecompileTransactionIntentRequest
	): DecompileTransactionIntentResponse | Error {
		return this.callWasmFunction(request, this.internal_service.decompile_transaction_intent) as
			| DecompileTransactionIntentResponse
			| Error;
	}

	compileSignedTransactionIntent(
		request: CompileSignedTransactionIntentRequest
	): CompileSignedTransactionIntentResponse | Error {
		return this.callWasmFunction(
			request,
			this.internal_service.compile_signed_transaction_intent
		) as CompileSignedTransactionIntentResponse | Error;
	}

	decompileSignedTransactionIntent(
		request: DecompileSignedTransactionIntentRequest
	): DecompileSignedTransactionIntentResponse | Error {
		return this.callWasmFunction(
			request,
			this.internal_service.decompile_signed_transaction_intent
		) as DecompileSignedTransactionIntentResponse | Error;
	}

	compileNotarizedTransactionIntent(
		request: CompileNotarizedTransactionIntentRequest
	): CompileNotarizedTransactionIntentResponse | Error {
		return this.callWasmFunction(
			request,
			this.internal_service.compile_notarized_transaction_intent
		) as CompileNotarizedTransactionIntentResponse | Error;
	}

	decompileNotarizedTransactionIntent(
		request: DecompileNotarizedTransactionIntentRequest
	): DecompileNotarizedTransactionIntentResponse | Error {
		return this.callWasmFunction(
			request,
			this.internal_service.decompile_notarized_transaction_intent
		) as DecompileNotarizedTransactionIntentResponse | Error;
	}

	decompileUnknownTransactionIntent(
		request: DecompileUnknownTransactionIntentRequest
	): DecompileUnknownTransactionIntentResponse | Error {
		return this.callWasmFunction(
			request,
			this.internal_service.decompile_unknown_transaction_intent
		) as DecompileUnknownTransactionIntentResponse | Error;
	}
	
	addressInformation(
		request: AddressInformationRequest
	): AddressInformationResponse | Error {
		return this.callWasmFunction(
			request,
			this.internal_service.address_information
		) as AddressInformationResponse | Error;
	}

	private callWasmFunction(
		request: Request,
		wasmFunction: (requestStringPointer: number) => number
	): Response | Error {
		// Serialize the request as JSON and write it to WASM's memory
		let requestStringPointer: number = this.writeString(JSON.stringify(request));

		// Call the method on the WASM module
		let responsePointer: number = wasmFunction(requestStringPointer);

		// Read and parse the returned response
		let returnedString: string = this.readString(responsePointer);
		let parsedResponse: Response | Error = JSON.parse(returnedString);

		// Free up the memory needed in this operation
		this.internal_service.__transaction_lib_free(requestStringPointer);
		this.internal_service.__transaction_lib_free(responsePointer);

		return parsedResponse;
	}

	// ===============
	// Helper Methods
	// ===============

	private readString(pointer: number): string {
		// @ts-ignore
		let memoryBuffer: Uint8Array = this.wasmModule.instance.exports.memory.buffer;

		const view: Uint8Array = new Uint8Array(memoryBuffer, pointer);
		const length: number = view.findIndex((byte) => byte === 0);
		const decoder: TextDecoder = new TextDecoder();

		return decoder.decode(new Uint8Array(memoryBuffer, pointer, length));
	}

	private writeString(string: string): number {
		const pointer: number = this.allocateMemory(string);

		// @ts-ignore
		let memoryBuffer: Uint8Array = this.wasmModule.instance.exports.memory.buffer;

		const view: Uint8Array = new Uint8Array(memoryBuffer, pointer);
		const encoder: TextEncoder = new TextEncoder();
		view.set(new Uint8Array([...encoder.encode(string), 0])); // Adding 0 at the end to be a c-style string

		return pointer;
	}

	private allocateMemory(string: string): number {
		// Take the string and convert it into a byte array to determine its length
		let byteArray: Uint8Array = new TextEncoder().encode(string);
		let pointer: number = this.internal_service.__transaction_lib_alloc(byteArray.length + 1);
		return pointer;
	}
}
