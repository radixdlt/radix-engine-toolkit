import { Instruction } from "./instruction";

// ===================
// Request & Response
// ===================

export type Request =
	| InformationRequest
	| ConvertManifestRequest
	| CompileTransactionIntentRequest
	| DecompileTransactionIntentRequest
	| CompileSignedTransactionIntentRequest
	| DecompileSignedTransactionIntentRequest
	| CompileNotarizedTransactionIntentRequest
	| DecompileNotarizedTransactionIntentRequest;

export type Response =
	| InformationResponse
	| ConvertManifestResponse
	| CompileTransactionIntentResponse
	| DecompileTransactionIntentResponse
	| CompileSignedTransactionIntentResponse
	| DecompileSignedTransactionIntentResponse
	| CompileNotarizedTransactionIntentResponse
	| DecompileNotarizedTransactionIntentResponse;

export interface InformationRequest {}

export interface InformationResponse {
	package_version: string;
}

export interface ConvertManifestRequest {
	transaction_version: number;
	network_id: number;
	manifest_output_format: ManifestKind;
	manifest: Manifest;
}

export type ConvertManifestResponse = Manifest;

export interface CompileTransactionIntentRequest {
	header: TransactionHeader;
	manifest: Manifest;
}

export interface CompileTransactionIntentResponse {
	compiled_intent: string;
}

export interface DecompileTransactionIntentRequest {
	manifest_output_format: ManifestKind;
	compiled_intent: string;
}

export interface DecompileTransactionIntentResponse {
	header: TransactionHeader;
	manifest: Manifest;
}

export interface CompileSignedTransactionIntentRequest {
	transaction_intent: TransactionIntent;
	signatures: Signature[];
}

export interface CompileSignedTransactionIntentResponse {
	compiled_signed_intent: string;
}

export interface DecompileSignedTransactionIntentRequest {
	manifest_output_format: ManifestKind;
	compiled_signed_intent: string;
}

export interface DecompileSignedTransactionIntentResponse {
	transaction_intent: TransactionIntent;
	signatures: Signature[];
}

export interface CompileNotarizedTransactionIntentRequest {
	signed_intent: SignedTransactionIntent;
	notary_signature: EcdsaSignature;
}

export interface CompileNotarizedTransactionIntentResponse {
	compiled_notarized_intent: string;
}

export interface DecompileNotarizedTransactionIntentRequest {
	manifest_output_format: ManifestKind;
	compiled_notarized_intent: string;
}

export interface DecompileNotarizedTransactionIntentResponse {
	signed_intent: SignedTransactionIntent;
	notary_signature: EcdsaSignature;
}

// =======
// Models
// =======

export enum ManifestKind {
	String = "String",
	JSON = "JSON",
}

export type Manifest = ManifestJSON | ManifestString;

export interface ManifestString {
	readonly type: ManifestKind.String;
	value: string;
}

export interface ManifestJSON {
	readonly type: ManifestJSON;
	value: Instruction[];
}

export interface TransactionHeader {
	version: number;
	network_id: number;
	start_epoch_inclusive: number;
	end_epoch_exclusive: number;
	nonce: number;
	notary_public_key: EcdsaPublicKey;
	notary_as_signatory: boolean;
	cost_unit_limit: number;
	tip_percentage: number;
}

export type EcdsaPublicKey = string;
export type EcdsaSignature = string;

export interface TransactionIntent {
	header: TransactionHeader;
	manifest: Manifest;
}

export interface SignedTransactionIntent {
	transaction_intent: TransactionIntent;
	signatures: Signature[];
}

export interface Signature {
	public_key: EcdsaPublicKey;
	signature: EcdsaSignature;
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

	compile_notarized_transaction_intent(requestStringPointer: number): number;
	decompile_notarized_transaction_intent(requestStringPointer: number): number;

	information(requestStringPointer: number): number;

	__transaction_lib_alloc(capacity: number): number;
	__transaction_lib_free(pointer: number): void;
}
