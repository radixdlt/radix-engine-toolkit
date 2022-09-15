import { Instruction } from "./instruction";
import { Value, ComponentAddress, PackageAddress, ResourceAddress } from "./value";

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
  | DecompileNotarizedTransactionIntentRequest
  | DecompileUnknownTransactionIntentRequest
  | EncodeAddressRequest
  | DecodeAddressRequest
  | SBOREncodeRequest
  | SBORDecodeRequest;

export type Response =
  | InformationResponse
  | ConvertManifestResponse
  | CompileTransactionIntentResponse
  | DecompileTransactionIntentResponse
  | CompileSignedTransactionIntentResponse
  | DecompileSignedTransactionIntentResponse
  | CompileNotarizedTransactionIntentResponse
  | DecompileNotarizedTransactionIntentResponse
  | DecompileUnknownTransactionIntentResponse
  | EncodeAddressResponse
  | DecodeAddressResponse
  | SBOREncodeResponse
  | SBORDecodeResponse;

// eslint-disable-next-line @typescript-eslint/no-empty-interface
export interface InformationRequest {}

export interface InformationResponse {
  package_version: string;
}

export interface ConvertManifestRequest {
  transaction_version: number;
  network_id: number;
  manifest_instructions_output_format: ManifestKind;
  manifest: TransactionManifest;
}

export type ConvertManifestResponse = TransactionManifest;

export interface CompileTransactionIntentRequest {
  header: TransactionHeader;
  manifest: TransactionManifest;
}

export interface CompileTransactionIntentResponse {
  compiled_intent: string;
}

export interface DecompileTransactionIntentRequest {
  manifest_instructions_output_format: ManifestKind;
  compiled_intent: string;
}

export interface DecompileTransactionIntentResponse {
  header: TransactionHeader;
  manifest: TransactionManifest;
}

export interface CompileSignedTransactionIntentRequest {
  transaction_intent: TransactionIntent;
  signatures: SignatureWithPublicKey[];
}

export interface CompileSignedTransactionIntentResponse {
  compiled_signed_intent: string;
}

export interface DecompileSignedTransactionIntentRequest {
  manifest_instructions_output_format: ManifestKind;
  compiled_signed_intent: string;
}

export interface DecompileSignedTransactionIntentResponse {
  transaction_intent: TransactionIntent;
  signatures: SignatureWithPublicKey[];
}

export interface CompileNotarizedTransactionIntentRequest {
  signed_intent: SignedTransactionIntent;
  notary_signature: EcdsaSignature;
}

export interface CompileNotarizedTransactionIntentResponse {
  compiled_notarized_intent: string;
}

export interface DecompileNotarizedTransactionIntentRequest {
  manifest_instructions_output_format: ManifestKind;
  compiled_notarized_intent: string;
}

export interface DecompileNotarizedTransactionIntentResponse {
  signed_intent: SignedTransactionIntent;
  notary_signature: EcdsaSignature;
}

export interface DecompileUnknownTransactionIntentRequest {
  manifest_instructions_output_format: ManifestKind;
  compiled_unknown_intent: string;
}

export type DecompileUnknownTransactionIntentResponse =
  | DecompileTransactionIntentResponse
  | DecompileSignedTransactionIntentResponse
  | DecompileNotarizedTransactionIntentResponse;

export interface DecodeAddressRequest {
  address: string;
}

export interface DecodeAddressResponse {
  network_id: number;
  entity_type: AddressKind;
  data: string;
  hrp: string;
  address: Address;
}

export interface EncodeAddressRequest {
  address: string;
  network_id: number;
}

export type EncodeAddressResponse = Address;

export type SBOREncodeRequest = Value;

export interface SBOREncodeResponse {
  encoded_value: string;
}

export interface SBORDecodeRequest {
  encoded_value: string;
  network_id: number;
}

export type SBORDecodeResponse = Value;

// =======
// Models
// =======

export enum ManifestKind {
  String = "String",
  JSON = "JSON",
}

export type ManifestInstructions = ManifestJSON | ManifestString;

export interface ManifestString {
  readonly type: ManifestKind.String;
  value: string;
}

export interface ManifestJSON {
  readonly type: ManifestKind.JSON;
  value: Instruction[];
}

export interface TransactionHeader {
  version: number;
  network_id: number;
  start_epoch_inclusive: number;
  end_epoch_exclusive: number;
  nonce: number;
  notary_public_key: PublicKey;
  notary_as_signatory: boolean;
  cost_unit_limit: number;
  tip_percentage: number;
}

export interface TransactionManifest {
  instructions: ManifestInstructions;
  blobs?: string[];
}

export enum Curve {
  Ecdsa = "Ecdsa",
  Ed25519 = "Ed25519",
}

export type PublicKey = Ed25519PublicKey | EcdsaPublicKey;
export type Signature = Ed25519Signature | EcdsaSignature;

export interface Ed25519PublicKey {
  readonly type: Curve.Ed25519;
  public_key: string;
}
export interface Ed25519Signature {
  readonly type: Curve.Ed25519;
  signature: string;
}

export interface EcdsaPublicKey {
  readonly type: Curve.Ecdsa;
  public_key: string;
}
export interface EcdsaSignature {
  readonly type: Curve.Ecdsa;
  signature: string;
}

export interface TransactionIntent {
  header: TransactionHeader;
  manifest: TransactionManifest;
}

export interface SignedTransactionIntent {
  transaction_intent: TransactionIntent;
  signatures: SignatureWithPublicKey[];
}

export type SignatureWithPublicKey = EcdsaSignatureWithPublicKey | Ed25519SignatureWithPublicKey;

export interface EcdsaSignatureWithPublicKey {
  readonly type: Curve.Ecdsa;
  signature: string;
}

export interface Ed25519SignatureWithPublicKey {
  readonly type: Curve.Ed25519;
  public_key: string;
  signature: string;
}

export type Address = ComponentAddress | ResourceAddress | PackageAddress;

export enum AddressKind {
  Resource = "Resource",
  Package = "Package",
  AccountComponent = "AccountComponent",
  SystemComponent = "SystemComponent",
  NormalComponent = "NormalComponent",
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

  decompile_unknown_transaction_intent(requestStringPointer: number): number;

  information(requestStringPointer: number): number;

  encode_address(requestStringPointer: number): number;
  decode_address(requestStringPointer: number): number;

  sbor_encode(requestStringPointer: number): number;
  sbor_decode(requestStringPointer: number): number;

  __transaction_lib_alloc(capacity: number): number;
  __transaction_lib_free(pointer: number): void;
}
