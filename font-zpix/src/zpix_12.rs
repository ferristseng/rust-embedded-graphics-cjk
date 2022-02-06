use embedded_graphics::{
    geometry::Size,
    image::ImageRaw,
    mono_font::{DecorationDimensions, MonoFont},
};
use embedded_graphics_cjk_glyph_mapping::RangeGlyphMapping;

#[rustfmt::skip]
pub const FONT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("data/zpix-12.bin"),
        32 * 12,
    ),
    glyph_mapping: &RangeGlyphMapping::new_unchecked(
        [
            '?'..='?',                      // ?
            '\u{2E80}'..='\u{2EF3}',    // CJK Radicals Supplement
            '\u{4E00}'..='\u{9FFF}',    // CJK Unified Ideographs
        ],
        0
    ),
    character_size: Size::new(12, 12),
    character_spacing: 0,
    baseline: 0,
    underline: DecorationDimensions::new(13, 1),
    strikethrough: DecorationDimensions::new(6, 1),
};