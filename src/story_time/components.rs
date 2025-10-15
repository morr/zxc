use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum YearSeason {
    Spring,
    Summer,
    Fall,
    Winter,
}

#[derive(Event, Debug)]
pub struct NewDayEvent(pub u32);

#[derive(Event, Debug)]
pub struct NewSeasonEvent(pub YearSeason);

#[derive(Event, Debug)]
pub struct NewYearEvent(pub u32);
