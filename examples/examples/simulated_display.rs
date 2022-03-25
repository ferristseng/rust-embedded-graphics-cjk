use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Text},
};
use embedded_graphics_cjk_font_noto::NOTO_SANS_MONO_CJK_SC_REGULAR_36;
use embedded_graphics_cjk_font_sarasa_gothic::SARASA_MONO_SC_LIGHT_36;
use embedded_graphics_cjk_font_zpix::ZPIX_24;
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(512, 128));

    Text::new(
        "大沼澤地國家公園",
        Point::new(0, 0),
        MonoTextStyle::new(&SARASA_MONO_SC_LIGHT_36, BinaryColor::On),
    )
    .draw(&mut display)?;
    Text::new(
        "大沼澤地國家公園",
        Point::new(0, 36),
        MonoTextStyle::new(&NOTO_SANS_MONO_CJK_SC_REGULAR_36, BinaryColor::On),
    )
    .draw(&mut display)?;
    Text::with_alignment(
        "新年快乐",
        Point::new(0, 72),
        MonoTextStyle::new(&ZPIX_24, BinaryColor::On),
        Alignment::Left,
    )
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .scale(1)
        .build();
    Window::new("Test", &output_settings).show_static(&display);

    Ok(())
}
