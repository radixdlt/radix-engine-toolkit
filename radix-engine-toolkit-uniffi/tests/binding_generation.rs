//! This module tests the generated bindings to ensure that that they at least compile.

uniffi::build_foreign_language_testcases!(
    // "tests/bindings/example.swift",
    "tests/bindings/example.kts",
);
