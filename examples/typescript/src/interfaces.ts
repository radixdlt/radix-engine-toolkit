// ===================
// Request & Response
// ===================

export type Request = InformationRequest;

export type Response = InformationResponse;

export interface InformationRequest {}

export interface InformationResponse {
	package_version: string;
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
