mod gui;
mod vlsm;

use gui::*;
use iced::Sandbox;
use iced::Settings;

fn main() -> iced::Result {
    Calculator::run(Settings::default())?;
    Ok(())
}
