use iced::Color;
use iced_audio::{knob, text_marks, tick_marks, KnobAngleRange};

use super::text;

macro_rules! knob_style {
    ($name:ident, $base_style:ident, $range:expr, $text_size:expr, $text_width:expr, $text_offset:expr) => {
        pub struct $name;
        impl knob::StyleSheet for $name {
            fn active(&self) -> knob::Style {
                $base_style
            }

            fn hovered(&self) -> knob::Style {
                $base_style
            }

            fn dragging(&self) -> knob::Style {
                $base_style
            }

            fn angle_range(&self) -> KnobAngleRange {
                KnobAngleRange::from_deg(180.0 - ($range) / 2.0, 180.0 + ($range) / 2.0)
            }

            fn text_marks_style(&self) -> Option<knob::TextMarksStyle> {
                Some(knob::TextMarksStyle {
                    style: text_marks::Style {
                        text_size: $text_size,
                        bounds_width: $text_width,
                        ..Default::default()
                    },
                    v_offset: $text_offset,
                    h_char_offset: 2.0,
                    ..Default::default()
                })
            }

            fn tick_marks_style(&self) -> Option<knob::TickMarksStyle> {
                Some(knob::TickMarksStyle {
                    style: tick_marks::Style {
                        tier_2: tick_marks::Shape::Circle {
                            diameter: 2.0,
                            color: Color::from_rgb(0.5, 0.5, 0.5),
                        },
                        ..Default::default()
                    },
                    offset: 4.0,
                })
            }
        }
    };
}

const STYLE: knob::Style = knob::Style::Circle(knob::CircleStyle {
    color: Color::from_rgb(1.0, 1.0, 1.0),
    border_width: 1.0,
    border_color: Color::from_rgb(0.5, 0.5, 0.5),
    notch: knob::NotchShape::Line(knob::LineNotch {
        color: Color::from_rgb(0.5, 0.5, 0.5),
        width: knob::StyleLength::Scaled(0.05),
        length: knob::StyleLength::Scaled(0.33),
        offset: knob::StyleLength::Scaled(0.0),
        cap: knob::LineCap::Round,
    }),
});

knob_style! {BinaryKnobStyle, STYLE, 30.0, text::SIZE_MEDIUM, 16, -0.75}
knob_style! {OctalKnobStyle, STYLE, 240.0, text::SIZE_MEDIUM, 16, -0.75}

knob_style! {OrderTypeKnobStyle, STYLE, 300.0, text::SIZE_MEDIUM, 16, -0.75}
knob_style! {ExcessMagnitudeKnobStyle, STYLE, 120.0, text::SIZE_SMALL, 48, -0.75}

knob_style! {OperatingModeKnobStyle, STYLE, 180.0, text::SIZE_SMALL, 64, 3.0}
knob_style! {MemoryModeKnobStyle, STYLE, 60.0, text::SIZE_SMALL, 16, -0.75}
