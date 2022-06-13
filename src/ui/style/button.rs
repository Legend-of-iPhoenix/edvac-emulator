use iced::{button, Background, Color};

pub struct ButtonStyle;
impl button::StyleSheet for ButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb(1.0, 1.0, 1.0))),

            border_color: Color::from_rgb(0.0, 0.0, 0.0),
            border_width: 1.0,
            border_radius: 100.0,

            ..button::Style::default()
        }
    }
}
