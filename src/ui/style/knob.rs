use iced::Color;
use iced_audio::{knob, text_marks, tick_marks, KnobAngleRange};

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

pub struct BinaryKnobStyle;
impl knob::StyleSheet for BinaryKnobStyle {
    fn active(&self) -> knob::Style {
        STYLE
    }

    fn hovered(&self) -> knob::Style {
        STYLE
    }

    fn dragging(&self) -> knob::Style {
        STYLE
    }

    fn angle_range(&self) -> KnobAngleRange {
        KnobAngleRange::from_deg(180.0 - 240.0 / 16.0, 180.0 + 240.0 / 16.0)
    }

    fn text_marks_style(&self) -> Option<knob::TextMarksStyle> {
        Some(knob::TextMarksStyle::default())
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
            offset: 5.0,
        })
    }
}

pub struct OctalKnobStyle;
impl knob::StyleSheet for OctalKnobStyle {
    fn active(&self) -> knob::Style {
        STYLE
    }

    fn hovered(&self) -> knob::Style {
        STYLE
    }

    fn dragging(&self) -> knob::Style {
        STYLE
    }

    fn angle_range(&self) -> KnobAngleRange {
        KnobAngleRange::from_deg(60.0, 300.0)
    }

    fn text_marks_style(&self) -> Option<knob::TextMarksStyle> {
        Some(knob::TextMarksStyle::default())
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
            offset: 5.0,
        })
    }
}

pub struct OrderTypeKnobStyle;
impl knob::StyleSheet for OrderTypeKnobStyle {
    fn active(&self) -> knob::Style {
        STYLE
    }

    fn hovered(&self) -> knob::Style {
        STYLE
    }

    fn dragging(&self) -> knob::Style {
        STYLE
    }

    fn text_marks_style(&self) -> Option<knob::TextMarksStyle> {
        Some(knob::TextMarksStyle::default())
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
            offset: 5.0,
        })
    }
}

pub struct ExcessMagnitudeKnobStyle;
impl knob::StyleSheet for ExcessMagnitudeKnobStyle {
    fn active(&self) -> knob::Style {
        STYLE
    }

    fn hovered(&self) -> knob::Style {
        STYLE
    }

    fn dragging(&self) -> knob::Style {
        STYLE
    }

    fn angle_range(&self) -> KnobAngleRange {
        KnobAngleRange::from_deg(180.0 - 240.0 / 4.0, 180.0 + 240.0 / 4.0)
    }

    fn text_marks_style(&self) -> Option<knob::TextMarksStyle> {
        Some(knob::TextMarksStyle {
            style: text_marks::Style {
                text_size: 8,
                bounds_width: 48,
                ..Default::default()
            },
            h_char_offset: 1.5,
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
            offset: 5.0,
        })
    }
}

pub struct OperatingModeKnobStyle;
impl knob::StyleSheet for OperatingModeKnobStyle {
    fn active(&self) -> knob::Style {
        STYLE
    }

    fn hovered(&self) -> knob::Style {
        STYLE
    }

    fn dragging(&self) -> knob::Style {
        STYLE
    }

    fn angle_range(&self) -> KnobAngleRange {
        KnobAngleRange::from_deg(180.0 - 240.0 / 4.0, 180.0 + 240.0 / 4.0)
    }

    fn text_marks_style(&self) -> Option<knob::TextMarksStyle> {
        Some(knob::TextMarksStyle {
            style: text_marks::Style {
                text_size: 8,
                bounds_width: 48,
                ..Default::default()
            },
            h_char_offset: 1.5,
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
            offset: 5.0,
        })
    }
}
