export type Value =
	| Unit
	| Bool
	| U8
	| U16
	| U32
	| U64
	| U128
	| I8
	| I16
	| I32
	| I64
	| I128
	| String
	| Struct
	| Enum
	| Option
	| Result
	| Array
	| Tuple
	| List
	| Set
	| Map
	| Decimal
	| PreciseDecimal
	| ComponentAddress
	| PackageAddress
	| ResourceAddress
	| Hash
	| Bucket
	| Proof
	| NonFungibleId
	| NonFungibleAddress
	| Bytes;

export interface Unit {
	readonly type: ValueKind.Unit;
}
export interface Bool {
	readonly type: ValueKind.Bool;
	value: boolean;
}

export interface U8 {
	readonly type: ValueKind.U8;
	value: string;
}
export interface U16 {
	readonly type: ValueKind.U16;
	value: string;
}
export interface U32 {
	readonly type: ValueKind.U32;
	value: string;
}
export interface U64 {
	readonly type: ValueKind.U64;
	value: string;
}
export interface U128 {
	readonly type: ValueKind.U128;
	value: string;
}

export interface I8 {
	readonly type: ValueKind.I8;
	value: string;
}
export interface I16 {
	readonly type: ValueKind.I16;
	value: string;
}
export interface I32 {
	readonly type: ValueKind.I32;
	value: string;
}
export interface I64 {
	readonly type: ValueKind.I64;
	value: string;
}
export interface I128 {
	readonly type: ValueKind.I128;
	value: string;
}

export interface String {
	readonly type: ValueKind.String;
	value: string;
}

export interface Struct {
	readonly type: ValueKind.Struct;
	fields: Value[];
}
export interface Enum {
	readonly type: ValueKind.Enum;
	variant_name: string;
	fields?: Value[];
}
export interface Option {
	readonly type: ValueKind.Option;
	value?: Value;
}

// TODO: Investigate the proper representation.
export interface Result {
	readonly type: ValueKind.Result;
	value?: Value;
}

export interface Array {
	readonly type: ValueKind.Array;
	element_type: ValueKind;
	elements: Value[];
}
export interface Tuple {
	readonly type: ValueKind.Tuple;
	elements: Value[];
}

export interface List {
	readonly type: ValueKind.List;
	element_type: ValueKind;
	elements: Value[];
}
export interface Set {
	readonly type: ValueKind.Set;
	element_type: ValueKind;
	elements: Value[];
}
export interface Map {
	readonly type: ValueKind.Map;
	key_type: ValueKind;
	value_type: ValueKind;
	elements: Value[];
}

export interface Decimal {
	readonly type: ValueKind.Decimal;
	value: string;
}
export interface PreciseDecimal {
	readonly type: ValueKind.PreciseDecimal;
	value: string;
}

export interface ComponentAddress {
	readonly type: ValueKind.ComponentAddress;
	value: string;
}
export interface ResourceAddress {
	readonly type: ValueKind.ResourceAddress;
	value: string;
}
export interface PackageAddress {
	readonly type: ValueKind.PackageAddress;
	value: string;
}

export interface Hash {
	readonly type: ValueKind.Hash;
	value: string;
}

export interface Bucket {
	readonly type: ValueKind.Bucket;
	value: number; // 32-bit unsigned number
}
export interface Proof {
	readonly type: ValueKind.Proof;
	value: number; // 32-bit unsigned number
}

export interface NonFungibleId {
	readonly type: ValueKind.NonFungibleId;
	value: string;
}
export interface NonFungibleAddress {
	readonly type: ValueKind.NonFungibleAddress;
	value: string;
}

export interface Bytes {
	readonly type: ValueKind.Bytes;
	value: string;
}

export enum ValueKind {
	Unit = "Unit",
	Bool = "Bool",

	I8 = "I8",
	I16 = "I16",
	I32 = "I32",
	I64 = "I64",
	I128 = "I128",

	U8 = "U8",
	U16 = "U16",
	U32 = "U32",
	U64 = "U64",
	U128 = "U128",

	String = "String",

	Struct = "Struct",
	Enum = "Enum",

	Option = "Option",
	Array = "Array",
	Tuple = "Tuple",
	Result = "Result",

	List = "List",
	Set = "Set",
	Map = "Map",

	Decimal = "Decimal",
	PreciseDecimal = "PreciseDecimal",

	PackageAddress = "PackageAddress",
	ComponentAddress = "ComponentAddress",
	ResourceAddress = "ResourceAddress",

	Hash = "Hash",

	Bucket = "Bucket",
	Proof = "Proof",

	NonFungibleId = "NonFungibleId",
	NonFungibleAddress = "NonFungibleAddress",

	Bytes = "Bytes",
}
