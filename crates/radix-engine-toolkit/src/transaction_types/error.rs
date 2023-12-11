pub enum TransactionTypesError {
    /// The is invalid for execution summary. This is typically because the
    /// receipt does not have the execution trace information or due to the
    /// transaction failing.
    InvalidReceipt,
}
