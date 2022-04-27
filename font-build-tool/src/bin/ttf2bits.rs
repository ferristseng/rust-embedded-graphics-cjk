use clap::{ArgEnum, Parser};
use embedded_graphics_cjk_font_build_tool::{
    BuildError, FontOutputSettings, MonoFontBuilder, UnicodeCodeBlock, CJK_RADICALS_SUPPLEMENT,
    CJK_UNIFIED_IDEOGRAPHS_UNICODE_BLOCK,
};

const UNICODE_CODE_BLOCKS: &[UnicodeCodeBlock] = &[
    UnicodeCodeBlock::new('?', '?'),
    CJK_RADICALS_SUPPLEMENT,
    CJK_UNIFIED_IDEOGRAPHS_UNICODE_BLOCK,
];

#[derive(Parser, Debug)]
#[clap(name = "ttf2bits", author, version, about, propagate_version = true)]
struct Ttf2Bits {
    font_path: String,

    output_prefix: String,

    #[clap(arg_enum)]
    output_format: OutputFormat,

    #[clap(short = 'o', long = "output-dir", default_value = ".")]
    output_directory: String,

    #[clap(long = "intensity-threshold", default_value = "128")]
    intensity_threshold: u8,

    #[clap(short = 's', long = "size")]
    font_sizes: Vec<u32>,
}

impl Ttf2Bits {
    fn run(self) -> Result<(), BuildError> {
        let mono_font_builder = MonoFontBuilder::new(self.font_path, UNICODE_CODE_BLOCKS)?;

        for font_size in self.font_sizes {
            let settings = FontOutputSettings {
                font_size,
                intensity_threshold: self.intensity_threshold,
            };
            let bitmap = mono_font_builder.build(settings)?;

            match self.output_format {
                OutputFormat::Rs => {
                    let bitmap_file = format!("{}-{}.bin", self.output_prefix, font_size);

                    bitmap.save_raw(format!("{}/data/{}", self.output_directory, bitmap_file))?;
                    bitmap.save_rust_source(
                        format!(
                            "{}/{}_{}.rs",
                            self.output_directory, self.output_prefix, font_size
                        ),
                        format!("data/{}", bitmap_file),
                    )?;
                }
                OutputFormat::Png => {
                    bitmap.save_png(format!(
                        "{}/{}-{}.png",
                        self.output_directory, self.output_prefix, font_size
                    ))?;
                }
            }
        }

        Ok(())
    }
}

#[derive(ArgEnum, Copy, Clone, Debug)]
enum OutputFormat {
    Rs,
    Png,
}

impl OutputFormat {}

fn main() -> Result<(), BuildError> {
    let program = Ttf2Bits::parse();

    program.run()
}
