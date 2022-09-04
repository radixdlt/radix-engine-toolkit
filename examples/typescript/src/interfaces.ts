import { Instruction } from "./instruction";

// ===================
// Request & Response
// ===================

export type Request = InformationRequest;

export type Response = InformationResponse;

export interface InformationRequest {}

export interface InformationResponse {
	package_version: string;
}

export interface ConvertManifestRequest {
	transaction_version: number
	network_id: number
	manifest_output_format: ManifestKind
	manifest: Manifest
}

export type ConvertManifestResponse = Manifest;

// =======
// Models
// =======

export enum ManifestKind {
	String = "String",
	JSON = "JSON",
}

export type Manifest = 
	| ManifestJSON
	| ManifestString ;

export interface ManifestString {
	readonly type: ManifestKind.String;
	value: string;
}

export interface ManifestJSON {
	readonly type: ManifestJSON;
	value: Instruction[];
}

// ============
// WASM Module
// ============

export interface TransactionServiceInterface {
	convert_manifest(requestStringPointer: number): number;

	compile_transaction_intent(requestStringPointer: number): number;
	decompile_transaction_intent(requestStringPointer: number): number;

	compile_signed_transaction_intent(requestStringPointer: number): number;
	decompile_signed_transaction_intent(requestStringPointer: number): number;

	information(requestStringPointer: number): number;

	alloc(capacity: number): number;
	free_mem(pointer: number): void;
}
