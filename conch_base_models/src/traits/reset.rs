/// Trait for anything that has a resetter method.
pub trait Resetter {
    fn resetter(&self, input: Option<&str>) -> Self;
}
