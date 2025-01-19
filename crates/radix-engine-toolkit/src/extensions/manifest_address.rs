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
