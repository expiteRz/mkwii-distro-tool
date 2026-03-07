use iced::Color;
use once_cell::sync::Lazy;

pub mod text_input;

pub static PRIMARY: Lazy<Color> = Lazy::new(|| Color::from_rgb8(255, 255, 255));
