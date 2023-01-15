use crate::model::Value;

/// A trait that defines a set of methods for aggregating value refs from objects
pub trait ValueRef {
    /// Used to borrow all values immutably
    fn borrow_values(&self) -> Vec<&Value>;

    /// Borrows all values mutably
    fn borrow_values_mut(&mut self) -> Vec<&mut Value>;
}
