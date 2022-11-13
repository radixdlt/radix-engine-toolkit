use radix_engine_toolkit::{
    models::TransactionManifest, requests::ConvertManifestRequest, traits::Request,
};

#[test]
pub fn basic_manifest_conversion_succeeds() {
    // Arrange
    let test_vectors: Vec<(String, Vec<Vec<u8>>)> = vec![
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

    let requests: Vec<ConvertManifestRequest> = test_vectors
        .iter()
        .map(|x| ConvertManifestRequest {
            transaction_version: 0x01,
            network_id: 0xF2,
            manifest_instructions_output_format:
                radix_engine_toolkit::models::ManifestInstructionsKind::JSON,
            manifest: TransactionManifest {
                instructions: radix_engine_toolkit::models::ManifestInstructions::String(
                    x.0.clone(),
                ),
                blobs: x.1.clone(),
            },
        })
        .collect();

    // Act
    for request in requests {
        let response = request.fulfill_request();

        // Assert
        assert!(response.is_ok());
    }
}
