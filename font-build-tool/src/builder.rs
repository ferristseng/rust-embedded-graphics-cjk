use crate::{error::BuildError, unicode::UnicodeCodeBlock};
use freetype::{face::LoadFlag, Face, Library};
use image::{EncodableLayout, ImageBuffer, ImageResult, Luma, PixelWithColorType};
use std::{cmp::max, ffi::OsStr, fmt::Display, fs, io, ops::Deref, path::Path};

/// The number of glyphs to include on a single line in the final bitmap.
const ROW_SIZE: usize = 32;

pub struct FontOutputSettings {
    /// The target font size.
    pub font_size: u32,

    /// Threshold for when to select a pixel when adding it to the bitmap.
    /// A reasonable value for this would be 128, meaning anything above 50%
    /// intensity will appear in the final bitmap (max threshold = 255).
    pub intensity_threshold: u8,
}

pub struct MonoFontBuilder<'a> {
    _lib: Library,

    font: Face,

    /// The unicode character blocks to generate bitmaps for.
    unicode_blocks: &'a [UnicodeCodeBlock],
}

impl<'a> MonoFontBuilder<'a> {
    pub fn new<P>(
        ttf_path: P,
        unicode_blocks: &'a [UnicodeCodeBlock],
    ) -> Result<MonoFontBuilder<'a>, BuildError>
    where
        P: AsRef<OsStr>,
    {
        let lib = Library::init().unwrap();
        let font = lib.new_face(ttf_path, 0).unwrap();

        Ok(MonoFontBuilder {
            _lib: lib,
            font,
            unicode_blocks,
        })
    }

    /// Returns an iterator over characters in each unicode code block.
    fn chars_iter(&self) -> impl Iterator<Item = char> + 'a {
        self.unicode_blocks.iter().flat_map(|block| block.range())
    }

    /// Number of characters covered by all code blocks.
    fn num_chars(&self) -> usize {
        self.unicode_blocks
            .iter()
            .map(UnicodeCodeBlock::block_size)
            .sum()
    }

    /// Renders glyphs for each of the selected fonts, then stores it in a
    /// bitmap that can be exported as PNG, BBP, or source code compatible with
    /// the embedded-graphics library.
    pub fn build(
        &self,
        settings: FontOutputSettings,
    ) -> Result<MonoFontData<ImageBuffer<Luma<u8>, Vec<u8>>>, BuildError> {
        println!(
            "font.family_name={:?} font.style_name={:?}",
            self.font.family_name(),
            self.font.style_name()
        );

        self.font
            .set_pixel_sizes(0, settings.font_size as u32)
            .unwrap();

        // Determines the maximum glyph height and glyph width based on the
        // glyph metrics for each chosen character.
        let (max_glyph_height, max_glyph_width) = self
            .chars_iter()
            .map(|chr| {
                self.font.load_char(chr as usize, LoadFlag::RENDER).unwrap();

                let glyph = self.font.glyph();
                let metrics = glyph.metrics();
                let bitmap = glyph.bitmap();

                (
                    std::cmp::max(bitmap.rows() as usize, metrics.vertAdvance as usize / 64)
                        .checked_add_signed(bitmap.rows() as isize - glyph.bitmap_top() as isize)
                        .unwrap(),
                    metrics.horiAdvance as usize / 64,
                )
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

        println!(
            "max_glyph_height={} max_glyph_width={}",
            max_glyph_height, max_glyph_width
        );

        // Image buffer that contains every glyph specified in rows of ROW_SIZE.
        let mut imgbuf = image::GrayImage::new(
            (max_glyph_width * ROW_SIZE) as u32,
            (max_glyph_height * ((self.num_chars() - 1) / ROW_SIZE + 1)) as u32,
        );

        // Rasterizes the font, and copies the bitmap onto the image buffer.
        for (index, chr) in self.chars_iter().enumerate() {
            self.font.load_char(chr as usize, LoadFlag::RENDER).unwrap();

            let glyph = self.font.glyph();
            let metrics = glyph.metrics();
            let bitmap = glyph.bitmap();

            /*
            println!(
                "[{: >8}: {}] bitmap.width = {: >3} \
                bitmap.rows = {: >3} \
                glyph.left = {: >3} \
                glyph.top = {: >3} \
                glyph.v_advance = {: >3} \
                glyph.h_advance = {: >3} \
                bitmap.len = {: >6} \
                bitmap.pixel_mode = {:?}",
                index,
                chr,
                bitmap.width(),
                bitmap.rows(),
                glyph.bitmap_left(),
                glyph.bitmap_top(),
                metrics.vertAdvance as usize / 64,
                metrics.horiAdvance as usize / 64,
                bitmap.buffer().len(),
                bitmap.pixel_mode().unwrap()
            );
            */

            let col = index % ROW_SIZE;
            let row = index / ROW_SIZE;
            let img_x = col as isize * max_glyph_width as isize;
            let img_y = row as isize * max_glyph_height as isize;
            let img_x_offset = glyph.bitmap_left() as usize;
            let img_y_offset = (max_glyph_height - glyph.bitmap_top() as usize) / 2;
            let cols = bitmap.width() as usize;

            // Copy onto image
            for y in 0..bitmap.rows() as usize {
                for x in 0..bitmap.width() as usize {
                    let val = bitmap.buffer()[y * cols + x];

                    if val > settings.intensity_threshold {
                        let pixel_x = img_x + x as isize + img_x_offset as isize;
                        let pixel_y = img_y + y as isize + img_y_offset as isize;
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
r#"// This is generated code. Any modifications to this file will
// be overwritten.
use embedded_graphics::{{
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
}};
"#,
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
