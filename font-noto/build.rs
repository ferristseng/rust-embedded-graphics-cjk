use embedded_graphics_cjk_font_build_tool::{
    BuildError, FontOutputSettings, MonoFontBuilder, UnicodeCodeBlock, CJK_RADICALS_SUPPLEMENT,
    CJK_UNIFIED_IDEOGRAPHS_UNICODE_BLOCK,
};
use std::env;

const VERSION: &str = include_str!("NOTO_SANS_VERSION");

#[rustfmt::skip]
const FONTS: &[(&str, &str, &[u32])] = &[
    (
        "NotoSansMonoCJKsc-Regular",
        "noto_sans_mono_cjk_sc_regular",
        &[36]
    ),
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
        for &(font_name, output_name, font_sizes) in FONTS {
            let mono_font_builder = MonoFontBuilder::new(
                format!(
                    "target/font/13_NotoSansMonoCJKsc-{}/{}.otf",
                    VERSION, font_name
                ),
                UNICODE_CODE_BLOCKS,
            )?;

            for &font_size in font_sizes {
                mono_font_builder
                    .build(FontOutputSettings {
                        font_size,
                        intensity_threshold: 128,
                    })?
                    .save_all_with_default_paths(output_name, font_size)?;
            }
        }
    }

    Ok(())
}
