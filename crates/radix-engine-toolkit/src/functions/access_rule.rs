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

use scrypto::prelude::*;

pub fn extract_entities(
    access_rule: &AccessRule,
) -> IndexSet<ResourceOrNonFungible> {
    #[derive(Default)]
    pub struct AccessRuleEntitiesVisitor(IndexSet<ResourceOrNonFungible>);

    impl AccessRuleEntitiesVisitor {
        pub fn into_output(self) -> IndexSet<ResourceOrNonFungible> {
            self.0
        }
    }

    impl AccessRuleVisitor for AccessRuleEntitiesVisitor {
        type Error = ();

        fn visit(
            &mut self,
            node: &CompositeRequirement,
            _: usize,
        ) -> Result<(), Self::Error> {
            match node {
                CompositeRequirement::BasicRequirement(basic) => match basic {
                    BasicRequirement::Require(requirement) => {
                        self.0.insert(requirement.clone());
                    }
                    BasicRequirement::AmountOf(_, resource_address) => {
                        self.0.insert(ResourceOrNonFungible::Resource(
                            *resource_address,
                        ));
                    }
                    BasicRequirement::CountOf(_, requirements)
                    | BasicRequirement::AllOf(requirements)
                    | BasicRequirement::AnyOf(requirements) => {
                        self.0.extend(requirements.clone());
                    }
                },
                CompositeRequirement::AnyOf(_)
                | CompositeRequirement::AllOf(_) => {}
            }

            Ok(())
        }
    }

    let mut visitor = AccessRuleEntitiesVisitor::default();
    access_rule
        .dfs_traverse_nodes(&mut visitor)
        .expect("Visitor will not error");
    visitor.into_output()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn addresses_can_be_found_in_access_rules() {
        // Arrange
        let required_resource = XRD;
        let required_non_fungible =
            NonFungibleGlobalId::from_public_key(&Secp256k1PublicKey([1; 33]));
        let rule = rule!(
            require(required_resource)
                && require(required_non_fungible.clone())
        );

        // Act
        let entities = extract_entities(&rule);

        // Assert
        assert_eq!(entities.len(), 2);
        assert!(entities
            .contains(&ResourceOrNonFungible::Resource(required_resource)));
        assert!(entities.contains(&ResourceOrNonFungible::NonFungible(
            required_non_fungible
        )));
    }
}
