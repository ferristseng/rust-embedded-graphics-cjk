use embedded_graphics::mono_font::mapping::GlyphMapping;
use std::ops::RangeInclusive;

pub struct RangeGlyphMapping<const SIZE: usize> {
    ranges: [RangeInclusive<char>; SIZE],
    default_idx: usize,
}

impl<const SIZE: usize> RangeGlyphMapping<SIZE> {
    /// Returns a new `RangeGlyphMapping`.
    ///
    /// `default_idx` is validated to ensure that it references a valid index.
    pub fn new(ranges: [RangeInclusive<char>; SIZE], default_idx: usize) -> Self {
        let map = Self {
            ranges,
            default_idx,
        };

        if default_idx > map.chars_in_range() {
            panic!("Default mapping index exceeded number of characters in the range")
        }

        map
    }

    /// Returns a new `RangeGlyphMapping`, but ranges aren't checked to ensure
    /// they aren't empty, and `default_idx` isn't validated.
    pub const fn new_unchecked(ranges: [RangeInclusive<char>; SIZE], default_idx: usize) -> Self {
        Self {
            ranges,
            default_idx,
        }
    }

    /// Determines how many characters are covered by all of the ranges
    /// specified in the `RangeGlyphMapping`.
    pub fn chars_in_range(&self) -> usize {
        // Add 1, since the range is inclusive
        self.ranges
            .iter()
            .map(|range| *range.end() as usize - *range.start() as usize + 1)
            .sum()
    }
}

impl<const SIZE: usize> GlyphMapping for RangeGlyphMapping<SIZE> {
    fn index(&self, chr: char) -> usize {
        let mut index = 0;
        for range in &self.ranges {
            if range.contains(&chr) {
                return index + chr as usize - *range.start() as usize;
            }

            index += *range.end() as usize - *range.start() as usize + 1;
        }
        self.default_idx
    }
}

#[cfg(test)]
mod tests {
    use super::RangeGlyphMapping;
    use embedded_graphics::mono_font::mapping::GlyphMapping;

    #[test]
    fn test_range_one_element() {
        let map = RangeGlyphMapping::new(['?'..='?'], 0);

        assert_eq!(map.chars_in_range(), 1);
        assert_eq!(map.index('?'), 0);
    }

    #[test]
    fn test_range_cjk_unified_ideographs() {
        let map =
            RangeGlyphMapping::new(['?'..='?', '\u{4E00}'..='\u{9FFF}'], 0);

        //     1: Question Mark
        // 20992: Code points in CJK Unified Ideographs
        assert_eq!(map.chars_in_range(), 1 + 20992);
        assert_eq!(map.index('A'), 0);
        assert_eq!(map.index('\u{4E41}'), 1 + 65);
    }

    #[test]
    fn test_range_cjk_radicals_supplement() {
        let map = RangeGlyphMapping::new(['?'..='?', '\u{2E80}'..='\u{2EF3}'], 0);

        //   1: Question Mark
        // 116: Code points in CJK Radicals Supplment (incl. U+2E9A, which is
        //      unassigned)
        assert_eq!(map.chars_in_range(), 1 + 116);
        assert_eq!(map.index('A'), 0);
        assert_eq!(map.index('\u{2E89}'), 1 + 9);
        assert_eq!(map.index('\u{2EBA}'), 1 + 58);
    }
}
