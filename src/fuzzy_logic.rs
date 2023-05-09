use crate::entities::Parameter;

pub trait FuzzySet {
    fn is_member(&self, x: impl Parameter) -> bool;
}
