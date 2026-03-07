use iced::{Background, Border, Color, Theme, border::Radius, widget::text_input};

use crate::styles::PRIMARY;

pub fn rounded_input(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let palette = theme.palette();

    match status {
        text_input::Status::Active => text_input::Style {
            background: Background::Color(palette.background),
            border: Border {
                color: *PRIMARY,
                width: 1.0,
                radius: Radius::new(3.0),
            },
            icon: palette.primary,
            placeholder: palette.text,
            value: palette.text,
            selection: palette.background,
        },
        text_input::Status::Hovered => todo!(),
        text_input::Status::Focused { is_hovered } => todo!(),
        text_input::Status::Disabled => todo!(),
    }
}
