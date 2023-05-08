use native_transaction::{builder::ManifestBuilder, ecdsa_secp256k1::EcdsaSecp256k1PrivateKey};
use radix_engine_toolkit::{
    model::{
        address::Bech32Coder,
        transaction::{InstructionKind, TransactionManifest},
    },
    request::{AnalyzeManifestHandler, AnalyzeManifestRequest, Handler},
};
use scrypto::{
    api::node_modules::metadata::{MetadataEntry, MetadataValue},
    prelude::*,
};

#[test]
pub fn identities_needed_to_sign_appear_in_analyze_manifest() {
    // Arrange
    let private_key = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap();
    let identity = ComponentAddress::virtual_identity_from_public_key(&private_key.public_key());
    let manifest = ManifestBuilder::new()
        .set_metadata(
            identity.into(),
            "name".into(),
            MetadataEntry::Value(MetadataValue::String("my_name".into())),
        )
        .build();

    // Act
    let addresses = {
        let input = AnalyzeManifestRequest {
            manifest: TransactionManifest::from_native_manifest(
                &manifest,
                InstructionKind::Parsed,
                &Bech32Coder::new(0xf2),
            )
            .unwrap(),
            network_id: 0xf2,
        };
        let output = AnalyzeManifestHandler::fulfill(input);
        output.unwrap()
    };

    // Assert
    assert_eq!(addresses.identities_requiring_auth.len(), 1)
}
