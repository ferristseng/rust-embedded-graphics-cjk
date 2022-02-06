use embedded_graphics_cjk_font_build_tool::{
    BuildError, MonoFontBuilder, CJK_RADICALS_SUPPLEMENT, CJK_UNIFIED_IDEOGRAPHS_UNICODE_BLOCK,
};
use fontdue::Font;
use std::fs;

const VERSION: &str = "0.35.8";

const FONTS: &[(&str, u32)] = &[
    ("sarasa-mono-sc-light", 24),
    ("sarasa-mono-slab-sc-light", 24),
];

fn main() -> Result<(), BuildError> {
    println!(
        "cargo:rerun-if-changed=target/font/sarasa-gothic-ttf-{}/sarasa-mono-sc-light.ttf",
        VERSION
    );
    println!(
        "cargo:rerun-if-changed=target/font/sarasa-gothic-ttf-{}/sarasa-mono-slab-sc-light.ttf",
        VERSION
    );

    for (font_name, font_size) in FONTS {
        let path = format!(
            "target/font/sarasa-gothic-ttf-{}/{}.ttf",
            VERSION, font_name
        );
        let font = {
            let ttf_file = fs::read(path)?;

            Font::from_bytes(ttf_file, Default::default())
                .map_err(|message| BuildError::ReadFontError { message })?
        };

        let mono_font_builder = MonoFontBuilder {
            font: &font,
            font_size: *font_size,
            chars: ('?'..='?')
                .chain(CJK_RADICALS_SUPPLEMENT)
                .chain(CJK_UNIFIED_IDEOGRAPHS_UNICODE_BLOCK),
            intensity_threshold: 120,
        };

        let mono_font = mono_font_builder.build()?;
        let bin_data_name = format!("{}-{}.bin", font_name, font_size);

        mono_font.save_png(format!("png/{}-{}.png", font_name, font_size))?;
        mono_font.save_raw(format!("src/data/{}", bin_data_name))?;

        mono_font.save_rust_source(
            format!("src/{}_{}.rs", font_name.replace('-', "_"), font_size),
            format!("data/{}", bin_data_name),
        )?;
    }

    Ok(())
}
