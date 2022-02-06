use embedded_graphics::{
    geometry::Size,
    image::ImageRaw,
    mono_font::{mapping::StrGlyphMapping, DecorationDimensions, MonoFont},
};

const GLYPH_MAPPING: StrGlyphMapping =
    StrGlyphMapping::new("?\0\u{2e80}\u{2ef3}\0\u{4E00}\u{9FEF}", 0);

pub const FONT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("data/zpix-24.bin"),
        32 * 24,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(24, 24),
    character_spacing: 0,
    baseline: 0,
    underline: DecorationDimensions::new(25, 1),
    strikethrough: DecorationDimensions::new(12, 1),
};