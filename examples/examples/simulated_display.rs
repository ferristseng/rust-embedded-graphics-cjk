use embedded_graphics::{
    mono_font::MonoTextStyle, pixelcolor::BinaryColor, prelude::*, text::Text,
};
use embedded_graphics_cjk_font_sarasa_gothic::SARASA_MONO_SC_LIGHT_24;
use embedded_graphics_cjk_font_zpix::{ZPIX_12, ZPIX_24};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(256, 80));

    Text::new(
        "大沼澤地國家公園",
        Point::new(0, 0),
        MonoTextStyle::new(&SARASA_MONO_SC_LIGHT_24, BinaryColor::On),
    )
    .draw(&mut display)?;
    Text::new(
        "大沼澤地國家公園",
        Point::new(0, 24),
        MonoTextStyle::new(&ZPIX_12, BinaryColor::On),
    )
    .draw(&mut display)?;
    Text::new(
        "新年快乐",
        Point::new(0, 48),
        MonoTextStyle::new(&ZPIX_24, BinaryColor::On),
    )
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    Window::new("Test", &output_settings).show_static(&display);

    Ok(())
}
