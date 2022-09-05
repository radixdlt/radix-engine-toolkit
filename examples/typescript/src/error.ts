import { ValueKind } from "./value"

export type Error = 
    | AddressError
    | DecodeError
    | DeserializationError
    | InvalidRequestString
    | UnexpectedContents
    | InvalidType
    | ParseError
    | TransactionCompileError
    | TransactionDecompileError
    | UnsupportedTransactionVersion
    | GeneratorError
    | RequestResponseConversionError
    | UnrecognizedCompiledIntentFormat;

export interface AddressError {
    readonly error: ErrorKind.AddressError,
    value: string
}

export interface DecodeError {
    readonly error: ErrorKind.DecodeError,
    value: string
}

export interface DeserializationError {
    readonly error: ErrorKind.DeserializationError,
    value: string
}

export interface InvalidRequestString {
    readonly error: ErrorKind.InvalidRequestString,
    value: string
}

export interface UnexpectedContents {
    readonly error: ErrorKind.UnexpectedContents,
    value: {
        kind: ValueKind,
        expected: ValueKind[],
        found: ValueKind,
    }
}

export interface InvalidType {
    readonly error: ErrorKind.InvalidType,
    value: {
        expected_type: ValueKind,
        actual_type: ValueKind,
    }
}

export interface ParseError {
    readonly error: ErrorKind.ParseError,
    value: {
        kind: ValueKind,
        error: string,
    }
}

export interface TransactionCompileError {
    readonly error: ErrorKind.TransactionCompileError,
    value: string
}

export interface TransactionDecompileError {
    readonly error: ErrorKind.TransactionDecompileError,
    value: string
}

export interface UnsupportedTransactionVersion {
    readonly error: ErrorKind.UnsupportedTransactionVersion,
    value: number
}

export interface GeneratorError {
    readonly error: ErrorKind.GeneratorError,
    value: string
}

export interface RequestResponseConversionError {
    readonly error: ErrorKind.RequestResponseConversionError,
    value: string
}

export interface UnrecognizedCompiledIntentFormat {
    readonly error: ErrorKind.UnrecognizedCompiledIntentFormat,
}

export enum ErrorKind {
    AddressError = "AddressError",
    DecodeError = "DecodeError",
    
    DeserializationError = "DeserializationError",
    InvalidRequestString = "InvalidRequestString",
    
    UnexpectedContents = "UnexpectedContents",
    InvalidType = "InvalidType",
    ParseError = "ParseError",
    
    TransactionCompileError = "TransactionCompileError",
    TransactionDecompileError = "TransactionDecompileError",
    UnsupportedTransactionVersion = "UnsupportedTransactionVersion",
    GeneratorError = "GeneratorError",
    
    RequestResponseConversionError = "RequestResponseConversionError",
    UnrecognizedCompiledIntentFormat = "UnrecognizedCompiledIntentFormat",
}
