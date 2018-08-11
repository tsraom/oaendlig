use rand::*;
use std::ops::Range;
use std::iter;

/// Returns a random number within given range.
pub fn rnd_within<Idx>(range: Range<Idx>) -> Idx
where distributions::Standard: distributions::Distribution<Idx>,
      Idx: ::num::Integer + Copy
{
    random::<Idx>() % (range.end - range.start) + range.start
}

/// Returns a random number less than given number.
pub fn rnd_lt<Num>(x: Num) -> Num
where distributions::Standard: distributions::Distribution<Num>,
      Num: ::num::Integer + Copy
{
    random::<Num>() % x
}

/// Returns a string with random characters at given length.
pub fn rnd_string(len: usize) -> String {
    iter::repeat_with(
        || repeat_until(|| random::<char>(), char::is_ascii_graphic)
    ).take(len).collect::<String>()
}

/// Repeat given closure until the return satisfies given predicate.
pub fn repeat_until<F, G, T>(mut closure: F, mut pred: G) -> T
where F: FnMut() -> T,
      G: FnMut(&T) -> bool
{
    let mut res = closure();
    while !pred(&res) {
        res = closure();
    }
    res
}
