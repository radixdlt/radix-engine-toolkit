// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

mod test_vector;

use radix_engine_toolkit::model::{Bech32Coder, Instruction};
use test_vector::*;

#[test]
fn serialized_instructions_match_expected() {
    // Checking that the serialization of instructions matches
    for test_vector in INSTRUCTION_CONVERSION_TEST_VECTORS.iter() {
        // Act
        let expected_serialized_instruction: serde_json::Value =
            serde_json::from_str(&test_vector.json_representation)
                .expect("Failed to deserialize trusted instruction");
        let serialized_instruction = serde_json::to_value(&test_vector.instruction)
            .expect("Failed to serialize trusted instruction");

        // Assert
        assert_eq!(expected_serialized_instruction, serialized_instruction);
    }
}

#[test]
fn deserialized_instructions_match_expected() {
    // Checking that the deserialization of instructions matches
    for test_vector in INSTRUCTION_CONVERSION_TEST_VECTORS.iter() {
        // Act
        let expected_instruction = &test_vector.instruction;
        let deserialized_instruction = serde_json::from_str(&test_vector.json_representation)
            .expect("Deserialization failed!");

        // Assert
        assert_eq!(*expected_instruction, deserialized_instruction)
    }
}

#[test]
fn instruction_ast_conversions_match_that_produced_by_transaction_compiler() {
    // Arrange
    let bech32_coder = Bech32Coder::new(0xf2);

    // Testing that the Instruction -> AstInstruction conversion matches that obtained from parsing
    // the manifest
    for test_vector in INSTRUCTION_CONVERSION_TEST_VECTORS.iter() {
        let expected_ast_instruction = test_vector.manifest_representation_as_ast_instruction();

        // Act
        let ast_instruction = test_vector
            .instruction
            .to_ast_instruction(&bech32_coder)
            .expect("Instruction -> AstInstruction conversion of trusted instruction failed");

        // Assert
        assert_eq!(expected_ast_instruction, ast_instruction)
    }
}

#[test]
fn no_information_is_lost_when_converting_instruction_to_ast_instruction_and_back() {
    // Arrange
    let bech32_coder = Bech32Coder::new(0xf2);

    // Testing that the Instruction -> AstInstruction conversion matches that obtained from parsing
    // the manifest
    for test_vector in INSTRUCTION_CONVERSION_TEST_VECTORS.iter() {
        let expected_instruction = &test_vector.instruction;

        // Act
        let ast_instruction = Instruction::from_ast_instruction(
            &test_vector
                .instruction
                .to_ast_instruction(&bech32_coder)
                .expect("Instruction -> AstInstruction conversion of trusted instruction failed"),
            &bech32_coder,
        )
        .expect("AstInstruction -> Instruction for a trusted instruction failed");

        // Assert
        assert_eq!(*expected_instruction, ast_instruction)
    }
}
