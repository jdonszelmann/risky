use core::ops::Range as Corerange;
use core::ops::RangeInclusive;
use core::slice;
extern crate static_assertions as sa;

/// Represents a range of integers (or memory pointers).
/// [A (half-open) range bounded inclusively below and exclusively above (start..end).](core::ops::Range)
/// very similar to the built-in range. This implementation includes some operations
/// on ranges such as subtracting two.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl From<Corerange<usize>> for Range {
    fn from(r: Corerange<usize>) -> Self {
        Self {
            start: r.start,
            end: r.end
        }
    }
}

impl From<RangeInclusive<usize>> for Range {
    fn from(r: RangeInclusive<usize>) -> Self {
        Self {
            start: *r.start(),
            end: r.end() + 1
        }
    }
}

impl Range {
    /// Creates a new range, panics if start > end
    pub fn new(start: usize, end: usize) -> Self {
        assert!(end >= start);

        Self {
            start,
            end
        }
    }

    /// creates a new range, will not panic when start > end.
    /// This should however never be allowed to occur.
    pub const fn new_unchecked(start: usize, end: usize) -> Self {
        Self {
            start,
            end
        }
    }


    /// Removes one range from another. May return two ranges if
    /// self totally includes other
    pub fn subtract(&self, other: Range) -> (Range, Option<Range>) {
        if self.start == other.start {
            (Self {
                start: other.end,
                end: self.end
            }, None)
        } else if self.end == other.end {
            (Self {
                start: self.start,
                end: other.start
            }, None)
        } else {
            (Self{
                start: self.start,
                end: other.start,
            }, Some(Self {
                start: other.end,
                end: self.end,
            }))
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.start as *const u8, self.end-self.start)
        }
    }

    pub fn contains(&self, other: usize) -> bool {
        other >= self.start && other < self.end
    }

    /// Returns the length of this range
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Splits off a section of `len` bytes. Returns a range of `len` and a range
    pub fn split(&self, len: usize) -> Option<(Self, Self)> {
        if len > self.len() {
            return None
        }

        Some((Self {
            start: self.start,
            end: self.start + len
        }, Self {
            start: self.start + len,
            end: self.end
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::ds::range::Range;

    #[test_case]
    pub fn test_range_from_core_range() {
        assert_eq!(Range::from(0..5), Range::new(0, 5));
        assert_eq!(Range::from(0..=5), Range::new(0, 6));
    }

    #[test_case]
    pub fn test_range_contains() {
        assert!( Range::new(3, 5).contains(4));
        assert!( Range::new(3, 5).contains(3));
        assert!(!Range::new(3, 5).contains(2));
        assert!(!Range::new(3, 5).contains(5));
    }
}