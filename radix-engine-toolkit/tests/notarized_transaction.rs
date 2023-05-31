use transaction::validation::ValidationConfig;
mod test_data;

#[test]
fn notarized_transaction_hash_can_be_obtained() {
    // Arrange
    let transaction = test_data::notarized_transaction();

    // Act
    let hash = radix_engine_toolkit::functions::notarized_transaction::hash(&transaction);

    // Assert
    assert!(hash.is_ok())
}

#[test]
fn notarized_transaction_can_be_compiled() {
    // Arrange
    let transaction = test_data::notarized_transaction();

    // Act
    let compiled = radix_engine_toolkit::functions::notarized_transaction::compile(&transaction);

    // Assert
    assert!(compiled.is_ok())
}

#[test]
fn notarized_transaction_can_be_compiled_and_later_decompiled() {
    // Arrange
    let transaction = test_data::notarized_transaction();
    let compiled =
        radix_engine_toolkit::functions::notarized_transaction::compile(&transaction).unwrap();

    // Act
    let decompiled = radix_engine_toolkit::functions::notarized_transaction::decompile(compiled);

    // Assert
    assert!(decompiled.is_ok());
    assert_eq!(decompiled, Ok(transaction))
}

#[test]
fn notarized_transaction_can_be_statically_validated() {
    // Arrange
    let transaction = test_data::notarized_transaction();
    let validation_config = ValidationConfig::default(0x01);

    // Act
    let validation_result =
        radix_engine_toolkit::functions::notarized_transaction::statically_validate(
            &transaction,
            validation_config,
        );

    // Assert
    assert!(validation_result.is_ok())
}
