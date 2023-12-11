//! Functions that expose the transaction types functionality without exposing
//! any of the implementation details of how the module finds and determines
//! the transaction types.

use radix_engine::prelude::*;
use radix_engine::transaction::*;
use transaction::prelude::*;

use super::error::*;
use super::types::*;

pub fn summary(manifest: &TransactionManifestV1) -> IndexSet<ManifestSummary> {
    todo!()
}

pub fn execution_summary(
    manifest: &TransactionManifestV1,
    receipt: &TransactionReceipt,
) -> Result<ExecutionSummary, TransactionTypesError> {
    todo!()
}
