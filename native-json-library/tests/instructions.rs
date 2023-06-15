#![allow(dead_code)]
#![allow(clippy::expect_fun_call)]

use native_json_library::models::instruction::{
    InstructionConversionError, SerializableInstruction,
};
use radix_engine_common::prelude::*;
use transaction::manifest::*;
use transaction::prelude::*;
use transaction::validation::*;
use walkdir::WalkDir;

#[test]
fn common_manifests_can_all_be_converted_to_serializable() {
    // Arrange
    let path = ".";
    for entry in WalkDir::new(path) {
        let path = entry.unwrap().path().canonicalize().unwrap();

        if path.extension().and_then(|str| str.to_str()) != Some("rtm") {
            continue;
        }

        let manifest_string = std::fs::read_to_string(&path).unwrap();
        let manifest = compile(
            &manifest_string,
            &NetworkDefinition::simulator(),
            MockBlobProvider::new(),
        )
        .unwrap();

        // Act
        let serializable_instructions = to_serializable_instructions(&manifest.instructions);

        // Assert
        serializable_instructions.expect(&format!("Failed on: {:?}", path));
    }
}

#[test]
fn common_manifests_can_be_converted_to_serialized_and_back() {
    // Arrange
    // let path = ".";
    let path = "/Users/omarabdulla/Desktop/toolkit-reboot/native-json-library/tests/manifests/resources/mint/non_fungible/";
    for entry in WalkDir::new(path) {
        let path = entry.unwrap().path().canonicalize().unwrap();

        if path.extension().and_then(|str| str.to_str()) != Some("rtm") {
            continue;
        }

        let manifest_string = std::fs::read_to_string(&path).unwrap();
        let manifest = compile(
            &manifest_string,
            &NetworkDefinition::simulator(),
            MockBlobProvider::new(),
        )
        .unwrap();

        let serializable_instructions =
            to_serializable_instructions(&manifest.instructions).unwrap();

        // Act
        let instructions = to_native_instructions(&serializable_instructions);

        // Assert
        instructions.expect(&format!("Failed on: {:?}", path));
    }
}

#[test]
fn common_manifests_can_be_converted_to_serialized_and_back_and_are_equal() {
    // Arrange
    // let path = ".";
    let path = "/Users/omarabdulla/Desktop/toolkit-reboot/native-json-library/tests/manifests/resources/mint/non_fungible/";
    for entry in WalkDir::new(path) {
        let path = entry.unwrap().path().canonicalize().unwrap();

        if path.extension().and_then(|str| str.to_str()) != Some("rtm") {
            continue;
        }

        let manifest_string = std::fs::read_to_string(&path).unwrap();
        let manifest = compile(
            &manifest_string,
            &NetworkDefinition::simulator(),
            MockBlobProvider::new(),
        )
        .unwrap();

        let serializable_instructions =
            to_serializable_instructions(&manifest.instructions).unwrap();

        // Act
        let instructions = to_native_instructions(&serializable_instructions).unwrap();

        // Assert
        assert_eq!(instructions, manifest.instructions)
    }
}

fn to_serializable_instructions(
    instructions: &[InstructionV1],
) -> Result<Vec<SerializableInstruction>, LocatedInstructionConversionError> {
    let network_id = 0xF2;
    let mut id_allocator = ManifestIdAllocator::default();

    instructions
        .iter()
        .enumerate()
        .map(|(instruction_index, instruction)| {
            SerializableInstruction::from_instruction(instruction, network_id, &mut id_allocator)
                .map_err(|error| LocatedInstructionConversionError {
                    instruction_index,
                    error,
                })
        })
        .collect::<Result<_, _>>()
}

fn to_native_instructions(
    instructions: &[SerializableInstruction],
) -> Result<Vec<InstructionV1>, LocatedInstructionConversionError> {
    instructions
        .iter()
        .enumerate()
        .map(|(instruction_index, instruction)| {
            instruction
                .to_instruction()
                .map_err(|error| LocatedInstructionConversionError {
                    instruction_index,
                    error,
                })
        })
        .collect::<Result<_, _>>()
}

#[derive(Debug, Clone)]
struct LocatedInstructionConversionError {
    instruction_index: usize,
    error: InstructionConversionError,
}
