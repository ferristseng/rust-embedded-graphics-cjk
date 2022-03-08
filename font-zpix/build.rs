use embedded_graphics_cjk_font_build_tool::{
    BuildError, FontOutputSettings, MonoFontBuilder, UnicodeCodeBlock, CJK_RADICALS_SUPPLEMENT,
    CJK_UNIFIED_IDEOGRAPHS_UNICODE_BLOCK,
};
use std::env;

const VERSION: &str = include_str!("ZPIX_VERSION");

#[rustfmt::skip]
const FONTS: &[(&str, &[u32])] = &[
    ("zpix", &[12, 24]),
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
        for &(font_name, font_sizes) in FONTS {
            let mono_font_builder = MonoFontBuilder::new(
                format!("target/font/zpix-{}/{}.ttf", VERSION, font_name),
                UNICODE_CODE_BLOCKS,
            )?;

            for &font_size in font_sizes {
                mono_font_builder
                    .build(FontOutputSettings {
                        font_size,
                        intensity_threshold: 0,
                    })?
                    .save_all_with_default_paths(font_name, font_size)?;
            }
        }
    }

    Ok(())
}
