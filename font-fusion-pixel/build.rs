use embedded_graphics_cjk_font_build_tool::{
    BuildError, FontOutputSettings, MonoFontBuilder, UnicodeCodeBlock, CJK_RADICALS_SUPPLEMENT,
    CJK_UNIFIED_IDEOGRAPHS_UNICODE_BLOCK,
};
use std::env;

const VERSION: &str = include_str!("FUSION_PIXEL_VERSION");

#[rustfmt::skip]
const FONT_SIZES: &[u32] = &[
    12, 24
];

const UNICODE_CODE_BLOCKS: &[UnicodeCodeBlock] = &[
    UnicodeCodeBlock::new('?', '?'),
    CJK_RADICALS_SUPPLEMENT,
    CJK_UNIFIED_IDEOGRAPHS_UNICODE_BLOCK,
];

fn main() -> Result<(), BuildError> {
    println!("cargo:rerun-if-env-changed=REBUILD_FONT_DATA");

    let rebuild_font_data = env::var("REBUILD_FONT_DATA")
        .map(|v| v == "1")
        .unwrap_or(false);

    if rebuild_font_data {
        for &font_size in FONT_SIZES {
            let mono_font_builder = MonoFontBuilder::new(
                format!("target/font/fusion-pixel-{}/fusion-pixel.otf", VERSION),
                UNICODE_CODE_BLOCKS,
            )?;

            mono_font_builder
                .build(FontOutputSettings {
                    font_size,
                    intensity_threshold: 0,
                })?
                .save_all_with_default_paths("fusion-pixel", font_size)?;
        }
    }

    Ok(())
}
