use embedded_graphics::{
    mono_font::MonoTextStyle, pixelcolor::BinaryColor, prelude::*, text::Text,
};
use embedded_graphics_cjk_font_fusion_pixel::{FUSION_PIXEL_12, FUSION_PIXEL_24};
use embedded_graphics_cjk_font_noto::NOTO_SANS_MONO_CJK_SC_REGULAR_36;
use embedded_graphics_cjk_font_sarasa_gothic::SARASA_MONO_SC_LIGHT_36;
use embedded_graphics_cjk_font_zpix::{ZPIX_12, ZPIX_24};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

const TEXT: &'static str = "大沼澤地國家公園";

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(512, 128));

    let width_multiplier = TEXT.chars().count() as i32 + 1;
    let lines_to_draw = &[
        Text::new(
            TEXT,
            Point::new(0, 0),
            MonoTextStyle::new(&SARASA_MONO_SC_LIGHT_36, BinaryColor::On),
        ),
        Text::new(
            TEXT,
            Point::new(0, 36),
            MonoTextStyle::new(&NOTO_SANS_MONO_CJK_SC_REGULAR_36, BinaryColor::On),
        ),
        Text::new(
            TEXT,
            Point::new(0, 72),
            MonoTextStyle::new(&FUSION_PIXEL_24, BinaryColor::On),
        ),
        Text::new(
            TEXT,
            Point::new(24 * width_multiplier, 72),
            MonoTextStyle::new(&ZPIX_24, BinaryColor::On),
        ),
        Text::new(
            TEXT,
            Point::new(0, 96),
            MonoTextStyle::new(&FUSION_PIXEL_12, BinaryColor::On),
        ),
        Text::new(
            TEXT,
            Point::new(12 * width_multiplier, 96),
            MonoTextStyle::new(&ZPIX_12, BinaryColor::On),
        ),
    ];

    for line in lines_to_draw {
        line.draw(&mut display)?;
    }

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    Window::new("Test", &output_settings).show_static(&display);

    Ok(())
}
