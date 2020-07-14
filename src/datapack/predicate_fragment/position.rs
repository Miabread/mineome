use super::{Range, RangeOrNumber};
use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PositionPredicateFragment<T> {
    pub x: RangeOrNumber<T>,
    pub y: RangeOrNumber<T>,
    pub z: RangeOrNumber<T>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DistancePredicateFragment<T> {
    pub absolute: Option<Range<T>>,
    pub horizontal: Option<Range<T>>,
    pub x: Option<Range<T>>,
    pub y: Option<Range<T>>,
    pub z: Option<Range<T>>,
}
