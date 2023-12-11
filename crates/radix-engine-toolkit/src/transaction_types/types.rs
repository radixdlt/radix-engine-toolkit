use std::ops::Deref;

use radix_engine::transaction::*;

/// A receipt used for the calculation of the execution summary. This receipt
/// must belong to a transaction that executed successfully and the execution
/// trace must be present.
pub struct TransactionTypesReceipt<'r> {
    receipt: &'r TransactionReceipt,
    commit_result: &'r CommitResult,
    execution_trace: &'r TransactionExecutionTrace,
}

impl<'r> TransactionTypesReceipt<'r> {
    pub fn new(receipt: &'r TransactionReceipt) -> Option<Self> {
        if let TransactionResult::Commit(
            ref commit_result @ CommitResult {
                execution_trace: Some(ref execution_trace),
                ..
            },
        ) = &receipt.result
        {
            Some(Self {
                receipt,
                commit_result,
                execution_trace,
            })
        } else {
            None
        }
    }
}

impl<'r> Deref for TransactionTypesReceipt<'r> {
    type Target = TransactionReceipt;

    fn deref(&self) -> &Self::Target {
        &self.receipt
    }
}
