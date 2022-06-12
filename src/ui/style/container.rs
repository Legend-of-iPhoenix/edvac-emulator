use iced::{container, Color};
pub struct ContainerStyle;
impl container::StyleSheet for ContainerStyle {
    fn style(&self) -> container::Style {
        container::Style {
            border_width: 1.0,
            border_color: Color::from_rgb(0.0, 0.0, 0.0),
            ..Default::default()
        }
    }
}
