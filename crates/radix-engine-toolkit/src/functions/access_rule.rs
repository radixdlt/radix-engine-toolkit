use crate::internal_prelude::*;

pub fn extract_addresses(
    access_rule: &AccessRule,
) -> (IndexSet<ResourceAddress>, IndexSet<NonFungibleGlobalId>) {
    #[derive(Clone, Debug, Default)]
    pub struct Visitor {
        pub resource_addresses: IndexSet<ResourceAddress>,
        pub non_fungible_global_ids: IndexSet<NonFungibleGlobalId>,
    }

    impl Visitor {
        pub fn add(&mut self, value: impl Into<ResourceOrNonFungible>) {
            match value.into() {
                ResourceOrNonFungible::Resource(resource_address) => {
                    self.resource_addresses.insert(resource_address);
                }
                ResourceOrNonFungible::NonFungible(non_fungible) => {
                    self.non_fungible_global_ids.insert(non_fungible);
                }
            }
        }
    }

    impl AccessRuleVisitor for Visitor {
        type Error = ();

        fn visit(
            &mut self,
            node: &CompositeRequirement,
            _: usize,
        ) -> Result<(), Self::Error> {
            match node {
                CompositeRequirement::BasicRequirement(
                    BasicRequirement::Require(address),
                ) => self.add(address.clone()),
                CompositeRequirement::BasicRequirement(
                    BasicRequirement::AmountOf(_, address),
                ) => self.add(*address),
                CompositeRequirement::BasicRequirement(
                    BasicRequirement::CountOf(_, addresses)
                    | BasicRequirement::AllOf(addresses)
                    | BasicRequirement::AnyOf(addresses),
                ) => {
                    addresses.iter().cloned().for_each(|value| self.add(value))
                }
                CompositeRequirement::AllOf(_)
                | CompositeRequirement::AnyOf(_) => {}
            }

            Ok(())
        }
    }

    let mut visitor = Visitor::default();
    access_rule
        .dfs_traverse_nodes(&mut visitor)
        .expect("Can't fail");
    (visitor.resource_addresses, visitor.non_fungible_global_ids)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn addresses_can_be_extracted_from_require_xrd() {
        // Arrange
        let rule = rule!(require(XRD));

        // Act
        let (resource_addresses, non_fungibles) = extract_addresses(&rule);

        // Assert
        assert_eq!(non_fungibles, indexset![]);
        assert_eq!(resource_addresses, indexset![XRD]);
    }

    #[test]
    fn addresses_can_be_extracted_from_require_signature() {
        // Arrange
        let public_key = Secp256k1PublicKey([1; 33]);
        let signature = NonFungibleGlobalId::from_public_key(&public_key);
        let rule = rule!(require(signature.clone()));

        // Act
        let (resource_addresses, non_fungibles) = extract_addresses(&rule);

        // Assert
        assert_eq!(non_fungibles, indexset![signature]);
        assert_eq!(resource_addresses, indexset![]);
    }
}
