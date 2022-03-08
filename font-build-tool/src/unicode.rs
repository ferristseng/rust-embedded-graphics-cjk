use std::ops::RangeInclusive;

pub struct UnicodeCodeBlock {
    start: char,
    end: char,
}

impl UnicodeCodeBlock {
    pub const fn new(start: char, end: char) -> UnicodeCodeBlock {
        if end > start {
            panic!("end of unicode block must be greater than the start");
        }

        UnicodeCodeBlock { start, end }
    }

    /// Unicode block expressed as an iterable range of characters.
    pub const fn range(&self) -> RangeInclusive<char> {
        RangeInclusive::new(self.start, self.end)
    }

    /// Number of characters covered by the code block.
    pub const fn block_size(&self) -> usize {
        self.end as usize - self.start as usize
    }
}

// Code Points           : 20992
// Wiki                  : https://en.wikipedia.org/wiki/CJK_Unified_Ideographs_(Unicode_block)
// Unicode Version       : 14.0
// Date Updated in Crate : 2022-02-06
// Notes                 :
pub const CJK_UNIFIED_IDEOGRAPHS_UNICODE_BLOCK: UnicodeCodeBlock = UnicodeCodeBlock {
    start: '\u{4E00}',
    end: '\u{9FFF}',
};

// Code Points           : 115
// Wiki                  : https://en.wikipedia.org/wiki/CJK_Radicals_Supplement
// Unicode Version       : 3.0
// Date Updated in Crate : 2022-02-06
// Notes                 :
//   2022-02-06          :
//     U+2E9A is unassigned, but is included in the range. Thus, this range
//     covers 116 code points, even though technically only 115 are assigned.
pub const CJK_RADICALS_SUPPLEMENT: UnicodeCodeBlock = UnicodeCodeBlock {
    start: '\u{2E80}',
    end: '\u{2EF3}',
};
