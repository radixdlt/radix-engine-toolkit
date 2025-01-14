use crate::internal_prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccountDynamicResourceMovementsAnalyzer(
    AccountDynamicResourceMovements,
);

impl ManifestStaticAnalyzer for AccountDynamicResourceMovementsAnalyzer {
    type Initializer = ();
    type Output = ();
    type PermissionState = ConstState<true>;
    type RequirementState = ConstState<true>;

    fn new(
        _: Self::Initializer,
    ) -> (Self, Self::PermissionState, Self::RequirementState) {
        Default::default()
    }

    fn output(self) -> Self::Output {}

    fn process_permission(
        &mut self,
        _: &mut Self::PermissionState,
        _: &NamedAddressStore,
        _: &GroupedInstruction,
        _: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
    }

    fn process_requirement(
        &mut self,
        _: &mut Self::RequirementState,
        _: &NamedAddressStore,
        _: &GroupedInstruction,
        _: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
    }

    fn process_instruction(
        &mut self,
        _: &NamedAddressStore,
        _: &GroupedInstruction,
        _: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
    }
}

impl ManifestDynamicAnalyzer for AccountDynamicResourceMovementsAnalyzer {
    type Output = AccountDynamicResourceMovements;
    type RequirementState = ConstState<true>;

    fn new(
        _: Self::Initializer,
    ) -> (
        Self,
        <Self as ManifestStaticAnalyzer>::PermissionState,
        <Self as ManifestStaticAnalyzer>::RequirementState,
        <Self as ManifestDynamicAnalyzer>::RequirementState,
    ) {
        Default::default()
    }

    fn output(
        self,
    ) -> CombinedAnalysisOutput<
        <Self as ManifestStaticAnalyzer>::Output,
        <Self as ManifestDynamicAnalyzer>::Output,
    > {
        CombinedAnalysisOutput {
            static_analyzer_output: (),
            dynamic_analyzer_output: self.0,
        }
    }

    fn process_requirement(
        &mut self,
        _: &mut <Self as ManifestDynamicAnalyzer>::RequirementState,
        _: &NamedAddressStore,
        _: &GroupedInstruction,
        _: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
    }

    fn process_instruction(
        &mut self,
        _: &NamedAddressStore,
        _: &GroupedInstruction,
        invocation_io: &InvocationIo<InvocationIoItems>,
        maybe_typed_invocation: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
        // Note: it was deemed to not be worth it to support dynamic addresses
        // here as this information is not exactly useful to the user. What is
        // a user supposed to do with "a new resource was created and withdrawn
        // from your account"? There isn't much that anybody can do with this
        // kind of information and there really isn't much that a simple user
        // can understand from this. Supporting this only adds complexity to
        // all parts of the code base without offering something that Sargon, or
        // any other client will have a use of. If clients have a use for this,
        // maybe they can write their own analyzer that does this :).
        match maybe_typed_invocation {
            Some((
                ManifestInvocationReceiver::GlobalMethod(
                    ResolvedManifestAddress::Static {
                        static_address: account_address,
                    },
                ),
                TypedManifestNativeInvocation::AccountBlueprintInvocation(
                    AccountBlueprintInvocation::Method(
                        AccountBlueprintMethod::Withdraw(AccountWithdrawManifestInput {
                            resource_address:
                                ManifestResourceAddress::Static(resource_address),
                            amount,
                        })
                        | AccountBlueprintMethod::LockFeeAndWithdraw(
                            AccountLockFeeAndWithdrawManifestInput {
                                resource_address:
                                    ManifestResourceAddress::Static(resource_address),
                                amount,
                                ..
                            },
                        ),
                    ),
                ),
            )) => {
                let is_fungible_resource_manager = resource_address.is_fungible();

                // This is a withdraw of amount of a fungible resource, we will
                // take note of the amount of resources withdrawn from the
                // account without the use of the dynamic analysis information.
                if is_fungible_resource_manager {
                    self.0
                        .account_withdraws
                        .entry(*account_address)
                        .or_default()
                        .push(InvocationIoItem::new_guaranteed_fungible(
                            *resource_address,
                            *amount,
                        ))
                }
                // This is a withdraw of a non-fungible by amount. So, we get
                // the ids from the dynamic information.
                else {
                    self.0
                        .account_withdraws
                        .entry(*account_address)
                        .or_default()
                        .extend(
                            invocation_io
                                .output
                                .io_of_resource(resource_address)
                                .cloned(),
                        );
                }
            }
            Some((
                ManifestInvocationReceiver::GlobalMethod(
                    ResolvedManifestAddress::Static {
                        static_address: account_address,
                    },
                ),
                TypedManifestNativeInvocation::AccountBlueprintInvocation(
                    AccountBlueprintInvocation::Method(
                        AccountBlueprintMethod::WithdrawNonFungibles(
                            AccountWithdrawNonFungiblesManifestInput {
                                resource_address:
                                    ManifestResourceAddress::Static(resource_address),
                                ids,
                            },
                        )
                        | AccountBlueprintMethod::LockFeeAndWithdrawNonFungibles(
                            AccountLockFeeAndWithdrawNonFungiblesManifestInput {
                                resource_address:
                                    ManifestResourceAddress::Static(resource_address),
                                ids,
                                ..
                            },
                        ),
                    ),
                ),
            )) => {
                self.0
                    .account_withdraws
                    .entry(*account_address)
                    .or_default()
                    .push(InvocationIoItem::new_guaranteed_non_fungible(
                        *resource_address,
                        ids.clone(),
                    ));
            }
            Some((
                ManifestInvocationReceiver::GlobalMethod(
                    ResolvedManifestAddress::Static {
                        static_address: account_address,
                    },
                ),
                TypedManifestNativeInvocation::AccountBlueprintInvocation(
                    AccountBlueprintInvocation::Method(
                        AccountBlueprintMethod::Deposit(..)
                        | AccountBlueprintMethod::DepositBatch(..)
                        | AccountBlueprintMethod::TryDepositOrAbort(..)
                        | AccountBlueprintMethod::TryDepositBatchOrAbort(..),
                    ),
                ),
            )) => {
                self.0
                    .account_withdraws
                    .entry(*account_address)
                    .or_default()
                    .extend(invocation_io.input.items_iter().cloned());
            }
            _ => {}
        }
    }
}
