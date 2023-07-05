// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! This is an internal prelude module meant to only be public within this crate which re-exports
//! types and functions from the core toolkit and Scrypto crates with a prefix. This is done to
//! distinguish between the core toolkit types, scrypto types, and types defined in this crate.
//! We choose the types in this crate to have short un-prefixed names since those are the names that
//! the types will be generated with when using UniFFI

#[rustfmt::skip]
mod core {
    /* Modules */
    pub use radix_engine_toolkit_core::functions::information::{
        information as core_information, 
        BuildInformation as CoreBuildInformation,
        DependencyInformation as CoreDependencyInformation,
    };
    pub use radix_engine_toolkit_core::functions::derive::{
        virtual_account_address_from_public_key as core_virtual_account_address_from_public_key,
        virtual_identity_address_from_public_key as core_virtual_identity_address_from_public_key,
        virtual_signature_non_fungible_global_id_from_public_key as core_virtual_signature_non_fungible_global_id_from_public_key,
        virtual_account_address_from_olympia_account_address as core_virtual_account_address_from_olympia_account_address,
        resource_address_from_olympia_resource_address as core_resource_address_from_olympia_resource_address,
        public_key_from_olympia_account_address as core_public_key_from_olympia_account_address,
        olympia_account_address_from_public_key as core_olympia_account_address_from_public_key,
        node_address_from_public_key as core_node_address_from_public_key,
        DerivationError as CoreDerivationError,
        OlympiaNetwork as CoreOlympiaNetwork,
    };
    pub use radix_engine_toolkit_core::functions::instructions::{
        hash as core_instructions_hash,
        compile as core_instructions_compile,
        decompile as core_instructions_decompile,
        statically_validate as core_instructions_statically_validate,
        extract_addresses as core_instructions_extract_addresses,
        identities_requiring_auth as core_instructions_identities_requiring_auth,
        accounts_requiring_auth as core_instructions_accounts_requiring_auth,
        accounts_withdrawn_from as core_instructions_accounts_withdrawn_from,
        accounts_deposited_into as core_instructions_accounts_deposited_into,
        InstructionValidationError as CoreInstructionValidationError,
    };
    pub use radix_engine_toolkit_core::functions::manifest::{
        hash as core_manifest_hash,
        compile as core_manifest_compile,
        decompile as core_manifest_decompile,
        statically_validate as core_manifest_statically_validate,
    };
    pub use radix_engine_toolkit_core::functions::intent::{
        hash as core_intent_hash,
        compile as core_intent_compile,
        decompile as core_intent_decompile,
        statically_validate as core_intent_statically_validate,
    };
    pub use radix_engine_toolkit_core::functions::signed_intent::{
        hash as core_signed_intent_hash,
        compile as core_signed_intent_compile,
        decompile as core_signed_intent_decompile,
        statically_validate as core_signed_intent_statically_validate,
    };
    pub use radix_engine_toolkit_core::functions::notarized_transaction::{
        hash as core_notarized_transaction_hash,
        compile as core_notarized_transaction_compile,
        decompile as core_notarized_transaction_decompile,
        statically_validate as core_notarized_transaction_statically_validate,
    };
    pub use radix_engine_toolkit_core::functions::execution::{
        analyze as core_execution_analyze,
        ExecutionAnalysis as CoreExecutionExecutionAnalysis,
        FeeSummary as CoreExecutionFeeSummary,
        FeeLocks as CoreExecutionFeeLocks,
        SimpleTransferTransactionType as CoreExecutionSimpleTransferTransactionType,
        TransferTransactionType as CoreExecutionTransferTransactionType,
        GeneralTransactionType as CoreExecutionGeneralTransactionType,
        TransactionType as CoreExecutionTransactionType,
        ExecutionModuleError as CoreExecutionExecutionModuleError,
    };
    pub use radix_engine_toolkit_core::functions::manifest_sbor::{
        ManifestSborError as CoreManifestSborError,
        ManifestSborStringRepresentation as CoreManifestSborStringRepresentation,
        decode_to_string_representation as core_manifest_decode_to_string_representation,
    };
    pub use radix_engine_toolkit_core::functions::scrypto_sbor::{
        ScryptoSborError as CoreScryptoSborError,
        decode_to_string_representation as core_scrypto_decode_to_string_representation,
    };

    /* Visitors */
    pub use radix_engine_toolkit_core::instruction_visitor::visitors::transaction_type::transfer_visitor::{
        Resources as CoreResources,
    };
    pub use radix_engine_toolkit_core::instruction_visitor::visitors::transaction_type::general_transaction_visitor::{
        Source as CoreSource,
    };

    /* Utils */
    pub use radix_engine_toolkit_core::utils::{
        manifest_from_intent as core_manifest_from_intent,
        network_definition_from_network_id as core_network_definition_from_network_id,
        network_id_from_hrp as core_network_id_from_hrp,
        network_id_from_address_string as core_network_id_from_address_string,
        to_manifest_type as core_to_manifest_type,
        validate_manifest_value_against_schema as core_validate_manifest_value_against_schema,
        is_account as core_is_account,
        is_identity as core_is_identity,
        metadata_of_newly_created_entities as core_metadata_of_newly_created_entities,
        data_of_newly_minted_non_fungibles as core_data_of_newly_minted_non_fungibles,
    };
}

#[rustfmt::skip]
mod native {
    pub use transaction::prelude::{
        /* Cryptography */
        Ed25519PrivateKey as NativeEd25519PrivateKey,
        Secp256k1PrivateKey as NativeSecp256k1PrivateKey,
        
        PublicKey as NativePublicKey, 
        Ed25519PublicKey as NativeEd25519PublicKey,
        Secp256k1PublicKey as NativeSecp256k1PublicKey,
        
        PublicKeyHash as NativePublicKeyHash,
        HasPublicKeyHash as NativeHasPublicKeyHash,
        Ed25519PublicKeyHash as NativeEd25519PublicKeyHash,
        Secp256k1PublicKeyHash as NativeSecp256k1PublicKeyHash,

        SignatureV1 as NativeSignature,
        Ed25519Signature as NativeEd25519Signature, 
        Secp256k1Signature as NativeSecp256k1Signature, 

        SignatureWithPublicKeyV1 as NativeSignatureWithPublicKey,
    };
    pub use transaction::manifest::{
        compile as native_compile,
        decompile as native_decompile,
        
        IsBlobProvider as NativeIsBlobProvider,
        BlobProvider as NativeBlobProvider,
        MockBlobProvider as NativeMockBlobProvider,

        CompileError as NativeCompileError,
        DecompileError as NativeDecompileError,
    };

    pub use scrypto::prelude::{
        NodeId as NativeNodeId,
        EntityType as NativeEntityType,
        
        GlobalAddress as NativeGlobalAddress,
        InternalAddress as NativeInternalAddress,
        ComponentAddress as NativeComponentAddress,
        ResourceAddress as NativeResourceAddress,
        PackageAddress as NativePackageAddress,
        
        NonFungibleLocalId as NativeNonFungibleLocalId,
        NonFungibleGlobalId as NativeNonFungibleGlobalId,

        ParseDecimalError as NativeParseDecimalError,
        ParsePreciseDecimalError as NativeParsePreciseDecimalError,
        ParseNonFungibleLocalIdError as NativeParseNonFungibleLocalIdError,
        ParseNonFungibleGlobalIdError as NativeParseNonFungibleGlobalIdError,
        ParseHashError as NativeParseHashError,
        ParseResourceAddressError as NativeParseResourceAddressError,
        ParseComponentAddressError as NativeParseComponentAddressError,
        ParsePackageAddressError as NativeParsePackageAddressError,
        ParseGlobalAddressError as NativeParseGlobalAddressError,

        Hash as NativeHash,
        hash as native_hash,

        Decimal as NativeDecimal,
        PreciseDecimal as NativePreciseDecimal,
        RoundingMode as NativeRoundingMode,

        XRD as NATIVE_XRD,
        SECP256K1_SIGNATURE_VIRTUAL_BADGE as NATIVE_SECP256K1_SIGNATURE_VIRTUAL_BADGE,
        ED25519_SIGNATURE_VIRTUAL_BADGE as NATIVE_ED25519_SIGNATURE_VIRTUAL_BADGE,
        PACKAGE_OF_DIRECT_CALLER_VIRTUAL_BADGE as NATIVE_PACKAGE_OF_DIRECT_CALLER_VIRTUAL_BADGE,
        GLOBAL_CALLER_VIRTUAL_BADGE as NATIVE_GLOBAL_CALLER_VIRTUAL_BADGE,
        SYSTEM_TRANSACTION_BADGE as NATIVE_SYSTEM_TRANSACTION_BADGE,
        PACKAGE_OWNER_BADGE as NATIVE_PACKAGE_OWNER_BADGE,
        VALIDATOR_OWNER_BADGE as NATIVE_VALIDATOR_OWNER_BADGE,
        ACCOUNT_OWNER_BADGE as NATIVE_ACCOUNT_OWNER_BADGE,
        IDENTITY_OWNER_BADGE as NATIVE_IDENTITY_OWNER_BADGE,
        PACKAGE_PACKAGE as NATIVE_PACKAGE_PACKAGE,
        RESOURCE_PACKAGE as NATIVE_RESOURCE_PACKAGE,
        ACCOUNT_PACKAGE as NATIVE_ACCOUNT_PACKAGE,
        IDENTITY_PACKAGE as NATIVE_IDENTITY_PACKAGE,
        CONSENSUS_MANAGER_PACKAGE as NATIVE_CONSENSUS_MANAGER_PACKAGE,
        ACCESS_CONTROLLER_PACKAGE as NATIVE_ACCESS_CONTROLLER_PACKAGE,
        POOL_PACKAGE as NATIVE_POOL_PACKAGE,
        TRANSACTION_PROCESSOR_PACKAGE as NATIVE_TRANSACTION_PROCESSOR_PACKAGE,
        METADATA_MODULE_PACKAGE as NATIVE_METADATA_MODULE_PACKAGE,
        ROYALTY_MODULE_PACKAGE as NATIVE_ROYALTY_MODULE_PACKAGE,
        ACCESS_RULES_MODULE_PACKAGE as NATIVE_ACCESS_RULES_MODULE_PACKAGE,
        GENESIS_HELPER_PACKAGE as NATIVE_GENESIS_HELPER_PACKAGE,
        GENESIS_HELPER_BLUEPRINT as NATIVE_GENESIS_HELPER_BLUEPRINT,
        FAUCET_PACKAGE as NATIVE_FAUCET_PACKAGE,
        FAUCET_BLUEPRINT as NATIVE_FAUCET_BLUEPRINT,
        TRANSACTION_TRACKER_PACKAGE as NATIVE_TRANSACTION_TRACKER_PACKAGE,
        TRANSACTION_TRACKER_BLUEPRINT as NATIVE_TRANSACTION_TRACKER_BLUEPRINT,
        CONSENSUS_MANAGER as NATIVE_CONSENSUS_MANAGER,
        GENESIS_HELPER as NATIVE_GENESIS_HELPER,
        FAUCET as NATIVE_FAUCET,
        TRANSACTION_TRACKER as NATIVE_TRANSACTION_TRACKER,

        ManifestValue as NativeManifestValue,
        ManifestCustomValue as NativeManifestCustomValue,
        ManifestValueKind as NativeManifestValueKind,
        ManifestCustomValueKind as NativeManifestCustomValueKind,
        
        ScryptoValue as NativeScryptoValue,
        ScryptoCustomValue as NativeScryptoCustomValue,
        ScryptoValueKind as NativeScryptoValueKind,
        ScryptoCustomValueKind as NativeScryptoCustomValueKind,

        ManifestAddress as NativeManifestAddress,
        ManifestBucket as NativeManifestBucket,
        ManifestProof as NativeManifestProof,
        ManifestExpression as NativeManifestExpression,
        ManifestBlobRef as NativeManifestBlobRef,
        ManifestAddressReservation as NativeManifestAddressReservation,

        scrypto_encode as native_scrypto_encode,
        scrypto_decode as native_scrypto_decode,
        manifest_encode as native_manifest_encode,
        manifest_decode as native_manifest_decode,
    };
    pub use scrypto::address::{
        AddressBech32Decoder as NativeAddressBech32Decoder,
        AddressBech32Encoder as NativeAddressBech32Encoder,
    };
    pub use transaction::prelude::{
        InstructionV1 as NativeInstruction,
        InstructionsV1 as NativeInstructions,
        DynamicGlobalAddress as NativeDynamicGlobalAddress,
        DynamicPackageAddress as NativeDynamicPackageAddress,

        TransactionHeaderV1 as NativeTransactionHeader,
        TransactionManifestV1 as NativeTransactionManifest,
        IntentV1 as NativeIntent,
        SignedIntentV1 as NativeSignedIntent,
        NotarizedTransactionV1 as NativeNotarizedTransaction,
        BlobV1 as NativeBlob,
        BlobsV1 as NativeBlobs,

        Epoch as NativeEpoch,

        AesGcmPayload as NativeAesGcmPayload,
        AesWrapped128BitKey as NativeAesWrapped128BitKey,
        CurveType as NativeCurveType,
        DecryptorsByCurve as NativeDecryptorsByCurve,
        EncryptedMessageV1 as NativeEncryptedMessage,
        MessageContentsV1 as NativeMessageContents,
        MessageV1 as NativeMessage,
        PlaintextMessageV1 as NativePlaintextMessage,
        PublicKeyFingerprint as NativePublicKeyFingerprint,

        TransactionPayload as NativeTransactionPayload,
        PrepareError as NativePrepareError,
        HasIntentHash as NativeHasIntentHash,
        HasSignedIntentHash as NativeHasSignedIntentHash,
        HasNotarizedTransactionHash as NativeHasNotarizedTransactionHash,

        IntentSignatureV1 as NativeIntentSignature,
        IntentSignaturesV1 as NativeIntentSignatures,
        NotarySignatureV1 as NativeNotarySignature,
    };
    pub use transaction::validation::{ 
        ValidationConfig as NativeValidationConfig,
        MessageValidationConfig as NativeMessageValidationConfig,
    };
    pub use transaction::errors::{
        TransactionValidationError as NativeTransactionValidationError,
    };
    pub use radix_engine_common::data::scrypto::model::{
        ContentValidationError as NativeContentValidationError,
    };
    pub use radix_engine_common::data::manifest::converter::{
        from_decimal as native_from_decimal,
        from_precise_decimal as native_from_precise_decimal,
        from_non_fungible_local_id as native_from_non_fungible_local_id,
        to_decimal as native_to_decimal,
        to_precise_decimal as native_to_precise_decimal,
        to_non_fungible_local_id as native_to_non_fungible_local_id,
    };
    pub use scrypto::api::node_modules::metadata::{
        MetadataValue as NativeMetadataValue
    };
    pub use sbor::prelude::{
        EncodeError as NativeEncodeError,
        DecodeError as NativeDecodeError,
    };
    pub use sbor::schema::{
        LocalTypeIndex as NativeLocalTypeIndex
    };
    pub use sbor::representations::{
        SerializationMode as NativeSerializationMode
    };
    pub use radix_engine_common::prelude::{
        ScryptoSchema as NativeScryptoSchema,
        SCRYPTO_SBOR_V1_PAYLOAD_PREFIX as NATIVE_SCRYPTO_SBOR_V1_PAYLOAD_PREFIX,
        MANIFEST_SBOR_V1_PAYLOAD_PREFIX as NATIVE_MANIFEST_SBOR_V1_PAYLOAD_PREFIX
    };
    pub use radix_engine::system::system_modules::execution_trace::{
        ResourceSpecifier as NativeResourceSpecifier,
    };
}

pub use self::core::*;
pub use self::native::*;
