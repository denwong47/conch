/// Custom Trait for anything that have a `len`.
///
/// The standard library does not have a trait for this purpose, and arguably we don't
/// need one either, but at this point there is no harm.
pub trait HasLength {
    fn len(&self) -> usize;
}
