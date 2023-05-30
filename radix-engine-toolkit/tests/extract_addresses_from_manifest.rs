use native_transaction::{builder::ManifestBuilder, ecdsa_secp256k1::EcdsaSecp256k1PrivateKey};
use radix_engine_toolkit::functions::*;
use radix_engine_toolkit::model::address::Bech32Coder;
use radix_engine_toolkit::model::transaction::{InstructionKind, TransactionManifest};
use scrypto::{api::node_modules::metadata::MetadataValue, prelude::*};

#[test]
pub fn identities_needed_to_sign_appear_when_extracting_addresses() {
    // Arrange
    let private_key = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap();
    let identity = ComponentAddress::virtual_identity_from_public_key(&private_key.public_key());
    let manifest = ManifestBuilder::new()
        .set_metadata(
            identity.into(),
            "name".into(),
            MetadataValue::String("my_name".into()),
        )
        .build();

    // Act
    let addresses = {
        let input = extract_addresses_from_manifest::Input {
            manifest: TransactionManifest::from_native_manifest(
                &manifest,
                InstructionKind::Parsed,
                &Bech32Coder::new(0xf2),
            )
            .unwrap(),
            network_id: 0xf2,
        };
        let output = extract_addresses_from_manifest::Handler::fulfill(input);
        output.unwrap()
    };

    // Assert
    assert_eq!(addresses.identities_requiring_auth.len(), 1)
}
