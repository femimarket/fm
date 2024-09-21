use crate::instrument::Instrument;

#[derive(Debug,Clone,Copy,Ord, PartialOrd, Eq, PartialEq)]
pub struct Tick {
    pub ask: i64,
    pub bid: i64,
    pub mid: i64,
    pub v: i64,
    pub dt: i64,
    pub instrument: Instrument
}

