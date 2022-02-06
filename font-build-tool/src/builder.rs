use crate::error::BuildError;
use fontdue::Font;
use image::{EncodableLayout, ImageBuffer, ImageResult, Luma, PixelWithColorType};
use std::{cmp::max, fmt::Display, fs, io, ops::Deref, path::Path};

/// The number of glyphs to include on a single line in the final bitmap.
const ROW_SIZE: usize = 32;

pub struct MonoFontBuilder<'a, I: Iterator<Item = char>> {
    pub font: &'a Font,

    /// The target font size.
    pub font_size: u32,

    /// The characters to generate bitmaps for.
    pub chars: I,

    /// Threshold for when to select a pixel when adding it to the bitmap.
    /// A reasonable value for this would be 128, meaning anything above 50%
    /// intensity will appear in the final bitmap (max threshold = 255).
    pub intensity_threshold: u8,
}

impl<'a, I> MonoFontBuilder<'a, I>
where
    I: Iterator<Item = char>,
{
    pub fn build(self) -> Result<MonoFontData<ImageBuffer<Luma<u8>, Vec<u8>>>, BuildError> {
        let chars = self.chars.collect::<Vec<_>>();

        // Determines the maximum glyph height and glyph width based on the
        // glyph metrics for each chosen character.
        let (max_glyph_height, max_glyph_width) = chars
            .iter()
            .map(|chr| {
                let metrics = self.font.metrics(*chr, self.font_size as f32);

                (metrics.height, metrics.width)
            })
            .reduce(
                |(max_glyph_height, max_glyph_width), (glyph_height, glyph_width)| {
                    (
                        max(max_glyph_height, glyph_height),
                        max(max_glyph_width, glyph_width),
                    )
                },
            )
            .expect("expected at least one character");

        // Image buffer that contains every glyph specified in rows of ROW_SIZE.
        let mut imgbuf = image::GrayImage::new(
            (max_glyph_width * ROW_SIZE) as u32,
            (max_glyph_height * ((chars.len() - 1) / ROW_SIZE + 1)) as u32,
        );

        // Rasterizes the font, and copies the bitmap onto the image buffer.
        for (index, chr) in chars.iter().enumerate() {
            let (metrics, bitmap) = self.font.rasterize(*chr, self.font_size as f32);

            let col = index % ROW_SIZE;
            let row = index / ROW_SIZE;
            let img_x = col * max_glyph_width;
            let img_y = row * max_glyph_height;

            // Copy onto image
            for y in (0..metrics.height).rev() {
                let (row_start, row_end) = (y * metrics.width, (y + 1) * metrics.width);

                let row = &bitmap[row_start..row_end];
                for x in 0..metrics.width {
                    let val = row[x];

                    if val > self.intensity_threshold {
                        let pixel_x = img_x + x;
                        let pixel_y = img_y + y;
                        if pixel_x > 0 && pixel_y > 0 {
                            imgbuf.put_pixel(pixel_x as u32, pixel_y as u32, Luma([0xFF]));
                        }
                    }
                }
            }
        }

        Ok(MonoFontData {
            data: imgbuf,
            glyph_width: max_glyph_width,
            glyph_height: max_glyph_height,
        })
    }
}

pub struct MonoFontData<C> {
    data: C,
    glyph_width: usize,
    glyph_height: usize,
}

impl<C> MonoFontData<C> {
    /// Writes the Rust source code that enables the generated font data to be
    /// used with embedded-graphics.
    ///
    /// This generates a single Rust source code file, with a single constant
    /// named `FONT`. `FONT` can then be imported in `lib.rs` and re-exported
    /// with the desired name.
    pub fn save_rust_source<P0, P1>(
        &self,
        rust_source_path: P0,
        bin_data_path: P1,
    ) -> io::Result<()>
    where
        P0: AsRef<Path>,
        P1: AsRef<Path> + Display,
    {
        // TODO: Make this better
        #[rustfmt::skip]
        let source = format!(
r#"use embedded_graphics::{{
    geometry::Size,
    image::ImageRaw,
    mono_font::{{DecorationDimensions, MonoFont}},
}};
use embedded_graphics_cjk_glyph_mapping::RangeGlyphMapping;

#[rustfmt::skip]
pub const FONT: MonoFont = MonoFont {{
    image: ImageRaw::new_binary(
        include_bytes!("{bin_data_path}"),
        {chars_per_row} * {glyph_width},
    ),
    glyph_mapping: &RangeGlyphMapping::new_unchecked(
        [
            '?'..='?',                      // ?
            '\u{{2E80}}'..='\u{{2EF3}}',    // CJK Radicals Supplement
            '\u{{4E00}}'..='\u{{9FFF}}',    // CJK Unified Ideographs
        ],
        0
    ),
    character_size: Size::new({glyph_width}, {glyph_height}),
    character_spacing: 0,
    baseline: 0,
    underline: DecorationDimensions::new({underline}, 1),
    strikethrough: DecorationDimensions::new({strikethrough}, 1),
}};"#,
            bin_data_path = bin_data_path,
            chars_per_row = ROW_SIZE,
            glyph_width = self.glyph_width,
            glyph_height = self.glyph_height,
            underline = self.glyph_height + 1,
            strikethrough = self.glyph_height / 2
        );

        fs::write(rust_source_path, &source)
    }
}

impl<Pxl, Container> MonoFontData<ImageBuffer<Pxl, Container>>
where
    Pxl: PixelWithColorType,
    [Pxl::Subpixel]: EncodableLayout,
    Container: Deref<Target = [Pxl::Subpixel]>,
{
    /// Saves the image data as a PNG for debugging. This data isn't included
    /// in the library.
    pub fn save_png<P>(&self, png_file: P) -> ImageResult<()>
    where
        P: AsRef<Path>,
    {
        self.data.save(png_file)
    }
}

impl<C> MonoFontData<C>
where
    C: Deref<Target = [u8]>,
{
    /// Mono font expects the image data to be in a BPP (1-Bit Per Pixel) format.
    /// This function will iterate collapse every 8 bytes (0 or 255) into a
    /// single byte, where each bit corresponds to a pixel's on/off state.
    ///
    /// See: https://docs.rs/embedded-graphics/0.7.1/embedded_graphics/image/struct.ImageRaw.html#draw-a-1bpp-image
    pub fn save_raw<P>(&self, raw_file: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let bpp_vec: Vec<u8> = self
            .data
            .chunks_exact(8)
            .map(|byte| {
                byte.iter()
                    .enumerate()
                    .filter(|(_, bit)| **bit > 0)
                    .map(|(i, _)| 0x80 >> i)
                    .sum()
            })
            .collect();

        fs::write(raw_file, &bpp_vec[..])
    }
}
