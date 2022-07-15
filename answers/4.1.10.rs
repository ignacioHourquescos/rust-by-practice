use std::ops::{Range, RangeInclusive};
fn main() {
    //range start from 1 to 5 (not included)
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    //5 included =>Creates a new inclusive range. Equivalent to writing start..=end.
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
}