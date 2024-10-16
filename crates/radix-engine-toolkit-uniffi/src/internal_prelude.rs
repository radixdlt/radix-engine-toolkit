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

//! This is an internal prelude module meant to only be public within this crate
//! which re-exports types and functions from the core toolkit and Scrypto
//! crates with a prefix. This is done to distinguish between the core toolkit
//! types, scrypto types, and types defined in this crate. We choose the types
//! in this crate to have short un-prefixed names since those are the names that
//! the types will be generated with when using UniFFI

#[rustfmt::skip]
#[allow(unused_braces)]
mod core {
    /* Models */
    pub use radix_engine_toolkit::models::node_id::{
        TypedNodeId as CoreTypedNodeId,
        InvalidEntityTypeIdError as CoreInvalidEntityTypeIdError
    };
    /* Modules */
    pub use radix_engine_toolkit::functions::information::{
        information as core_information,
        BuildInformation as CoreBuildInformation,
        DependencyInformation as CoreDependencyInformation,
    };
    pub use radix_engine_toolkit::functions::derive::{
        virtual_account_address_from_public_key as core_virtual_account_address_from_public_key,
        virtual_identity_address_from_public_key as core_virtual_identity_address_from_public_key,
        virtual_signature_non_fungible_global_id_from_public_key as core_virtual_signature_non_fungible_global_id_from_public_key,
        virtual_global_caller_non_fungible_global_id_from_component_address as core_virtual_global_caller_non_fungible_global_id_from_component_address,
        virtual_package_of_direct_caller_non_fungible_global_id_from_component_address as core_virtual_package_of_direct_caller_non_fungible_global_id_from_component_address,
        virtual_account_address_from_olympia_account_address as core_virtual_account_address_from_olympia_account_address,
        resource_address_from_olympia_resource_address as core_resource_address_from_olympia_resource_address,
        public_key_from_olympia_account_address as core_public_key_from_olympia_account_address,
        olympia_account_address_from_public_key as core_olympia_account_address_from_public_key,
        node_address_from_public_key as core_node_address_from_public_key,
        DerivationError as CoreDerivationError,
        OlympiaNetwork as CoreOlympiaNetwork,
    };
    pub use radix_engine_toolkit::functions::instructions::{
        hash as core_instructions_hash,
        compile as core_instructions_compile,
        decompile as core_instructions_decompile,
        statically_validate as core_instructions_statically_validate,
        extract_addresses as core_instructions_extract_addresses,
        InstructionValidationError as CoreInstructionValidationError,
    };
    pub use radix_engine_toolkit::functions::utils::{
        decode_transaction_id as core_decode_transaction_id
    };
    pub use radix_engine_toolkit::functions::manifest::{
        hash as core_manifest_hash,
        compile as core_manifest_compile,
        decompile as core_manifest_decompile,
        statically_validate as core_manifest_statically_validate,
        modify as core_manifest_modify,
        summary as core_manifest_summary,
        execution_summary as core_manifest_execution_summary,
        TransactionManifestModifications as CoreManifestTransactionManifestModifications,
        Assertion as CoreManifestAssertion,
        ManifestModificationError as CoreManifestModificationError
    };
    pub use radix_engine_toolkit::functions::intent::{
        hash as core_intent_hash,
        compile as core_intent_compile,
        decompile as core_intent_decompile,
        statically_validate as core_intent_statically_validate,
    };
    pub use radix_engine_toolkit::functions::signed_intent::{
        hash as core_signed_intent_hash,
        compile as core_signed_intent_compile,
        decompile as core_signed_intent_decompile,
        statically_validate as core_signed_intent_statically_validate,
    };
    pub use radix_engine_toolkit::functions::notarized_transaction::{
        hash as core_notarized_transaction_hash,
        compile as core_notarized_transaction_compile,
        decompile as core_notarized_transaction_decompile,
        statically_validate as core_notarized_transaction_statically_validate,
    };
    pub use radix_engine_toolkit::functions::manifest_sbor::{
        ManifestSborError as CoreManifestSborError,
        ManifestSborStringRepresentation as CoreManifestSborStringRepresentation,
        decode_to_string_representation as core_manifest_decode_to_string_representation,
    };
    pub use radix_engine_toolkit::functions::scrypto_sbor::{
        ScryptoSborError as CoreScryptoSborError,
        StringRepresentation as CoreScryptoStringRepresentation,
        decode_to_string_representation as core_scrypto_decode_to_string_representation,
        encode_string_representation as core_scrypto_encode_string_representation,
    };
    pub use radix_engine_toolkit::functions::events::{
        sbor_decode_to_native_event as core_events_sbor_decode_to_native_event
    };

    /* TransactionTypes */
    pub use radix_engine_toolkit::transaction_types::{
        ManifestSummaryCallback as CoreManifestSummaryCallback,
        ExecutionSummaryCallback as CoreExecutionSummaryCallback,
        TransactionTypesError as CoreTransactionTypesError,
        ManifestSummary as CoreManifestSummary,
        ExecutionSummary as CoreExecutionSummary,
        TransactionTypesReceipt as CoreTransactionTypesReceipt,
        NewEntities as CoreNewEntities,
        FnRules as CoreFnRules,
        Predicted as CorePredicted,
        FeeSummary as CoreFeeSummary,
        ManifestClass as CoreManifestClass,
        DetailedManifestClass as CoreDetailedManifestClass,
        ReservedInstruction as CoreReservedInstruction,
        FnRule as CoreFnRule,
        ResourceIndicator as CoreResourceIndicator,
        FungibleResourceIndicator as CoreFungibleResourceIndicator,
        NonFungibleResourceIndicator as CoreNonFungibleResourceIndicator,
        Update as CoreUpdate,
        Operation as CoreOperation,
        EncounteredGlobalEntities as CoreEncounteredGlobalEntities,
        PresentedProofsDetector as CorePresentedProofsDetector,
        RequiringAuthDetector as CoreRequiringAuthDetector,
        ReservedInstructionsDetector as CoreReservedInstructionsDetector,
        StaticAccountResourceMovementsDetector as CoreStaticAccountResourceMovementsDetector,
        AccountResourceMovementsDetector as CoreAccountResourceMovementsDetector,
        AccountSettingsUpdateDetector as CoreAccountSettingsUpdateDetector,
        GeneralDetector as CoreGeneralDetector,
        PoolContributionDetector as CorePoolContributionDetector,
        TrackedPoolContribution as CoreTrackedPoolContribution,
        PoolRedemptionDetector as CorePoolRedemptionDetector,
        TrackedPoolRedemption as CoreTrackedPoolRedemption,
        TransferDetector as CoreTransferDetector,
        ValidatorStakeDetector as CoreValidatorStakeDetector,
        TrackedValidatorStake as CoreTrackedValidatorStake,
        ValidatorUnstakeDetector as CoreValidatorUnstakeDetector,
        TrackedValidatorUnstake as CoreTrackedValidatorUnstake,
        ValidatorClaimDetector as CoreValidatorClaimDetector,
        TrackedValidatorClaim as CoreTrackedValidatorClaim,
        summary as core_summary,
        execution_summary as core_execution_summary,
    };

    /* Utils */
    pub use radix_engine_toolkit::utils::{
        manifest_from_intent as core_manifest_from_intent,
        network_definition_from_network_id as core_network_definition_from_network_id,
        network_id_from_hrp as core_network_id_from_hrp,
        network_id_from_address_string as core_network_id_from_address_string,
        to_manifest_type as core_to_manifest_type,
        validate_manifest_value_against_schema as core_validate_manifest_value_against_schema,
        is_account as core_is_account,
        is_identity as core_is_identity,
    };
}

#[rustfmt::skip]
#[allow(unused_braces)]
mod native {
    pub use radix_transactions::prelude::{
        /* Cryptography */
        Signer as NativeSigner,
        PrivateKey as NativePrivateKey,

        SignatureV1 as NativeSignature,

        SignatureWithPublicKeyV1 as NativeSignatureWithPublicKey,
    };
    pub use radix_common::prelude::{
        Epoch as NativeEpoch,
        IsHash as NativeIsHash,
        FromPublicKey as NativeFromPublicKey,

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

        Ed25519Signature as NativeEd25519Signature,
        Secp256k1Signature as NativeSecp256k1Signature,
    };
    pub use radix_transactions::manifest::{
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
        InnerDecimal as NativeInnerDecimal,
        PreciseDecimal as NativePreciseDecimal,
        InnerPreciseDecimal as NativeInnerPreciseDecimal,
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
        ROLE_ASSIGNMENT_MODULE_PACKAGE as NATIVE_ROLE_ASSIGNMENT_MODULE_PACKAGE,
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
        LOCKER_PACKAGE as NATIVE_LOCKER_PACKAGE,

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

        Instant as NativeInstant,
        UncheckedOrigin as NativeUncheckedOrigin,
        UncheckedUrl as NativeUncheckedUrl,
        CheckedOrigin as NativeCheckedOrigin,
        CheckedUrl as NativeCheckedUrl,

        ResourceOrNonFungible as NativeResourceOrNonFungible,
        ResourceOrNonFungibleList as NativeResourceOrNonFungibleList,
        ProofRule as NativeProofRule,
        AccessRule as NativeAccessRule,
        AccessRuleNode as NativeAccessRuleNode,
        OwnerRole as NativeOwnerRole,
        RoleAssignmentInit as NativeRoleAssignmentInit,
        RoleKey as NativeRoleKey,
        MintRoles as NativeMintRoles,
        BurnRoles as NativeBurnRoles,
        FreezeRoles as NativeFreezeRoles,
        RecallRoles as NativeRecallRoles,
        WithdrawRoles as NativeWithdrawRoles,
        DepositRoles as NativeDepositRoles,
        RoleDefinition as NativeRoleDefinition,
        manifest_args as native_manifest_args,
        rule as native_rule,
        require as native_require,

        dec as native_dec,
        CheckedAdd as NativeCheckedAdd,
        CheckedSub as NativeCheckedSub,
        CheckedMul as NativeCheckedMul,
        CheckedDiv as NativeCheckedDiv,
        CheckedNeg as NativeCheckedNeg,
        CheckedTruncate as NativeCheckedTruncate,

        RoyaltyAmount as NativeRoyaltyAmount,
        WithdrawStrategy as NativeWithdrawStrategy,
    };
    pub use scrypto::address::{
        AddressBech32Decoder as NativeAddressBech32Decoder,
        AddressBech32Encoder as NativeAddressBech32Encoder,
    };
    pub use radix_transactions::prelude::{
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

        IntentHash as NativeIntentHash,
        SignedIntentHash as NativeSignedIntentHash,
        NotarizedTransactionHash as NativeNotarizedTransactionHash,

        HashHasHrp as NativeHashHasHrp,
        TransactionHashBech32Encoder as NativeTransactionHashBech32Encoder,
        TransactionHashBech32Decoder as NativeTransactionHashBech32Decoder,

        NamedManifestAddress as NativeNamedManifestAddress
    };
    pub use radix_transactions::validation::{
        ValidationConfig as NativeValidationConfig,
        MessageValidationConfig as NativeMessageValidationConfig,
        ManifestIdAllocator as NativeManifestIdAllocator
    };
    pub use radix_transactions::errors::{
        TransactionValidationError as NativeTransactionValidationError,
    };
    pub use radix_common::data::scrypto::model::{
        ContentValidationError as NativeContentValidationError,
    };
    pub use radix_common::data::manifest::converter::{
        from_decimal as native_from_decimal,
        from_precise_decimal as native_from_precise_decimal,
        from_non_fungible_local_id as native_from_non_fungible_local_id,
        to_decimal as native_to_decimal,
        to_precise_decimal as native_to_precise_decimal,
        to_non_fungible_local_id as native_to_non_fungible_local_id,
    };
    pub use scrypto::object_modules:: {
        ModuleConfig as NativeModuleConfig,
    };
    pub use scrypto::object_modules::metadata::{
        MetadataValue as NativeMetadataValue,
        MetadataInit as NativeMetadataInit,
    };
    pub use sbor::prelude::{
        EncodeError as NativeEncodeError,
        DecodeError as NativeDecodeError,
        IndexSet
    };
    pub use sbor::schema::{
        LocalTypeId as NativeLocalTypeId
    };
    pub use sbor::representations::{
        SerializationMode as NativeSerializationMode
    };
    pub use radix_common::prelude::{
        to_manifest_value_and_unwrap as native_to_manifest_value_and_unwrap,
        Schema as NativeSchema,
        VersionedSchema as NativeVersionedSchema,
        ScryptoCustomSchema as NativeScryptoCustomSchema,
        SCRYPTO_SBOR_V1_PAYLOAD_PREFIX as NATIVE_SCRYPTO_SBOR_V1_PAYLOAD_PREFIX,
        MANIFEST_SBOR_V1_PAYLOAD_PREFIX as NATIVE_MANIFEST_SBOR_V1_PAYLOAD_PREFIX,
    };
    pub use radix_engine::transaction::{
        FeeLocks as NativeFeeLocks
    };
    pub use radix_engine::system::system_modules::execution_trace::{
        ResourceSpecifier as NativeResourceSpecifier,
    };
    pub use radix_engine_interface::blueprints::locker::{
        ResourceSpecifier as NativeLockerResourceSpecifier,
    };
    pub use radix_engine::transaction::{
        VersionedTransactionReceipt as NativeVersionedTransactionReceipt,
        TransactionReceipt as NativeTransactionReceipt,
        CommitResult as NativeCommitResult
    };
    pub use radix_engine::blueprints::account::{
        AccountNativePackage as NativeAccountNativePackage,
        AccountBlueprint as NativeAccountBlueprint,
        WithdrawEvent as NativeAccountWithdrawEvent,
        DepositEvent as NativeAccountDepositEvent,
        RejectedDepositEvent as NativeAccountRejectedDepositEvent,
        SetResourcePreferenceEvent as NativeAccountSetResourcePreferenceEvent,
        RemoveResourcePreferenceEvent as NativeAccountRemoveResourcePreferenceEvent,
        SetDefaultDepositRuleEvent as NativeAccountSetDefaultDepositRuleEvent,
        AddAuthorizedDepositorEvent as NativeAccountAddAuthorizedDepositorEvent,
        RemoveAuthorizedDepositorEvent as NativeAccountRemoveAuthorizedDepositorEvent,
    };
    pub use radix_engine::blueprints::identity::{
        IdentityNativePackage as NativeIdentityNativePackage,
        IdentityBlueprint as NativeIdentityBlueprint
    };
    pub use radix_engine::blueprints::consensus_manager::{
        UnstakeData as NativeUnstakeData
    };

    pub use scrypto::blueprints::access_controller::{
        Role as NativeRole,
        Proposer as NativeProposer,
        RuleSet as NativeRuleSet,
        RecoveryProposal as NativeRecoveryProposal
    };

    pub use radix_engine::blueprints::consensus_manager::{
        Validator as NativeValidator,
        ActiveValidatorSet as NativeActiveValidatorSet
    };

    pub use radix_engine::blueprints::access_controller::latest::{
        InitiateRecoveryEvent as NativeInitiateRecoveryEvent,
        InitiateBadgeWithdrawAttemptEvent as NativeInitiateBadgeWithdrawAttemptEvent,
        RuleSetUpdateEvent as NativeRuleSetUpdateEvent,
        BadgeWithdrawEvent as NativeBadgeWithdrawEvent,
        CancelRecoveryProposalEvent as NativeCancelRecoveryProposalEvent,
        CancelBadgeWithdrawAttemptEvent as NativeCancelBadgeWithdrawAttemptEvent,
        LockPrimaryRoleEvent as NativeLockPrimaryRoleEvent,
        UnlockPrimaryRoleEvent as NativeUnlockPrimaryRoleEvent,
        StopTimedRecoveryEvent as NativeStopTimedRecoveryEvent,
        DepositRecoveryXrdEvent as NativeDepositRecoveryXrdEvent,
        WithdrawRecoveryXrdEvent as NativeWithdrawRecoveryXrdEvent
    };
    pub use radix_engine::blueprints::consensus_manager::{
        RoundChangeEvent as NativeRoundChangeEvent,
        EpochChangeEvent as NativeEpochChangeEvent,
        RegisterValidatorEvent as NativeRegisterValidatorEvent,
        UnregisterValidatorEvent as NativeUnregisterValidatorEvent,
        StakeEvent as NativeStakeEvent,
        UnstakeEvent as NativeUnstakeEvent,
        ClaimXrdEvent as NativeClaimXrdEvent,
        UpdateAcceptingStakeDelegationStateEvent as NativeUpdateAcceptingStakeDelegationStateEvent,
        ProtocolUpdateReadinessSignalEvent as NativeProtocolUpdateReadinessSignalEvent,
        ValidatorEmissionAppliedEvent as NativeValidatorEmissionAppliedEvent,
        ValidatorRewardAppliedEvent as NativeValidatorRewardAppliedEvent,
    };
    pub use radix_engine::blueprints::pool::v1::events::one_resource_pool::{
        ContributionEvent as NativeOneResourcePoolContributionEvent,
        RedemptionEvent as NativeOneResourcePoolRedemptionEvent,
        WithdrawEvent as NativeOneResourcePoolWithdrawEvent,
        DepositEvent as NativeOneResourcePoolDepositEvent,
    };
    pub use radix_engine::blueprints::pool::v1::events::two_resource_pool::{
        ContributionEvent as NativeTwoResourcePoolContributionEvent,
        RedemptionEvent as NativeTwoResourcePoolRedemptionEvent,
        WithdrawEvent as NativeTwoResourcePoolWithdrawEvent,
        DepositEvent as NativeTwoResourcePoolDepositEvent,
    };
    pub use radix_engine::blueprints::pool::v1::events::multi_resource_pool::{
        ContributionEvent as NativeMultiResourcePoolContributionEvent,
        RedemptionEvent as NativeMultiResourcePoolRedemptionEvent,
        WithdrawEvent as NativeMultiResourcePoolWithdrawEvent,
        DepositEvent as NativeMultiResourcePoolDepositEvent,
    };
    pub use radix_engine::blueprints::pool::v1::constants::{
        POOL_MANAGER_ROLE as NATIVE_POOL_MANAGER_ROLE,
        ONE_RESOURCE_POOL_BLUEPRINT_IDENT as NATIVE_ONE_RESOURCE_POOL_BLUEPRINT_IDENT,
        TWO_RESOURCE_POOL_BLUEPRINT_IDENT as NATIVE_TWO_RESOURCE_POOL_BLUEPRINT_IDENT,
        MULTI_RESOURCE_POOL_BLUEPRINT_IDENT as NATIVE_MULTI_RESOURCE_POOL_BLUEPRINT_IDENT
    };
    pub use radix_engine::blueprints::resource::{
        VaultCreationEvent as NativeVaultCreationEvent,
        MintFungibleResourceEvent as NativeMintFungibleResourceEvent,
        BurnFungibleResourceEvent as NativeBurnFungibleResourceEvent,
        MintNonFungibleResourceEvent as NativeMintNonFungibleResourceEvent,
        BurnNonFungibleResourceEvent as NativeBurnNonFungibleResourceEvent,
        fungible_vault::LockFeeEvent as NativeFungibleVaultLockFeeEvent,
        fungible_vault::WithdrawEvent as NativeFungibleVaultWithdrawEvent,
        fungible_vault::DepositEvent as NativeFungibleVaultDepositEvent,
        fungible_vault::RecallEvent as NativeFungibleVaultRecallEvent,
        fungible_vault::PayFeeEvent as NativeFungibleVaultPayFeeEvent,
        non_fungible_vault::WithdrawEvent as NativeNonFungibleVaultWithdrawEvent,
        non_fungible_vault::DepositEvent as NativeNonFungibleVaultDepositEvent,
        non_fungible_vault::RecallEvent as NativeNonFungibleVaultRecallEvent,
    };
    pub use radix_engine::object_modules::role_assignment::{
        SetRoleEvent as NativeSetRoleEvent,
        SetOwnerRoleEvent as NativeSetOwnerRoleEvent,
        LockOwnerRoleEvent as NativeLockOwnerRoleEvent,
    };
    pub use radix_engine::object_modules::metadata::{
        SetMetadataEvent as NativeSetMetadataEvent,
        RemoveMetadataEvent as NativeRemoveMetadataEvent,
    };
    pub use radix_substate_store_queries::typed_native_events::{
        TypedNativeEvent as NativeTypedNativeEvent,
        TypedNativeEventError as NativeTypedNativeEventError
    };
    pub use radix_engine_interface::types::{
        KeyValueStoreInitEntry as NativeKeyValueStoreInitEntry,
        BlueprintId as NativeBlueprintId
    };
    pub use radix_engine_interface::blueprints::locker::{
        ACCOUNT_LOCKER_BLUEPRINT as NATIVE_ACCOUNT_LOCKER_BLUEPRINT,
        ACCOUNT_LOCKER_INSTANTIATE_IDENT as NATIVE_ACCOUNT_LOCKER_INSTANTIATE_IDENT,
        ACCOUNT_LOCKER_INSTANTIATE_SIMPLE_IDENT as NATIVE_ACCOUNT_LOCKER_INSTANTIATE_SIMPLE_IDENT,
        ACCOUNT_LOCKER_STORE_IDENT as NATIVE_ACCOUNT_LOCKER_STORE_IDENT,
        ACCOUNT_LOCKER_AIRDROP_IDENT as NATIVE_ACCOUNT_LOCKER_AIRDROP_IDENT,
        ACCOUNT_LOCKER_RECOVER_IDENT as NATIVE_ACCOUNT_LOCKER_RECOVER_IDENT,
        ACCOUNT_LOCKER_RECOVER_NON_FUNGIBLES_IDENT as NATIVE_ACCOUNT_LOCKER_RECOVER_NON_FUNGIBLES_IDENT,
        ACCOUNT_LOCKER_CLAIM_IDENT as NATIVE_ACCOUNT_LOCKER_CLAIM_IDENT,
        ACCOUNT_LOCKER_CLAIM_NON_FUNGIBLES_IDENT as NATIVE_ACCOUNT_LOCKER_CLAIM_NON_FUNGIBLES_IDENT,
        ACCOUNT_LOCKER_GET_AMOUNT_IDENT as NATIVE_ACCOUNT_LOCKER_GET_AMOUNT_IDENT,
        ACCOUNT_LOCKER_GET_NON_FUNGIBLE_LOCAL_IDS_IDENT as NATIVE_ACCOUNT_LOCKER_GET_NON_FUNGIBLE_LOCAL_IDS_IDENT,
        AccountLockerInstantiateManifestInput as NativeAccountLockerInstantiateManifestInput,
        AccountLockerInstantiateSimpleManifestInput as NativeAccountLockerInstantiateSimpleManifestInput,
        AccountLockerStoreManifestInput as NativeAccountLockerStoreManifestInput,
        AccountLockerAirdropManifestInput as NativeAccountLockerAirdropManifestInput,
        AccountLockerRecoverManifestInput as NativeAccountLockerRecoverManifestInput,
        AccountLockerRecoverNonFungiblesManifestInput as NativeAccountLockerRecoverNonFungiblesManifestInput,
        AccountLockerClaimManifestInput as NativeAccountLockerClaimManifestInput,
        AccountLockerClaimNonFungiblesManifestInput as NativeAccountLockerClaimNonFungiblesManifestInput,
        AccountLockerGetAmountManifestInput as NativeAccountLockerGetAmountManifestInput,
        AccountLockerGetNonFungibleLocalIdsManifestInput as NativeAccountLockerGetNonFungibleLocalIdsManifestInput,
    };
    pub use radix_engine::blueprints::locker::{
        ClaimEvent as NativeClaimEvent,
        RecoverEvent as NativeRecoverEvent,
        StoreEvent as NativeStoreEvent
    };
    pub use radix_engine_interface::object_modules::royalty::{
        COMPONENT_ROYALTY_SETTER_ROLE as NATIVE_COMPONENT_ROYALTY_SETTER_ROLE,
        COMPONENT_ROYALTY_SETTER_UPDATER_ROLE as NATIVE_COMPONENT_ROYALTY_SETTER_UPDATER_ROLE,
        COMPONENT_ROYALTY_LOCKER_ROLE as NATIVE_COMPONENT_ROYALTY_LOCKER_ROLE,
        COMPONENT_ROYALTY_LOCKER_UPDATER_ROLE as NATIVE_COMPONENT_ROYALTY_LOCKER_UPDATER_ROLE,
        COMPONENT_ROYALTY_CLAIMER_ROLE as NATIVE_COMPONENT_ROYALTY_CLAIMER_ROLE,
        COMPONENT_ROYALTY_CLAIMER_UPDATER_ROLE as NATIVE_COMPONENT_ROYALTY_CLAIMER_UPDATER_ROLE,
        COMPONENT_ROYALTY_BLUEPRINT as NATIVE_COMPONENT_ROYALTY_BLUEPRINT,
        COMPONENT_ROYALTY_CREATE_IDENT as NATIVE_COMPONENT_ROYALTY_CREATE_IDENT,
        COMPONENT_ROYALTY_SET_ROYALTY_IDENT as NATIVE_COMPONENT_ROYALTY_SET_ROYALTY_IDENT,
        COMPONENT_ROYALTY_LOCK_ROYALTY_IDENT as NATIVE_COMPONENT_ROYALTY_LOCK_ROYALTY_IDENT,
        COMPONENT_ROYALTY_CLAIM_ROYALTIES_IDENT as NATIVE_COMPONENT_ROYALTY_CLAIM_ROYALTIES_IDENT,
        ComponentRoyaltyCreateInput as NativeComponentRoyaltyCreateInput,
        ComponentRoyaltySetInput as NativeComponentRoyaltySetInput,
        ComponentRoyaltyLockInput as NativeComponentRoyaltyLockInput,
        ComponentClaimRoyaltiesInput as NativeComponentClaimRoyaltiesInput,
        ComponentRoyaltyCreateOutput as NativeComponentRoyaltyCreateOutput,
        ComponentRoyaltySetOutput as NativeComponentRoyaltySetOutput,
        ComponentRoyaltyLockOutput as NativeComponentRoyaltyLockOutput,
        ComponentClaimRoyaltiesOutput as NativeComponentClaimRoyaltiesOutput,
    };
    pub use radix_engine_interface::prelude::{
        RoleAssignmentCreateInput as NativeRoleAssignmentCreateInput,
        RoleAssignmentSetInput as NativeRoleAssignmentSetInput,
        RoleAssignmentSetOwnerInput as NativeRoleAssignmentSetOwnerInput,
        RoleAssignmentLockOwnerInput as NativeRoleAssignmentLockOwnerInput,
        RoleAssignmentGetInput as NativeRoleAssignmentGetInput,
        ROLE_ASSIGNMENT_BLUEPRINT as NATIVE_ROLE_ASSIGNMENT_BLUEPRINT,
        ROLE_ASSIGNMENT_CREATE_IDENT as NATIVE_ROLE_ASSIGNMENT_CREATE_IDENT,
        ROLE_ASSIGNMENT_SET_IDENT as NATIVE_ROLE_ASSIGNMENT_SET_IDENT,
        ROLE_ASSIGNMENT_SET_OWNER_IDENT as NATIVE_ROLE_ASSIGNMENT_SET_OWNER_IDENT,
        ROLE_ASSIGNMENT_LOCK_OWNER_IDENT as NATIVE_ROLE_ASSIGNMENT_LOCK_OWNER_IDENT,
        ROLE_ASSIGNMENT_GET_IDENT as NATIVE_ROLE_ASSIGNMENT_GET_IDENT,

        METADATA_BLUEPRINT as NATIVE_METADATA_BLUEPRINT,
        METADATA_VALUE_STRING_DISCRIMINATOR as NATIVE_METADATA_VALUE_STRING_DISCRIMINATOR,
        METADATA_VALUE_BOOLEAN_DISCRIMINATOR as NATIVE_METADATA_VALUE_BOOLEAN_DISCRIMINATOR,
        METADATA_VALUE_U8_DISCRIMINATOR as NATIVE_METADATA_VALUE_U8_DISCRIMINATOR,
        METADATA_VALUE_U32_DISCRIMINATOR as NATIVE_METADATA_VALUE_U32_DISCRIMINATOR,
        METADATA_VALUE_U64_DISCRIMINATOR as NATIVE_METADATA_VALUE_U64_DISCRIMINATOR,
        METADATA_VALUE_I32_DISCRIMINATOR as NATIVE_METADATA_VALUE_I32_DISCRIMINATOR,
        METADATA_VALUE_I64_DISCRIMINATOR as NATIVE_METADATA_VALUE_I64_DISCRIMINATOR,
        METADATA_VALUE_DECIMAL_DISCRIMINATOR as NATIVE_METADATA_VALUE_DECIMAL_DISCRIMINATOR,
        METADATA_VALUE_GLOBAL_ADDRESS_DISCRIMINATOR as NATIVE_METADATA_VALUE_GLOBAL_ADDRESS_DISCRIMINATOR,
        METADATA_VALUE_PUBLIC_KEY_DISCRIMINATOR as NATIVE_METADATA_VALUE_PUBLIC_KEY_DISCRIMINATOR,
        METADATA_VALUE_NON_FUNGIBLE_GLOBAL_ID_DISCRIMINATOR as NATIVE_METADATA_VALUE_NON_FUNGIBLE_GLOBAL_ID_DISCRIMINATOR,
        METADATA_VALUE_NON_FUNGIBLE_LOCAL_ID_DISCRIMINATOR as NATIVE_METADATA_VALUE_NON_FUNGIBLE_LOCAL_ID_DISCRIMINATOR,
        METADATA_VALUE_INSTANT_DISCRIMINATOR as NATIVE_METADATA_VALUE_INSTANT_DISCRIMINATOR,
        METADATA_VALUE_URL_DISCRIMINATOR as NATIVE_METADATA_VALUE_URL_DISCRIMINATOR,
        METADATA_VALUE_ORIGIN_DISCRIMINATOR as NATIVE_METADATA_VALUE_ORIGIN_DISCRIMINATOR,
        METADATA_VALUE_PUBLIC_KEY_HASH_DISCRIMINATOR as NATIVE_METADATA_VALUE_PUBLIC_KEY_HASH_DISCRIMINATOR,
        METADATA_DISCRIMINATOR_ARRAY_BASE as NATIVE_METADATA_DISCRIMINATOR_ARRAY_BASE,
        METADATA_VALUE_STRING_ARRAY_DISCRIMINATOR as NATIVE_METADATA_VALUE_STRING_ARRAY_DISCRIMINATOR,
        METADATA_VALUE_BOOLEAN_ARRAY_DISCRIMINATOR as NATIVE_METADATA_VALUE_BOOLEAN_ARRAY_DISCRIMINATOR,
        METADATA_VALUE_U8_ARRAY_DISCRIMINATOR as NATIVE_METADATA_VALUE_U8_ARRAY_DISCRIMINATOR,
        METADATA_VALUE_U32_ARRAY_DISCRIMINATOR as NATIVE_METADATA_VALUE_U32_ARRAY_DISCRIMINATOR,
        METADATA_VALUE_U64_ARRAY_DISCRIMINATOR as NATIVE_METADATA_VALUE_U64_ARRAY_DISCRIMINATOR,
        METADATA_VALUE_I32_ARRAY_DISCRIMINATOR as NATIVE_METADATA_VALUE_I32_ARRAY_DISCRIMINATOR,
        METADATA_VALUE_I64_ARRAY_DISCRIMINATOR as NATIVE_METADATA_VALUE_I64_ARRAY_DISCRIMINATOR,
        METADATA_VALUE_DECIMAL_ARRAY_DISCRIMINATOR as NATIVE_METADATA_VALUE_DECIMAL_ARRAY_DISCRIMINATOR,
        METADATA_VALUE_GLOBAL_ADDRESS_ARRAY_DISCRIMINATOR as NATIVE_METADATA_VALUE_GLOBAL_ADDRESS_ARRAY_DISCRIMINATOR,
        METADATA_VALUE_PUBLIC_KEY_ARRAY_DISCRIMINATOR as NATIVE_METADATA_VALUE_PUBLIC_KEY_ARRAY_DISCRIMINATOR,
        METADATA_VALUE_NON_FUNGIBLE_GLOBAL_ID_ARRAY_DISCRIMINATOR as NATIVE_METADATA_VALUE_NON_FUNGIBLE_GLOBAL_ID_ARRAY_DISCRIMINATOR,
        METADATA_VALUE_NON_FUNGIBLE_LOCAL_ID_ARRAY_DISCRIMINATOR as NATIVE_METADATA_VALUE_NON_FUNGIBLE_LOCAL_ID_ARRAY_DISCRIMINATOR,
        METADATA_VALUE_INSTANT_ARRAY_DISCRIMINATOR as NATIVE_METADATA_VALUE_INSTANT_ARRAY_DISCRIMINATOR,
        METADATA_VALUE_URL_ARRAY_DISCRIMINATOR as NATIVE_METADATA_VALUE_URL_ARRAY_DISCRIMINATOR,
        METADATA_VALUE_ORIGIN_ARRAY_DISCRIMINATOR as NATIVE_METADATA_VALUE_ORIGIN_ARRAY_DISCRIMINATOR,
        METADATA_VALUE_PUBLIC_KEY_HASH_ARRAY_DISCRIMINATOR as NATIVE_METADATA_VALUE_PUBLIC_KEY_HASH_ARRAY_DISCRIMINATOR,
        METADATA_SETTER_ROLE as NATIVE_METADATA_SETTER_ROLE,
        METADATA_SETTER_UPDATER_ROLE as NATIVE_METADATA_SETTER_UPDATER_ROLE,
        METADATA_LOCKER_ROLE as NATIVE_METADATA_LOCKER_ROLE,
        METADATA_LOCKER_UPDATER_ROLE as NATIVE_METADATA_LOCKER_UPDATER_ROLE,
        METADATA_CREATE_IDENT as NATIVE_METADATA_CREATE_IDENT,
        METADATA_CREATE_WITH_DATA_IDENT as NATIVE_METADATA_CREATE_WITH_DATA_IDENT,
        METADATA_SET_IDENT as NATIVE_METADATA_SET_IDENT,
        METADATA_LOCK_IDENT as NATIVE_METADATA_LOCK_IDENT,
        METADATA_GET_IDENT as NATIVE_METADATA_GET_IDENT,
        METADATA_REMOVE_IDENT as NATIVE_METADATA_REMOVE_IDENT,
        MetadataCreateInput as NativeMetadataCreateInput,
        MetadataCreateWithDataInput as NativeMetadataCreateWithDataInput,
        MetadataSetInput as NativeMetadataSetInput,
        MetadataLockInput as NativeMetadataLockInput,
        MetadataGetInput as NativeMetadataGetInput,
        MetadataRemoveInput as NativeMetadataRemoveInput,
    };
    pub use radix_engine_interface::blueprints::access_controller::{
        AccessControllerCreateInput as NativeAccessControllerCreateInput,
        AccessControllerCreateManifestInput as NativeAccessControllerCreateManifestInput,
        AccessControllerCreateProofInput as NativeAccessControllerCreateProofInput,
        AccessControllerInitiateRecoveryAsPrimaryInput as NativeAccessControllerInitiateRecoveryAsPrimaryInput,
        AccessControllerInitiateRecoveryAsRecoveryInput as NativeAccessControllerInitiateRecoveryAsRecoveryInput,
        AccessControllerInitiateBadgeWithdrawAttemptAsPrimaryInput as NativeAccessControllerInitiateBadgeWithdrawAttemptAsPrimaryInput,
        AccessControllerInitiateBadgeWithdrawAttemptAsRecoveryInput as NativeAccessControllerInitiateBadgeWithdrawAttemptAsRecoveryInput,
        AccessControllerQuickConfirmPrimaryRoleRecoveryProposalInput as NativeAccessControllerQuickConfirmPrimaryRoleRecoveryProposalInput,
        AccessControllerQuickConfirmRecoveryRoleRecoveryProposalInput as NativeAccessControllerQuickConfirmRecoveryRoleRecoveryProposalInput,
        AccessControllerQuickConfirmPrimaryRoleBadgeWithdrawAttemptInput as NativeAccessControllerQuickConfirmPrimaryRoleBadgeWithdrawAttemptInput,
        AccessControllerQuickConfirmRecoveryRoleBadgeWithdrawAttemptInput as NativeAccessControllerQuickConfirmRecoveryRoleBadgeWithdrawAttemptInput,
        AccessControllerTimedConfirmRecoveryInput as NativeAccessControllerTimedConfirmRecoveryInput,
        AccessControllerCancelPrimaryRoleRecoveryProposalInput as NativeAccessControllerCancelPrimaryRoleRecoveryProposalInput,
        AccessControllerCancelRecoveryRoleRecoveryProposalInput as NativeAccessControllerCancelRecoveryRoleRecoveryProposalInput,
        AccessControllerCancelPrimaryRoleBadgeWithdrawAttemptInput as NativeAccessControllerCancelPrimaryRoleBadgeWithdrawAttemptInput,
        AccessControllerCancelRecoveryRoleBadgeWithdrawAttemptInput as NativeAccessControllerCancelRecoveryRoleBadgeWithdrawAttemptInput,
        AccessControllerLockPrimaryRoleInput as NativeAccessControllerLockPrimaryRoleInput,
        AccessControllerUnlockPrimaryRoleInput as NativeAccessControllerUnlockPrimaryRoleInput,
        AccessControllerStopTimedRecoveryInput as NativeAccessControllerStopTimedRecoveryInput,
        AccessControllerMintRecoveryBadgesInput as NativeAccessControllerMintRecoveryBadgesInput,
        ACCESS_CONTROLLER_BLUEPRINT as NATIVE_ACCESS_CONTROLLER_BLUEPRINT,
        ACCESS_CONTROLLER_CREATE_IDENT as NATIVE_ACCESS_CONTROLLER_CREATE_IDENT,
        ACCESS_CONTROLLER_CREATE_PROOF_IDENT as NATIVE_ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
        ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_PRIMARY_IDENT as NATIVE_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_PRIMARY_IDENT,
        ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT as NATIVE_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
        ACCESS_CONTROLLER_INITIATE_BADGE_WITHDRAW_ATTEMPT_AS_PRIMARY_IDENT as NATIVE_ACCESS_CONTROLLER_INITIATE_BADGE_WITHDRAW_ATTEMPT_AS_PRIMARY_IDENT,
        ACCESS_CONTROLLER_INITIATE_BADGE_WITHDRAW_ATTEMPT_AS_RECOVERY_IDENT as NATIVE_ACCESS_CONTROLLER_INITIATE_BADGE_WITHDRAW_ATTEMPT_AS_RECOVERY_IDENT,
        ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT as NATIVE_ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT,
        ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT as NATIVE_ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
        ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT as NATIVE_ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT,
        ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT as NATIVE_ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT,
        ACCESS_CONTROLLER_TIMED_CONFIRM_RECOVERY_IDENT as NATIVE_ACCESS_CONTROLLER_TIMED_CONFIRM_RECOVERY_IDENT,
        ACCESS_CONTROLLER_CANCEL_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT as NATIVE_ACCESS_CONTROLLER_CANCEL_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT,
        ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT as NATIVE_ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
        ACCESS_CONTROLLER_CANCEL_PRIMARY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT as NATIVE_ACCESS_CONTROLLER_CANCEL_PRIMARY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT,
        ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT as NATIVE_ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT,
        ACCESS_CONTROLLER_LOCK_PRIMARY_ROLE_IDENT as NATIVE_ACCESS_CONTROLLER_LOCK_PRIMARY_ROLE_IDENT,
        ACCESS_CONTROLLER_UNLOCK_PRIMARY_ROLE_IDENT as NATIVE_ACCESS_CONTROLLER_UNLOCK_PRIMARY_ROLE_IDENT,
        ACCESS_CONTROLLER_STOP_TIMED_RECOVERY_IDENT as NATIVE_ACCESS_CONTROLLER_STOP_TIMED_RECOVERY_IDENT,
        ACCESS_CONTROLLER_MINT_RECOVERY_BADGES_IDENT as NATIVE_ACCESS_CONTROLLER_MINT_RECOVERY_BADGES_IDENT,
    };
    pub use radix_engine_interface::blueprints::account::{
        ResourcePreference as NativeResourcePreference,
        DefaultDepositRule as NativeDefaultDepositRule,
        AccountCreateAdvancedInput as NativeAccountCreateAdvancedInput,
        AccountCreateAdvancedManifestInput as NativeAccountCreateAdvancedManifestInput,
        AccountCreateInput as NativeAccountCreateInput,
        AccountSecurifyInput as NativeAccountSecurifyInput,
        AccountLockFeeInput as NativeAccountLockFeeInput,
        AccountLockContingentFeeInput as NativeAccountLockContingentFeeInput,
        AccountDepositInput as NativeAccountDepositInput,
        AccountDepositManifestInput as NativeAccountDepositManifestInput,
        AccountDepositBatchInput as NativeAccountDepositBatchInput,
        AccountDepositBatchManifestInput as NativeAccountDepositBatchManifestInput,
        AccountWithdrawInput as NativeAccountWithdrawInput,
        AccountWithdrawNonFungiblesInput as NativeAccountWithdrawNonFungiblesInput,
        AccountLockFeeAndWithdrawInput as NativeAccountLockFeeAndWithdrawInput,
        AccountLockFeeAndWithdrawNonFungiblesInput as NativeAccountLockFeeAndWithdrawNonFungiblesInput,
        AccountCreateProofOfAmountInput as NativeAccountCreateProofOfAmountInput,
        AccountCreateProofOfNonFungiblesInput as NativeAccountCreateProofOfNonFungiblesInput,
        AccountSetDefaultDepositRuleInput as NativeAccountSetDefaultDepositRuleInput,
        AccountSetResourcePreferenceInput as NativeAccountSetResourcePreferenceInput,
        AccountRemoveResourcePreferenceInput as NativeAccountRemoveResourcePreferenceInput,
        AccountTryDepositOrRefundInput as NativeAccountTryDepositOrRefundInput,
        AccountTryDepositOrRefundManifestInput as NativeAccountTryDepositOrRefundManifestInput,
        AccountTryDepositBatchOrRefundInput as NativeAccountTryDepositBatchOrRefundInput,
        AccountTryDepositBatchOrRefundManifestInput as NativeAccountTryDepositBatchOrRefundManifestInput,
        AccountTryDepositOrAbortInput as NativeAccountTryDepositOrAbortInput,
        AccountTryDepositOrAbortManifestInput as NativeAccountTryDepositOrAbortManifestInput,
        AccountTryDepositBatchOrAbortInput as NativeAccountTryDepositBatchOrAbortInput,
        AccountTryDepositBatchOrAbortManifestInput as NativeAccountTryDepositBatchOrAbortManifestInput,
        AccountBurnInput as NativeAccountBurnInput,
        AccountBurnNonFungiblesInput as NativeAccountBurnNonFungiblesInput,
        AccountAddAuthorizedDepositorInput as NativeAccountAddAuthorizedDepositorInput,
        AccountRemoveAuthorizedDepositorInput as NativeAccountRemoveAuthorizedDepositorInput,
        ACCOUNT_BLUEPRINT as NATIVE_ACCOUNT_BLUEPRINT,
        ACCOUNT_CREATE_ADVANCED_IDENT as NATIVE_ACCOUNT_CREATE_ADVANCED_IDENT,
        ACCOUNT_CREATE_IDENT as NATIVE_ACCOUNT_CREATE_IDENT,
        ACCOUNT_SECURIFY_IDENT as NATIVE_ACCOUNT_SECURIFY_IDENT,
        ACCOUNT_LOCK_FEE_IDENT as NATIVE_ACCOUNT_LOCK_FEE_IDENT,
        ACCOUNT_LOCK_CONTINGENT_FEE_IDENT as NATIVE_ACCOUNT_LOCK_CONTINGENT_FEE_IDENT,
        ACCOUNT_DEPOSIT_IDENT as NATIVE_ACCOUNT_DEPOSIT_IDENT,
        ACCOUNT_DEPOSIT_BATCH_IDENT as NATIVE_ACCOUNT_DEPOSIT_BATCH_IDENT,
        ACCOUNT_WITHDRAW_IDENT as NATIVE_ACCOUNT_WITHDRAW_IDENT,
        ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT as NATIVE_ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT,
        ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT as NATIVE_ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
        ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT as NATIVE_ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
        ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT as NATIVE_ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT,
        ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT as NATIVE_ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT,
        ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT as NATIVE_ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
        ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT as NATIVE_ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
        ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT as NATIVE_ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT,
        ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT as NATIVE_ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT,
        ACCOUNT_TRY_DEPOSIT_BATCH_OR_REFUND_IDENT as NATIVE_ACCOUNT_TRY_DEPOSIT_BATCH_OR_REFUND_IDENT,
        ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT as NATIVE_ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
        ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT as NATIVE_ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT,
        ACCOUNT_BURN_IDENT as NATIVE_ACCOUNT_BURN_IDENT,
        ACCOUNT_BURN_NON_FUNGIBLES_IDENT as NATIVE_ACCOUNT_BURN_NON_FUNGIBLES_IDENT,
        ACCOUNT_ADD_AUTHORIZED_DEPOSITOR as NATIVE_ACCOUNT_ADD_AUTHORIZED_DEPOSITOR,
        ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR as NATIVE_ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR,
    };
    pub use radix_engine_interface::blueprints::identity::{
        IDENTITY_BLUEPRINT as NATIVE_IDENTITY_BLUEPRINT,
        IDENTITY_CREATE_ADVANCED_IDENT as NATIVE_IDENTITY_CREATE_ADVANCED_IDENT,
        IDENTITY_CREATE_IDENT as NATIVE_IDENTITY_CREATE_IDENT,
        IDENTITY_SECURIFY_IDENT as NATIVE_IDENTITY_SECURIFY_IDENT,
        IdentityCreateAdvancedInput as NativeIdentityCreateAdvancedInput,
        IdentityCreateInput as NativeIdentityCreateInput,
        IdentitySecurifyToSingleBadgeInput as NativeIdentitySecurifyToSingleBadgeInput,
    };
    pub use radix_engine_interface::blueprints::consensus_manager::{
        VALIDATOR_BLUEPRINT as NATIVE_VALIDATOR_BLUEPRINT,
        VALIDATOR_REGISTER_IDENT as NATIVE_VALIDATOR_REGISTER_IDENT,
        VALIDATOR_UNREGISTER_IDENT as NATIVE_VALIDATOR_UNREGISTER_IDENT,
        VALIDATOR_STAKE_AS_OWNER_IDENT as NATIVE_VALIDATOR_STAKE_AS_OWNER_IDENT,
        VALIDATOR_STAKE_IDENT as NATIVE_VALIDATOR_STAKE_IDENT,
        VALIDATOR_UNSTAKE_IDENT as NATIVE_VALIDATOR_UNSTAKE_IDENT,
        VALIDATOR_CLAIM_XRD_IDENT as NATIVE_VALIDATOR_CLAIM_XRD_IDENT,
        VALIDATOR_UPDATE_KEY_IDENT as NATIVE_VALIDATOR_UPDATE_KEY_IDENT,
        VALIDATOR_UPDATE_FEE_IDENT as NATIVE_VALIDATOR_UPDATE_FEE_IDENT,
        VALIDATOR_UPDATE_ACCEPT_DELEGATED_STAKE_IDENT as NATIVE_VALIDATOR_UPDATE_ACCEPT_DELEGATED_STAKE_IDENT,
        VALIDATOR_ACCEPTS_DELEGATED_STAKE_IDENT as NATIVE_VALIDATOR_ACCEPTS_DELEGATED_STAKE_IDENT,
        VALIDATOR_TOTAL_STAKE_XRD_AMOUNT_IDENT as NATIVE_VALIDATOR_TOTAL_STAKE_XRD_AMOUNT_IDENT,
        VALIDATOR_TOTAL_STAKE_UNIT_SUPPLY_IDENT as NATIVE_VALIDATOR_TOTAL_STAKE_UNIT_SUPPLY_IDENT,
        VALIDATOR_GET_REDEMPTION_VALUE_IDENT as NATIVE_VALIDATOR_GET_REDEMPTION_VALUE_IDENT,
        VALIDATOR_SIGNAL_PROTOCOL_UPDATE_READINESS as NATIVE_VALIDATOR_SIGNAL_PROTOCOL_UPDATE_READINESS,
        VALIDATOR_GET_PROTOCOL_UPDATE_READINESS_IDENT as NATIVE_VALIDATOR_GET_PROTOCOL_UPDATE_READINESS_IDENT,
        VALIDATOR_APPLY_EMISSION_IDENT as NATIVE_VALIDATOR_APPLY_EMISSION_IDENT,
        VALIDATOR_APPLY_REWARD_IDENT as NATIVE_VALIDATOR_APPLY_REWARD_IDENT,
        VALIDATOR_LOCK_OWNER_STAKE_UNITS_IDENT as NATIVE_VALIDATOR_LOCK_OWNER_STAKE_UNITS_IDENT,
        VALIDATOR_START_UNLOCK_OWNER_STAKE_UNITS_IDENT as NATIVE_VALIDATOR_START_UNLOCK_OWNER_STAKE_UNITS_IDENT,
        VALIDATOR_FINISH_UNLOCK_OWNER_STAKE_UNITS_IDENT as NATIVE_VALIDATOR_FINISH_UNLOCK_OWNER_STAKE_UNITS_IDENT,
        ValidatorRegisterInput as NativeValidatorRegisterInput,
        ValidatorUnregisterInput as NativeValidatorUnregisterInput,
        ValidatorStakeAsOwnerInput as NativeValidatorStakeAsOwnerInput,
        ValidatorStakeInput as NativeValidatorStakeInput,
        ValidatorUnstakeInput as NativeValidatorUnstakeInput,
        ValidatorClaimXrdInput as NativeValidatorClaimXrdInput,
        ValidatorUpdateKeyInput as NativeValidatorUpdateKeyInput,
        ValidatorUpdateFeeInput as NativeValidatorUpdateFeeInput,
        ValidatorUpdateAcceptDelegatedStakeInput as NativeValidatorUpdateAcceptDelegatedStakeInput,
        ValidatorAcceptsDelegatedStakeInput as NativeValidatorAcceptsDelegatedStakeInput,
        ValidatorTotalStakeXrdAmountInput as NativeValidatorTotalStakeXrdAmountInput,
        ValidatorTotalStakeUnitSupplyInput as NativeValidatorTotalStakeUnitSupplyInput,
        ValidatorGetRedemptionValueInput as NativeValidatorGetRedemptionValueInput,
        ValidatorSignalProtocolUpdateReadinessInput as NativeValidatorSignalProtocolUpdateReadinessInput,
        ValidatorGetProtocolUpdateReadinessInput as NativeValidatorGetProtocolUpdateReadinessInput,
        ValidatorApplyEmissionInput as NativeValidatorApplyEmissionInput,
        ValidatorApplyRewardInput as NativeValidatorApplyRewardInput,
        ValidatorLockOwnerStakeUnitsInput as NativeValidatorLockOwnerStakeUnitsInput,
        ValidatorStartUnlockOwnerStakeUnitsInput as NativeValidatorStartUnlockOwnerStakeUnitsInput,
        ValidatorFinishUnlockOwnerStakeUnitsInput as NativeValidatorFinishUnlockOwnerStakeUnitsInput,
        ValidatorStakeAsOwnerManifestInput as NativeValidatorStakeAsOwnerManifestInput,
        ValidatorStakeManifestInput as NativeValidatorStakeManifestInput,
        ValidatorUnstakeManifestInput as NativeValidatorUnstakeManifestInput,
        ValidatorClaimXrdManifestInput as NativeValidatorClaimXrdManifestInput,
        ValidatorLockOwnerStakeUnitsManifestInput as NativeValidatorLockOwnerStakeUnitsManifestInput,
    };
    pub use radix_engine_interface::blueprints::package::{
        PackagePublishWasmInput as NativePackagePublishWasmInput,
        PackagePublishWasmManifestInput as NativePackagePublishWasmManifestInput,
        PackagePublishWasmAdvancedInput as NativePackagePublishWasmAdvancedInput,
        PackagePublishWasmAdvancedManifestInput as NativePackagePublishWasmAdvancedManifestInput,
        PackagePublishNativeInput as NativePackagePublishNativeInput,
        PackagePublishNativeManifestInput as NativePackagePublishNativeManifestInput,
        PackageClaimRoyaltiesInput as NativePackageClaimRoyaltiesInput,
        PackageDefinition as NativePackageDefinition,
        BlueprintDefinitionInit as NativeBlueprintDefinitionInit,
        AuthConfig as NativeAuthConfig,
        StaticRoleDefinition as NativeStaticRoleDefinition,
        PACKAGE_BLUEPRINT as NATIVE_PACKAGE_BLUEPRINT,
        PACKAGE_PUBLISH_WASM_IDENT as NATIVE_PACKAGE_PUBLISH_WASM_IDENT,
        PACKAGE_PUBLISH_WASM_ADVANCED_IDENT as NATIVE_PACKAGE_PUBLISH_WASM_ADVANCED_IDENT,
        PACKAGE_PUBLISH_NATIVE_IDENT as NATIVE_PACKAGE_PUBLISH_NATIVE_IDENT,
        PACKAGE_CLAIM_ROYALTIES_IDENT as NATIVE_PACKAGE_CLAIM_ROYALTIES_IDENT,
    };
    pub use radix_engine_interface::blueprints::resource::{
        MINTER_ROLE as NATIVE_MINTER_ROLE,
        MINTER_UPDATER_ROLE as NATIVE_MINTER_UPDATER_ROLE,
        BURNER_ROLE as NATIVE_BURNER_ROLE,
        BURNER_UPDATER_ROLE as NATIVE_BURNER_UPDATER_ROLE,
        WITHDRAWER_ROLE as NATIVE_WITHDRAWER_ROLE,
        WITHDRAWER_UPDATER_ROLE as NATIVE_WITHDRAWER_UPDATER_ROLE,
        DEPOSITOR_ROLE as NATIVE_DEPOSITOR_ROLE,
        DEPOSITOR_UPDATER_ROLE as NATIVE_DEPOSITOR_UPDATER_ROLE,
        RECALLER_ROLE as NATIVE_RECALLER_ROLE,
        RECALLER_UPDATER_ROLE as NATIVE_RECALLER_UPDATER_ROLE,
        FREEZER_ROLE as NATIVE_FREEZER_ROLE,
        FREEZER_UPDATER_ROLE as NATIVE_FREEZER_UPDATER_ROLE,
        NON_FUNGIBLE_DATA_UPDATER_ROLE as NATIVE_NON_FUNGIBLE_DATA_UPDATER_ROLE,
        NON_FUNGIBLE_DATA_UPDATER_UPDATER_ROLE as NATIVE_NON_FUNGIBLE_DATA_UPDATER_UPDATER_ROLE,
        RESOURCE_MANAGER_BURN_IDENT as NATIVE_RESOURCE_MANAGER_BURN_IDENT,
        RESOURCE_MANAGER_PACKAGE_BURN_IDENT as NATIVE_RESOURCE_MANAGER_PACKAGE_BURN_IDENT,
        RESOURCE_MANAGER_CREATE_EMPTY_VAULT_IDENT as NATIVE_RESOURCE_MANAGER_CREATE_EMPTY_VAULT_IDENT,
        RESOURCE_MANAGER_CREATE_EMPTY_BUCKET_IDENT as NATIVE_RESOURCE_MANAGER_CREATE_EMPTY_BUCKET_IDENT,
        RESOURCE_MANAGER_DROP_EMPTY_BUCKET_IDENT as NATIVE_RESOURCE_MANAGER_DROP_EMPTY_BUCKET_IDENT,
        RESOURCE_MANAGER_GET_RESOURCE_TYPE_IDENT as NATIVE_RESOURCE_MANAGER_GET_RESOURCE_TYPE_IDENT,
        RESOURCE_MANAGER_GET_TOTAL_SUPPLY_IDENT as NATIVE_RESOURCE_MANAGER_GET_TOTAL_SUPPLY_IDENT,
        RESOURCE_MANAGER_GET_AMOUNT_FOR_WITHDRAWAL_IDENT as NATIVE_RESOURCE_MANAGER_GET_AMOUNT_FOR_WITHDRAWAL_IDENT,
        FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT as NATIVE_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT,
        FUNGIBLE_RESOURCE_MANAGER_CREATE_IDENT as NATIVE_FUNGIBLE_RESOURCE_MANAGER_CREATE_IDENT,
        FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT as NATIVE_FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT,
        FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT as NATIVE_FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT,
        ResourceManagerBurnInput as NativeResourceManagerBurnInput,
        ResourceManagerPackageBurnInput as NativeResourceManagerPackageBurnInput,
        ResourceManagerCreateEmptyVaultInput as NativeResourceManagerCreateEmptyVaultInput,
        ResourceManagerCreateEmptyBucketInput as NativeResourceManagerCreateEmptyBucketInput,
        ResourceManagerDropEmptyBucketInput as NativeResourceManagerDropEmptyBucketInput,
        ResourceManagerGetResourceTypeInput as NativeResourceManagerGetResourceTypeInput,
        ResourceManagerGetTotalSupplyInput as NativeResourceManagerGetTotalSupplyInput,
        ResourceManagerGetAmountForWithdrawalInput as NativeResourceManagerGetAmountForWithdrawalInput,
        ResourceFeature as NativeResourceFeature,
        FungibleResourceRoles as NativeFungibleResourceRoles,
        FungibleResourceManagerCreateInput as NativeFungibleResourceManagerCreateInput,
        FungibleResourceManagerCreateManifestInput as NativeFungibleResourceManagerCreateManifestInput,
        FungibleResourceManagerCreateWithInitialSupplyInput as NativeFungibleResourceManagerCreateWithInitialSupplyInput,
        FungibleResourceManagerCreateWithInitialSupplyManifestInput as NativeFungibleResourceManagerCreateWithInitialSupplyManifestInput,
        FungibleResourceManagerMintInput as NativeFungibleResourceManagerMintInput,
    };
    pub use radix_engine_interface::blueprints::pool::{
        ONE_RESOURCE_POOL_INSTANTIATE_IDENT as NATIVE_ONE_RESOURCE_POOL_INSTANTIATE_IDENT,
        ONE_RESOURCE_POOL_CONTRIBUTE_IDENT as NATIVE_ONE_RESOURCE_POOL_CONTRIBUTE_IDENT,
        ONE_RESOURCE_POOL_REDEEM_IDENT as NATIVE_ONE_RESOURCE_POOL_REDEEM_IDENT,
        ONE_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT as NATIVE_ONE_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT,
        ONE_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT as NATIVE_ONE_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT,
        ONE_RESOURCE_POOL_GET_REDEMPTION_VALUE_IDENT as NATIVE_ONE_RESOURCE_POOL_GET_REDEMPTION_VALUE_IDENT,
        ONE_RESOURCE_POOL_GET_VAULT_AMOUNT_IDENT as NATIVE_ONE_RESOURCE_POOL_GET_VAULT_AMOUNT_IDENT,
        OneResourcePoolInstantiateManifestInput as NativeOneResourcePoolInstantiateManifestInput,
        OneResourcePoolContributeManifestInput as NativeOneResourcePoolContributeManifestInput,
        OneResourcePoolRedeemManifestInput as NativeOneResourcePoolRedeemManifestInput,
        OneResourcePoolProtectedDepositManifestInput as NativeOneResourcePoolProtectedDepositManifestInput,
        OneResourcePoolProtectedWithdrawManifestInput as NativeOneResourcePoolProtectedWithdrawManifestInput,
        OneResourcePoolGetRedemptionValueManifestInput as NativeOneResourcePoolGetRedemptionValueManifestInput,
        OneResourcePoolGetVaultAmountManifestInput as NativeOneResourcePoolGetVaultAmountManifestInput,

        TWO_RESOURCE_POOL_INSTANTIATE_IDENT as NATIVE_TWO_RESOURCE_POOL_INSTANTIATE_IDENT,
        TWO_RESOURCE_POOL_CONTRIBUTE_IDENT as NATIVE_TWO_RESOURCE_POOL_CONTRIBUTE_IDENT,
        TWO_RESOURCE_POOL_REDEEM_IDENT as NATIVE_TWO_RESOURCE_POOL_REDEEM_IDENT,
        TWO_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT as NATIVE_TWO_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT,
        TWO_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT as NATIVE_TWO_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT,
        TWO_RESOURCE_POOL_GET_REDEMPTION_VALUE_IDENT as NATIVE_TWO_RESOURCE_POOL_GET_REDEMPTION_VALUE_IDENT,
        TWO_RESOURCE_POOL_GET_VAULT_AMOUNTS_IDENT as NATIVE_TWO_RESOURCE_POOL_GET_VAULT_AMOUNTS_IDENT,
        TwoResourcePoolInstantiateManifestInput as NativeTwoResourcePoolInstantiateManifestInput,
        TwoResourcePoolContributeManifestInput as NativeTwoResourcePoolContributeManifestInput,
        TwoResourcePoolRedeemManifestInput as NativeTwoResourcePoolRedeemManifestInput,
        TwoResourcePoolProtectedDepositManifestInput as NativeTwoResourcePoolProtectedDepositManifestInput,
        TwoResourcePoolProtectedWithdrawManifestInput as NativeTwoResourcePoolProtectedWithdrawManifestInput,
        TwoResourcePoolGetRedemptionValueManifestInput as NativeTwoResourcePoolGetRedemptionValueManifestInput,
        TwoResourcePoolGetVaultAmountsManifestInput as NativeTwoResourcePoolGetVaultAmountsManifestInput,

        MULTI_RESOURCE_POOL_INSTANTIATE_IDENT as NATIVE_MULTI_RESOURCE_POOL_INSTANTIATE_IDENT,
        MULTI_RESOURCE_POOL_CONTRIBUTE_IDENT as NATIVE_MULTI_RESOURCE_POOL_CONTRIBUTE_IDENT,
        MULTI_RESOURCE_POOL_REDEEM_IDENT as NATIVE_MULTI_RESOURCE_POOL_REDEEM_IDENT,
        MULTI_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT as NATIVE_MULTI_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT,
        MULTI_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT as NATIVE_MULTI_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT,
        MULTI_RESOURCE_POOL_GET_REDEMPTION_VALUE_IDENT as NATIVE_MULTI_RESOURCE_POOL_GET_REDEMPTION_VALUE_IDENT,
        MULTI_RESOURCE_POOL_GET_VAULT_AMOUNTS_IDENT as NATIVE_MULTI_RESOURCE_POOL_GET_VAULT_AMOUNTS_IDENT,
        MultiResourcePoolInstantiateManifestInput as NativeMultiResourcePoolInstantiateManifestInput,
        MultiResourcePoolContributeManifestInput as NativeMultiResourcePoolContributeManifestInput,
        MultiResourcePoolRedeemManifestInput as NativeMultiResourcePoolRedeemManifestInput,
        MultiResourcePoolProtectedDepositManifestInput as NativeMultiResourcePoolProtectedDepositManifestInput,
        MultiResourcePoolProtectedWithdrawManifestInput as NativeMultiResourcePoolProtectedWithdrawManifestInput,
        MultiResourcePoolGetRedemptionValueManifestInput as NativeMultiResourcePoolGetRedemptionValueManifestInput,
        MultiResourcePoolGetVaultAmountsManifestInput as NativeMultiResourcePoolGetVaultAmountsManifestInput,
    };
    pub use radix_engine_interface::prelude::{
        EventTypeIdentifier as NativeEventTypeIdentifier,
        Emitter as NativeEmitter,
    };
    pub use radix_engine_interface::api::{
        ObjectModuleId as NativeObjectModuleId
    };
}

pub use self::core::*;
pub use self::native::*;
