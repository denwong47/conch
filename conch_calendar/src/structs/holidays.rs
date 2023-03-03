use std::marker::PhantomData;

use chrono::NaiveDate;
// use chrono::offset::Local;

use crate::{HolidayList, RegionMarker};

/// Structs for static method to list public holidays of a region in a specific year.
pub struct Holidays<Region>
where
    Region: RegionMarker,
{
    region: PhantomData<Region>,
}

/// Generate list of bank holidays for a given year.
impl<Region> HolidayList for Holidays<Region>
where
    Region: RegionMarker,
{
    fn list(year: i32) -> Vec<NaiveDate> {
        Region::list_holidays(year)
    }
}
