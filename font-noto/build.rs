use embedded_graphics_cjk_font_build_tool::{
    BuildError, MonoFontBuilder, CJK_RADICALS_SUPPLEMENT, CJK_UNIFIED_IDEOGRAPHS_UNICODE_BLOCK,
};

const VERSION: &str = include_str!("NOTO_SANS_VERSION");

#[rustfmt::skip]
const FONTS: &[(&str, &str, u32)] = &[
    ("NotoSansMonoCJKsc-Regular", "noto_sans_mono_cjk_sc_regular", 24),
    ("NotoSansMonoCJKsc-Regular", "noto_sans_mono_cjk_sc_regular", 36)
];

fn main() -> Result<(), BuildError> {
    println!(
        "cargo:rerun-if-changed=target/font/13_NotoSansMonoCJKsc-{}/NotoSansMonoCJKsc-Regular.otf",
        VERSION
    );

    for &(font_name, output_name, font_size) in FONTS {
        let mono_font_builder = MonoFontBuilder::new(
            format!("target/font/13_NotoSansMonoCJKsc-{}/{}.otf", VERSION, font_name),
            font_size,
            ('?'..='?')
                .chain(CJK_RADICALS_SUPPLEMENT.range())
                .chain(CJK_UNIFIED_IDEOGRAPHS_UNICODE_BLOCK.range()),
            128,
        )?;

        mono_font_builder
            .build()?
            .save_all_with_default_paths(output_name, font_size)?;
    }

    Ok(())
}
