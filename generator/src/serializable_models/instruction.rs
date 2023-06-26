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

use super::traits::HasExamples;
use crate::function_examples::notarized_transaction::*;
use radix_engine::system::bootstrap::*;
use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::consensus_manager::*;
use radix_engine_interface::blueprints::package::*;
use radix_engine_toolkit::prelude::*;
use scrypto::api::node_modules::auth::*;
use scrypto::api::node_modules::metadata::*;
use scrypto::api::node_modules::royalty::*;
use scrypto::prelude::*;
use transaction::prelude::*;
use transaction::validation::*;

impl<'f> HasExamples<'f> for SerializableInstruction {
    fn examples() -> Vec<Self> {
        let (code, setup) = example_package();

        ExamplesBuilder::new()
            .add_example(|mut builder| {
                builder
                    .call_function(
                        FAUCET_PACKAGE,
                        "blueprint",
                        "function_name",
                        manifest_args!(dec!("10"), "hello world!", RADIX_TOKEN),
                    )
                    .call_function(
                        FAUCET_PACKAGE,
                        "blueprint",
                        "function_name",
                        manifest_args!(),
                    )
                    .call_method(FAUCET, "free", manifest_args!(dec!("100")))
                    .call_method(FAUCET, "free", manifest_args!())
                    .take_from_worktop(RADIX_TOKEN, 10.into(), |builder, _| builder)
                    .take_all_from_worktop(RADIX_TOKEN, |builder, bucket| {
                        builder
                            .create_proof_from_bucket(&bucket, |builder, _| builder)
                            .create_proof_from_bucket_of_all(&bucket, |builder, _| builder)
                            .create_proof_from_bucket_of_amount(&bucket, 10.into(), |builder, _| {
                                builder
                            })
                            .create_proof_from_bucket_of_non_fungibles(
                                &bucket,
                                BTreeSet::from([
                                    NonFungibleLocalId::integer(1),
                                    NonFungibleLocalId::integer(2),
                                ]),
                                |builder, _| builder,
                            )
                    })
                    .take_non_fungibles_from_worktop(
                        RADIX_TOKEN,
                        &BTreeSet::from([
                            NonFungibleLocalId::integer(1),
                            NonFungibleLocalId::integer(2),
                        ]),
                        |builder, bucket| builder.return_to_worktop(bucket),
                    )
                    .assert_worktop_contains(RADIX_TOKEN, 10.into())
                    .assert_worktop_contains_non_fungibles(
                        RADIX_TOKEN,
                        &BTreeSet::from([
                            NonFungibleLocalId::integer(1),
                            NonFungibleLocalId::integer(2),
                        ]),
                    )
                    .pop_from_auth_zone(|builder, proof| {
                        builder
                            .push_to_auth_zone(proof)
                            .clone_proof(&proof, |builder, proof2| builder.drop_proof(proof2))
                    })
                    .clear_auth_zone()
                    .create_proof_from_auth_zone(RADIX_TOKEN, |builder, _| builder)
                    .create_proof_from_auth_zone_of_all(RADIX_TOKEN, |builder, _| builder)
                    .create_proof_from_auth_zone_of_amount(RADIX_TOKEN, 10.into(), |builder, _| {
                        builder
                    })
                    .create_proof_from_auth_zone_of_non_fungibles(
                        RADIX_TOKEN,
                        &BTreeSet::from([
                            NonFungibleLocalId::integer(1),
                            NonFungibleLocalId::integer(2),
                        ]),
                        |builder, _| builder,
                    )
                    .clear_signature_proofs()
                    .burn_from_worktop(10.into(), RADIX_TOKEN)
                    .add_instruction(InstructionV1::CallRoyaltyMethod {
                        address: DynamicGlobalAddress::Static(FAUCET.into()),
                        method_name: COMPONENT_ROYALTY_CLAIM_ROYALTIES_IDENT.to_string(),
                        args: to_manifest_value(&ComponentClaimRoyaltiesInput {}).unwrap(),
                    })
                    .0
                    .add_instruction(InstructionV1::CallMetadataMethod {
                        address: DynamicGlobalAddress::Static(FAUCET.into()),
                        method_name: METADATA_SET_IDENT.to_string(),
                        args: to_manifest_value(&MetadataSetInput {
                            key: "key".into(),
                            value: MetadataValue::Bool(true),
                        })
                        .unwrap(),
                    })
                    .0
                    .add_instruction(InstructionV1::CallAccessRulesMethod {
                        address: DynamicGlobalAddress::Static(FAUCET.into()),
                        method_name: ACCESS_RULES_UPDATE_ROLE_IDENT.to_string(),
                        args: to_manifest_value(&AccessRulesUpdateRoleInput {
                            module: scrypto::api::ObjectModuleId::Main,
                            role_key: RoleKey {
                                key: "free".to_string(),
                            },
                            rule: Some(rule!(allow_all)),
                            mutability: Some((
                                RoleList {
                                    list: vec![RoleKey {
                                        key: "_owner_".to_string(),
                                    }],
                                },
                                true,
                            )),
                        })
                        .unwrap(),
                    })
                    .0
                    .drop_all_proofs()
                    .recall(vault_id(), 10.into())
                    .freeze(vault_id())
                    .unfreeze(vault_id())
                    .create_identity()
                    .create_identity_advanced(OwnerRole::None)
                    .call_function(
                        ACCOUNT_PACKAGE,
                        ACCOUNT_BLUEPRINT,
                        ACCOUNT_CREATE_IDENT,
                        to_manifest_value(&AccountCreateInput {}).unwrap(),
                    )
                    .call_function(
                        ACCOUNT_PACKAGE,
                        ACCOUNT_BLUEPRINT,
                        ACCOUNT_CREATE_ADVANCED_IDENT,
                        to_manifest_value(&AccountCreateAdvancedInput {
                            owner_role: OwnerRole::None,
                        })
                        .unwrap(),
                    )
                    .create_validator(
                        Secp256k1PrivateKey::from_u64(1).unwrap().public_key(),
                        10.into(),
                    )
                    .mint_ruid_non_fungible(
                        RADIX_TOKEN,
                        vec![
                            Human {
                                name: "Jack".into(),
                                age: 30,
                                height: (6, 11, 2),
                            },
                            Human {
                                name: "Not Jack".into(),
                                age: 60,
                                height: (5, 11, 2),
                            },
                        ],
                    )
                    .mint_non_fungible(
                        RADIX_TOKEN,
                        btreemap! {
                            NonFungibleLocalId::integer(1) => Human {
                                name: "Jack".into(),
                                age: 30,
                                height: (6, 11, 2),
                            },
                            NonFungibleLocalId::integer(2) => Human {
                                name: "Not Jack".into(),
                                age: 60,
                                height: (5, 11, 2),
                            },
                        },
                    )
                    .mint_fungible(RADIX_TOKEN, 10.into())
                    .claim_package_royalty(ACCOUNT_PACKAGE)
                    .claim_component_royalties(FAUCET)
                    .set_component_royalty(FAUCET, "free", RoyaltyAmount::Free)
                    .add_instruction(InstructionV1::CallMetadataMethod {
                        address: DynamicGlobalAddress::Static(FAUCET.into()),
                        method_name: METADATA_REMOVE_IDENT.into(),
                        args: to_manifest_value(&MetadataRemoveInput {
                            key: "free".to_string(),
                        })
                        .unwrap(),
                    })
                    .0
                    .set_metadata(
                        FAUCET.into(),
                        "free".to_string(),
                        MetadataValue::Decimal(10.into()),
                    )
                    .create_access_controller(
                        ManifestBucket(10),
                        rule!(allow_all),
                        rule!(allow_all),
                        rule!(allow_all),
                        Some(10),
                    )
                    .create_fungible_resource::<AccessRule>(
                        true,
                        18,
                        BTreeMap::new(),
                        BTreeMap::new(),
                        Some(10.into()),
                    )
                    .create_fungible_resource::<AccessRule>(
                        true,
                        18,
                        BTreeMap::new(),
                        BTreeMap::new(),
                        None,
                    )
                    .create_non_fungible_resource(
                        NonFungibleIdType::Integer,
                        true,
                        Default::default(),
                        BTreeMap::<ResourceMethodAuthKey, (AccessRule, AccessRule)>::new(),
                        None::<BTreeMap<NonFungibleLocalId, Human>>,
                    )
                    .create_non_fungible_resource(
                        NonFungibleIdType::Integer,
                        true,
                        Default::default(),
                        BTreeMap::<ResourceMethodAuthKey, (AccessRule, AccessRule)>::new(),
                        Some(btreemap! {
                            NonFungibleLocalId::integer(1) => Human {
                                name: "Jack".into(),
                                age: 30,
                                height: (6, 11, 2),
                            },
                            NonFungibleLocalId::integer(2) => Human {
                                name: "Not Jack".into(),
                                age: 60,
                                height: (5, 11, 2),
                            },
                        }),
                    )
                    .allocate_global_address(
                        BlueprintId {
                            package_address: ACCOUNT_PACKAGE,
                            blueprint_name: ACCOUNT_BLUEPRINT.to_string(),
                        },
                        |builder, _, _| builder,
                    )
                    .publish_package(code.clone(), setup.clone())
                    .publish_package_advanced(
                        code.clone(),
                        setup.clone(),
                        Default::default(),
                        OwnerRole::None,
                    )
                    .build()
            })
            .add_instructions(
                notarized_transactions()
                    .into_iter()
                    .flat_map(|item| {
                        item.signed_intent
                            .intent
                            .manifest
                            .instructions
                            .to_instructions(0xf2)
                            .unwrap()
                    })
                    .collect::<Vec<_>>(),
            )
            .build()
    }
}

#[derive(Default)]
struct ExamplesBuilder(Vec<SerializableInstruction>);
impl ExamplesBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_example<F>(self, callback: F) -> Self
    where
        F: FnOnce(ManifestBuilder) -> TransactionManifestV1,
    {
        let TransactionManifestV1 { instructions, .. } = callback(ManifestBuilder::new());
        self.add_instructions(instructions)
    }

    pub fn add_instructions(self, instructions: Vec<InstructionV1>) -> Self {
        self.add_serializable_instructions(
            instructions
                .into_iter()
                .map(|instruction| {
                    SerializableInstruction::from_instruction(
                        &instruction,
                        0xf2,
                        &mut ManifestIdAllocator::new(),
                    )
                    .unwrap()
                })
                .collect(),
        )
    }

    pub fn add_serializable_instructions(
        mut self,
        instructions: Vec<SerializableInstruction>,
    ) -> Self {
        self.0.extend(instructions);
        self
    }

    pub fn build(self) -> Vec<SerializableInstruction> {
        self.0
            .into_iter()
            .collect::<sbor::prelude::IndexSet<_>>()
            .into_iter()
            .collect()
    }
}

fn vault_id() -> InternalAddress {
    let mut address = RADIX_TOKEN.as_node_id().0;
    address[0] = EntityType::InternalFungibleVault as u8;
    InternalAddress::new_or_panic(address)
}

fn example_package() -> (Vec<u8>, PackageDefinition) {
    let SystemTransactionV1 {
        instructions,
        blobs,
        ..
    } = create_system_bootstrap_transaction(
        Epoch::of(0),
        ConsensusManagerConfig {
            max_validators: 0,
            epoch_change_condition: EpochChangeCondition::default(),
            num_unstake_epochs: 0,
            total_emission_xrd_per_epoch: 0.into(),
            min_validator_reliability: 0.into(),
            num_owner_stake_units_unlock_epochs: 0,
            num_fee_increase_delay_epochs: 0,
        },
        0,
        None,
    );

    for instruction in instructions.0.into_iter() {
        let (code, setup) = match instruction {
            InstructionV1::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } if package_address == DynamicPackageAddress::Static(PACKAGE_PACKAGE)
                && blueprint_name == PACKAGE_BLUEPRINT
                && function_name == PACKAGE_PUBLISH_WASM_IDENT =>
            {
                let PackagePublishWasmManifestIndexMapInput { code, setup, .. } =
                    manifest_decode(&manifest_encode(&args).unwrap()).unwrap();

                (code, setup)
            }
            InstructionV1::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } if package_address == DynamicPackageAddress::Static(PACKAGE_PACKAGE)
                && blueprint_name == PACKAGE_BLUEPRINT
                && function_name == PACKAGE_PUBLISH_WASM_ADVANCED_IDENT =>
            {
                let PackagePublishWasmAdvancedManifestIndexMapInput { code, setup, .. } =
                    manifest_decode(&manifest_encode(&args).unwrap()).unwrap();

                (code, setup)
            }
            _ => continue,
        };

        let code = blobs
            .blobs
            .into_iter()
            .find_map(|BlobV1(value)| {
                if hash(&value).0 == code.0 {
                    Some(value)
                } else {
                    None
                }
            })
            .unwrap();
        return (code, setup);
    }
    panic!("Can't get here")
}

#[derive(NonFungibleData, ScryptoSbor, ManifestSbor)]
pub struct Human {
    name: String,
    age: u128,
    height: (u8, u8, u8),
}
