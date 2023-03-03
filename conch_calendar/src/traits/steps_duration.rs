use chrono::{Duration, NaiveDate};
use core::ops::{Range, RangeInclusive};
use std::ops::Add;

/// Iterator struct to step over a [`Range`] or [`RangeInclusive`] of time values that
/// supports [`std::ops::Add`] [`Duration`]
pub struct StepsOverRange<T> {
    range: T,
    count: usize,
    duration: Duration,
}
impl<T> StepsOverRange<T> {
    /// Create a new instance of [`StepsOverRange`] and set its [`Self::count`] to `0`.
    pub fn new(range: T, duration: Duration) -> Self {
        Self {
            range,
            count: 0,
            duration,
        }
    }
}

macro_rules! impl_factory {
    ($(($struct:ty, $start:expr)),+) => {
        $(
            impl<Idx> Iterator for StepsOverRange<$struct>
            where
                Idx: Add<Duration, Output=Idx> + Clone + PartialOrd
            {
                type Item = Idx;

                fn next(&mut self) -> Option<Self::Item> {
                    let item: Idx = $start(&self.range) + self.duration * (self.count as i32);
                    self.count += 1;

                    if self.range.contains(&item) { Some(item) }
                    else { None }
                }
            }
        )*
    };
}

impl_factory!(
    (Range<Idx>, |range: &Range<Idx>| range.start.clone()),
    (RangeInclusive<Idx>, |range: &RangeInclusive<Idx>| range.start().clone())
);

/// Trait to allow a [`Range<Idx>`] or [`RangeInclusive<Idx>`] of time values to produce
/// an [`Iterator<Idx>`] which steps over the range by a specified [`Duration`].
pub trait IterRangeByDuration
where
    Self: Sized,
{
    fn into_iter_by_duration(self, duration: Duration) -> StepsOverRange<Self>;
}

impl IterRangeByDuration for RangeInclusive<NaiveDate> {
    fn into_iter_by_duration(self, duration: Duration) -> StepsOverRange<Self> {
        StepsOverRange::new(self, duration)
    }
}
