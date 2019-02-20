use std::u64;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Occurrence {
    min: u64,
    max: u64
}

impl Default for Occurrence {
    fn default() -> Self {
        Occurrence {
            min: 1,
            max: u64::MAX
        }
    }
}

impl Occurrence {
    #[inline(always)]
    pub fn min(&mut self, num: u64) {
        self.min = num;
    }

    #[inline(always)]
    pub fn max(&mut self, num: u64) {
        self.max = num;
    }

    #[inline]
    pub fn exact(&mut self, num: u64) {
        self.max = num;
        self.min = num;
    }
}