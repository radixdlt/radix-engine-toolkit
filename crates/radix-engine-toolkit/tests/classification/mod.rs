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

mod account_deposit_settings;
mod general;
mod general_subintent;
mod pool_contribution;
mod pool_redemption;
mod transfer;
mod validator_claim;
mod validator_stake;
mod validator_unstake;

#[test]
fn static_classification_returns_correct_ordering_of_classifications() {
    use radix_common::prelude::*;
    use radix_engine_toolkit::prelude::*;
    use radix_transactions::manifest::*;

    // Arrange
    let manifest_str = r#"
    CALL_METHOD
        Address("account_loc1cxqegq32y82vx7wd0du4zswm7303t7uzhywyc7x4wz6pskel9sz2rd")
        "lock_fee"
        Decimal("25")
    ;
    CALL_METHOD
        Address("account_loc1cxqegq32y82vx7wd0du4zswm7303t7uzhywyc7x4wz6pskel9sz2rd")
        "withdraw"
        Address("resource_loc1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxvq32hv")
        Decimal("10")
    ;
    TAKE_FROM_WORKTOP
        Address("resource_loc1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxvq32hv")
        Decimal("10")
        Bucket("bucket1")
    ;
    CALL_METHOD
        Address("validator_loc1s0a9c9kwjr3dmw79djvhalyz32sywumacv873yr7ms02v725cjwplj")
        "stake"
        Bucket("bucket1")
    ;
    CALL_METHOD
        Address("account_loc1cxqegq32y82vx7wd0du4zswm7303t7uzhywyc7x4wz6pskel9sz2rd")
        "deposit_batch"
        Expression("ENTIRE_WORKTOP")
    ;
    "#;
    let manifest = compile(
        manifest_str,
        &NetworkDefinition::localnet(),
        MockBlobProvider,
    )
    .unwrap();

    // Act
    let manifest_classification =
        radix_engine_toolkit::prelude::statically_analyze(&manifest);

    // Assert
    let manifest_classification =
        manifest_classification.expect("Should not fail");
    assert_eq!(
        manifest_classification.manifest_classification,
        [
            ManifestClassification::ValidatorStake,
            ManifestClassification::General
        ]
    )
}
