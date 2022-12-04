use radix_engine_toolkit_core::models::TransactionManifest;
use radix_engine_toolkit_core::requests::ConvertManifestRequest;
use radix_engine_toolkit_core::traits::Request;

#[test]
pub fn basic_manifest_conversion_succeeds() {
    // Arrange
    let test_vectors = vec![
        (
            include_str!("test_manifests/manifest1.rtm").to_string(),
            vec![
                include_bytes!("test_manifests/manifest1_code.blob").to_vec(),
                include_bytes!("test_manifests/manifest1_abi.blob").to_vec(),
            ],
        ),
        (
            include_str!("test_manifests/manifest2.rtm").to_string(),
            vec![],
        ),
        (
            include_str!("test_manifests/manifest3.rtm").to_string(),
            vec![],
        ),
        (
            include_str!("test_manifests/manifest4.rtm").to_string(),
            vec![include_bytes!("test_manifests/manifest4.blob").to_vec()],
        ),
    ];

    let requests = test_vectors
        .iter()
        .map(|x| ConvertManifestRequest {
            transaction_version: 0x01,
            network_id: 0xF2,
            manifest_instructions_output_format:
                radix_engine_toolkit_core::models::ManifestInstructionsKind::JSON,
            manifest: TransactionManifest {
                instructions: radix_engine_toolkit_core::models::ManifestInstructions::String(
                    x.0.clone(),
                ),
                blobs: x.1.clone(),
            },
        })
        .collect::<Vec<_>>();

    for request in requests {
        // Act
        let response = request.fulfill_request();

        // Assert
        assert!(response.is_ok());
    }
}
