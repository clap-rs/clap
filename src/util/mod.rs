#![allow(clippy::single_component_path_imports)]

mod fnv;
mod graph;
mod id;
mod str_to_bool;

pub use self::fnv::Key;

pub(crate) use self::str_to_bool::str_to_bool;
pub(crate) use self::str_to_bool::FALSE_LITERALS;
pub(crate) use self::str_to_bool::TRUE_LITERALS;
pub(crate) use self::{graph::ChildGraph, id::Id};

pub(crate) mod color;

pub(crate) const SUCCESS_CODE: i32 = 0;
// While sysexists.h defines EX_USAGE as 64, this doesn't seem to be used much in practice but
// instead 2 seems to be frequently used.
// Examples
// - GNU `ls` returns 2
// - Python's `argparse` returns 2
pub(crate) const USAGE_CODE: i32 = 2;

pub(crate) fn safe_exit(code: i32) -> ! {
    use std::io::Write;

    let _ = std::io::stdout().lock().flush();
    let _ = std::io::stderr().lock().flush();

    std::process::exit(code)
}

#[cfg(not(feature = "unicode"))]
pub(crate) fn eq_ignore_case(left: &str, right: &str) -> bool {
    left.eq_ignore_ascii_case(right)
}

#[cfg(feature = "unicode")]
pub(crate) use unicase::eq as eq_ignore_case;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub(crate) enum Ord {
    Lt(usize),
    Gt(usize),
}

#[derive(Debug, Clone)]
pub struct ClapRange<T> {
    pub(crate) inclusive: bool,
    pub(crate) start: Option<T>,
    pub(crate) end: Option<T>,
}

impl Default for ClapRange<usize> {
    fn default() -> Self {
        ClapRange {
            inclusive: false,
            start: Some(1),
            end: None,
        }
    }
}

impl ClapRange<usize> {
    pub(crate) fn is_valid(&self, num: usize) -> bool {
        self.pend(num).is_ok()
    }

    pub(crate) fn pend(&self, num: usize) -> Result<(), Ord> {
        if let Some(start) = self.start {
            if num < start {
                return Err(Ord::Lt(start));
            }
        }
        if let Some(end) = self.end {
            if self.inclusive {
                if num > end {
                    return Err(Ord::Gt(end));
                }
            } else if num >= end {
                // Saturating would be useful only if somebody uses `..0`
                // for the num value range.(which should be defended in arg
                // building).
                return Err(Ord::Gt(end.saturating_sub(1)));
            }
        }
        Ok(())
    }

    /// Is more values needed to satisfy the range?
    pub(crate) fn need_more(&self, current: usize) -> bool {
        // Only if current + 1 is bigger than the range, we don't need more values.
        !matches!(self.pend(current + 1), Err(Ord::Gt(_)))
    }

    pub(crate) fn num_vals(&self) -> Option<usize> {
        if self.inclusive {
            match (self.start, self.end) {
                (Some(a), Some(b)) if a == b => Some(a),
                _ => None,
            }
        } else {
            match (self.start, self.end) {
                (Some(a), Some(b)) if b - a == 1 => Some(a),
                _ => None,
            }
        }
    }
}

impl From<usize> for ClapRange<usize> {
    fn from(num: usize) -> Self {
        ClapRange {
            inclusive: true,
            start: Some(num),
            end: Some(num),
        }
    }
}

impl<T> From<std::ops::Range<T>> for ClapRange<T> {
    fn from(range: std::ops::Range<T>) -> Self {
        ClapRange {
            inclusive: false,
            start: Some(range.start),
            end: Some(range.end),
        }
    }
}

impl<T> From<std::ops::RangeFull> for ClapRange<T> {
    fn from(_: std::ops::RangeFull) -> Self {
        ClapRange {
            inclusive: false,
            start: None,
            end: None,
        }
    }
}

impl<T> From<std::ops::RangeFrom<T>> for ClapRange<T> {
    fn from(range: std::ops::RangeFrom<T>) -> Self {
        ClapRange {
            inclusive: false,
            start: Some(range.start),
            end: None,
        }
    }
}

impl<T> From<std::ops::RangeTo<T>> for ClapRange<T> {
    fn from(range: std::ops::RangeTo<T>) -> Self {
        ClapRange {
            inclusive: false,
            start: None,
            end: Some(range.end),
        }
    }
}

impl<T> From<std::ops::RangeInclusive<T>> for ClapRange<T> {
    fn from(range: std::ops::RangeInclusive<T>) -> Self {
        let (start, end) = range.into_inner();
        ClapRange {
            inclusive: true,
            start: Some(start),
            end: Some(end),
        }
    }
}

impl<T> From<std::ops::RangeToInclusive<T>> for ClapRange<T> {
    fn from(range: std::ops::RangeToInclusive<T>) -> Self {
        ClapRange {
            inclusive: true,
            start: None,
            end: Some(range.end),
        }
    }
}
