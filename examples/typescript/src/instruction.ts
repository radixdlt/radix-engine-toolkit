import {
	Value,
	PackageAddress,
	ResourceAddress,
	ComponentAddress,
	Decimal,
	Bucket,
	Proof,
	NonFungibleId,
	Bytes,
	String
} from "./value";

export type Instruction =
	| CallFunction
	| CallMethod
	| CallMethodWithAllResources
	| TakeFromWorktop
	| TakeFromWorktopByAmount
	| TakeFromWorktopByIds
	| ReturnToWorktop
	| AssertWorktopContains
	| AssertWorktopContainsByAmount
	| AssertWorktopContainsByIds
	| PopFromAuthZone
	| PushToAuthZone
	| ClearAuthZone
	| CreateProofFromAuthZone
	| CreateProofFromAuthZoneByAmount
	| CreateProofFromAuthZoneByIds
	| CreateProofFromBucket
	| CloneProof
	| DropProof
	| DropAllProofs
	| PublishPackage;

export interface CallFunction {
	readonly instruction: InstructionKind.CallFunction;
	package_address: PackageAddress;
	blueprint_name: String;
	function_name: String;
	arguments?: Value[];
}
export interface CallMethod {
	readonly instruction: InstructionKind.CallMethod;
	component_address: ComponentAddress;
	method_name: String;
	arguments?: Value[];
}
export interface CallMethodWithAllResources {
	readonly instruction: InstructionKind.CallMethodWithAllResources;
	component_address: ComponentAddress;
	method_name: String;
}

export interface TakeFromWorktop {
	readonly instruction: InstructionKind.TakeFromWorktop;
	resource_address: ResourceAddress;
	into_bucket: Bucket;
}
export interface TakeFromWorktopByAmount {
	readonly instruction: InstructionKind.TakeFromWorktopByAmount;
	resource_address: ResourceAddress;
	into_bucket: Bucket;
	amount: Decimal;
}
export interface TakeFromWorktopByIds {
	readonly instruction: InstructionKind.TakeFromWorktopByIds;
	resource_address: ResourceAddress;
	into_bucket: Bucket;
	ids: NonFungibleId[];
}
export interface ReturnToWorktop {
	readonly instruction: InstructionKind.ReturnToWorktop;
	bucket: Bucket;
}

export interface AssertWorktopContains {
	readonly instruction: InstructionKind.AssertWorktopContains;
	resource_address: ResourceAddress;
}
export interface AssertWorktopContainsByAmount {
	readonly instruction: InstructionKind.AssertWorktopContainsByAmount;
	resource_address: ResourceAddress;
	amount: Decimal;
}
export interface AssertWorktopContainsByIds {
	readonly instruction: InstructionKind.AssertWorktopContainsByIds;
	resource_address: ResourceAddress;
	ids: NonFungibleId[];
}
export interface PopFromAuthZone {
	readonly instruction: InstructionKind.PopFromAuthZone;
	into_proof: Proof;
}
export interface PushToAuthZone {
	readonly instruction: InstructionKind.PushToAuthZone;
	proof: Proof;
}
export interface ClearAuthZone {
	readonly instruction: InstructionKind.ClearAuthZone;
}

export interface CreateProofFromAuthZone {
	readonly instruction: InstructionKind.CreateProofFromAuthZone;
	resource_address: ResourceAddress;
	into_proof: Proof;
}
export interface CreateProofFromAuthZoneByAmount {
	readonly instruction: InstructionKind.CreateProofFromAuthZoneByAmount;
	resource_address: ResourceAddress;
	into_proof: Proof;
	amount: Decimal;
}
export interface CreateProofFromAuthZoneByIds {
	readonly instruction: InstructionKind.CreateProofFromAuthZoneByIds;
	resource_address: ResourceAddress;
	into_proof: Proof;
	ids: NonFungibleId[];
}

export interface CreateProofFromBucket {
	readonly instruction: InstructionKind.CreateProofFromBucket;
	bucket: Bucket;
	into_proof: Proof;
}

export interface CloneProof {
	readonly instruction: InstructionKind.CloneProof;
	proof: Proof;
	into_proof: Proof;
}
export interface DropProof {
	readonly instruction: InstructionKind.DropProof;
	proof: Proof;
}
export interface DropAllProofs {
	readonly instruction: InstructionKind.DropAllProofs;
}

export interface PublishPackage {
	readonly instruction: InstructionKind.PublishPackage;
	package: Bytes;
}

export enum InstructionKind {
	CallFunction = "CALL_FUNCTION",
	CallMethod = "CALL_METHOD",
	CallMethodWithAllResources = "CALL_METHOD_WITH_ALL_RESOURCES",

	TakeFromWorktop = "TAKE_FROM_WORKTOP",
	TakeFromWorktopByAmount = "TAKE_FROM_WORKTOP_BY_AMOUNT",
	TakeFromWorktopByIds = "TAKE_FROM_WORKTOP_BY_IDS",

	ReturnToWorktop = "RETURN_TO_WORKTOP",

	AssertWorktopContains = "ASSERT_WORKTOP_CONTAINS",
	AssertWorktopContainsByAmount = "ASSERT_WORKTOP_CONTAINS_BY_AMOUNT",
	AssertWorktopContainsByIds = "ASSERT_WORKTOP_CONTAINS_BY_IDS",

	PopFromAuthZone = "POP_FROM_AUTH_ZOME",
	PushToAuthZone = "PUSH_TO_AUTH_ZOME",
	ClearAuthZone = "CLEAR_AUTH_ZOME",

	CreateProofFromAuthZone = "CREATE_PROOF_FROM_AUTH_ZONE",
	CreateProofFromAuthZoneByAmount = "CREATE_PROOF_FROM_AUTH_ZONE_BY_AMOUNT",
	CreateProofFromAuthZoneByIds = "CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS",
	CreateProofFromBucket = "CREATE_PROOF_FROM_BUCKET",

	CloneProof = "CLONE_PROOF",
	DropProof = "DROP_PROOF",
	DropAllProofs = "DROP_ALL_PROOFS",

	PublishPackage = "PUBLISH_PACKAGE",
}
