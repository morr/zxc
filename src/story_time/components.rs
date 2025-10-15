use core::panic;

use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum YearSeason {
    Spring,
    Summer,
    Fall,
    Winter,
}

#[derive(Message, Debug)]
pub struct NewDayMessage(pub u32);

#[derive(Message, Debug)]
pub struct NewSeasonMessage(pub YearSeason);

#[derive(Message, Debug)]
pub struct NewYearMessage(pub u32);
