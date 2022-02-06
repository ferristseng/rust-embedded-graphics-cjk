use std::ops::RangeInclusive;

// https://en.wikipedia.org/wiki/CJK_Unified_Ideographs_(Unicode_block)
pub const CJK_UNIFIED_IDEOGRAPHS_UNICODE_BLOCK: RangeInclusive<char> = '\u{4E00}'..='\u{9FEF}';

pub const CJK_RADICALS_SUPPLEMENT: RangeInclusive<char> = '\u{2e80}'..='\u{2ef3}';
