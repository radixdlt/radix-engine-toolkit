use radix_engine_common::prelude::*;

pub trait CheckedAddAssign<T> {
    fn checked_add_assign(&mut self, other: &Self) -> Option<()>;
}

impl CheckedAddAssign<Decimal> for Decimal {
    fn checked_add_assign(&mut self, other: &Self) -> Option<()> {
        *self = self.checked_add(*other)?;
        Some(())
    }
}
