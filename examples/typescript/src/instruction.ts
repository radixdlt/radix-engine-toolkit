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
	readonly type: InstructionKind.CallFunction;
	package_address: PackageAddress;
	blueprint_name: String;
	function_name: String;
	arguments?: Value[];
}
export interface CallMethod {
	readonly type: InstructionKind.CallMethod;
	component_address: ComponentAddress;
	method_name: String;
	arguments?: Value[];
}
export interface CallMethodWithAllResources {
	readonly type: InstructionKind.CallMethodWithAllResources;
	component_address: ComponentAddress;
	method_name: String;
}

export interface TakeFromWorktop {
	readonly type: InstructionKind.TakeFromWorktop;
	resource_address: ResourceAddress;
	into_bucket: Bucket;
}
export interface TakeFromWorktopByAmount {
	readonly type: InstructionKind.TakeFromWorktopByAmount;
	resource_address: ResourceAddress;
	into_bucket: Bucket;
	amount: Decimal;
}
export interface TakeFromWorktopByIds {
	readonly type: InstructionKind.TakeFromWorktopByIds;
	resource_address: ResourceAddress;
	into_bucket: Bucket;
	ids: NonFungibleId[];
}
export interface ReturnToWorktop {
	readonly type: InstructionKind.ReturnToWorktop;
	bucket: Bucket;
}

export interface AssertWorktopContains {
	readonly type: InstructionKind.AssertWorktopContains;
	resource_address: ResourceAddress;
}
export interface AssertWorktopContainsByAmount {
	readonly type: InstructionKind.AssertWorktopContainsByAmount;
	resource_address: ResourceAddress;
	amount: Decimal;
}
export interface AssertWorktopContainsByIds {
	readonly type: InstructionKind.AssertWorktopContainsByIds;
	resource_address: ResourceAddress;
	ids: NonFungibleId[];
}
export interface PopFromAuthZone {
	readonly type: InstructionKind.PopFromAuthZone;
	into_proof: Proof;
}
export interface PushToAuthZone {
	readonly type: InstructionKind.PushToAuthZone;
	proof: Proof;
}
export interface ClearAuthZone {
	readonly type: InstructionKind.ClearAuthZone;
}

export interface CreateProofFromAuthZone {
	readonly type: InstructionKind.CreateProofFromAuthZone;
	resource_address: ResourceAddress;
	into_proof: Proof;
}
export interface CreateProofFromAuthZoneByAmount {
	readonly type: InstructionKind.CreateProofFromAuthZoneByAmount;
	resource_address: ResourceAddress;
	into_proof: Proof;
	amount: Decimal;
}
export interface CreateProofFromAuthZoneByIds {
	readonly type: InstructionKind.CreateProofFromAuthZoneByIds;
	resource_address: ResourceAddress;
	into_proof: Proof;
	ids: NonFungibleId[];
}

export interface CreateProofFromBucket {
	readonly type: InstructionKind.CreateProofFromBucket;
	bucket: Bucket;
	into_proof: Proof;
}

export interface CloneProof {
	readonly type: InstructionKind.CloneProof;
	proof: Proof;
	into_proof: Proof;
}
export interface DropProof {
	readonly type: InstructionKind.DropProof;
	proof: Proof;
}
export interface DropAllProofs {
	readonly type: InstructionKind.DropAllProofs;
}

export interface PublishPackage {
	readonly type: InstructionKind.PublishPackage;
	package: Bytes;
}

export enum InstructionKind {
	CallFunction = "CallFunction",
	CallMethod = "CallMethod",
	CallMethodWithAllResources = "CallMethodWithAllResources",

	TakeFromWorktop = "TakeFromWorktop",
	TakeFromWorktopByAmount = "TakeFromWorktopByAmount",
	TakeFromWorktopByIds = "TakeFromWorktopByIds",

	ReturnToWorktop = "ReturnToWorktop",

	AssertWorktopContains = "AssertWorktopContains",
	AssertWorktopContainsByAmount = "AssertWorktopContainsByAmount",
	AssertWorktopContainsByIds = "AssertWorktopContainsByIds",

	PopFromAuthZone = "PopFromAuthZone",
	PushToAuthZone = "PushToAuthZone",
	ClearAuthZone = "ClearAuthZone",

	CreateProofFromAuthZone = "CreateProofFromAuthZone",
	CreateProofFromAuthZoneByAmount = "CreateProofFromAuthZoneByAmount",
	CreateProofFromAuthZoneByIds = "CreateProofFromAuthZoneByIds",
	CreateProofFromBucket = "CreateProofFromBucket",

	CloneProof = "CloneProof",
	DropProof = "DropProof",
	DropAllProofs = "DropAllProofs",

	PublishPackage = "PublishPackage",
}
