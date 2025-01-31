use crate::internal_prelude::*;

macro_rules! impl_conversions {
    (
        $(
            ($scrypto_type: ty, $manifest_type: ty $(,)?)
        ),* $(,)?
    ) => {
        $(
            #[ext_sized]
            pub impl $manifest_type {
                fn as_static(&self) -> Option<&$scrypto_type> {
                    if let Self::Static(v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn into_static(self) -> Option<$scrypto_type> {
                    if let Self::Static(v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn resolve_entity_type(
                    &self,
                    named_address_store: &NamedAddressStore
                ) -> Option<EntityType> {
                    match self {
                        Self::Static(static_address) => {
                            let node_id = NodeId::from(*static_address);
                            node_id.entity_type()
                        },
                        Self::Named(named_address) => {
                            named_address_store
                                .get(&named_address)
                                .and_then(BlueprintId::entity_type)
                        }
                    }
                }

                fn resolve_grouped_entity_type(
                    &self,
                    named_address_store: &NamedAddressStore
                ) -> Option<GroupedEntityType> {
                    self.resolve_entity_type(named_address_store)
                        .map(Into::into)
                }
            }
        )*
    };
}
impl_conversions![
    (NodeId, ManifestAddress),
    (GlobalAddress, ManifestGlobalAddress),
    (ComponentAddress, ManifestComponentAddress),
    (ResourceAddress, ManifestResourceAddress),
    (PackageAddress, ManifestPackageAddress),
];
