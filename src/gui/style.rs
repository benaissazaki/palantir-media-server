use iced::{
    font,
    theme::{self, Palette},
    widget::{button, container, scrollable, text_input},
    Background, Color, Font, Theme, Vector,
};

/// Creates an opaque color from 8-bit RGB components
const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    }
}

pub const BACKGROUND: Color = rgb(0x0f, 0x11, 0x15);
pub const SURFACE: Color = rgb(0x17, 0x1a, 0x21);
pub const BORDER: Color = rgb(0x26, 0x2a, 0x33);
pub const TEXT: Color = rgb(0xe7, 0xe9, 0xee);
pub const MUTED: Color = rgb(0x8a, 0x90, 0x9c);
pub const ACCENT: Color = rgb(0x7c, 0x8c, 0xff);
pub const SUCCESS: Color = rgb(0x4a, 0xde, 0x80);
pub const DANGER: Color = rgb(0xf8, 0x71, 0x71);

const RADIUS: f32 = 10.0;

pub const BOLD: Font = Font {
    weight: font::Weight::Bold,
    ..Font::DEFAULT
};

pub fn theme() -> Theme {
    Theme::custom(Palette {
        background: BACKGROUND,
        text: TEXT,
        primary: ACCENT,
        success: SUCCESS,
        danger: DANGER,
    })
}

/// Lightens (positive amount) or darkens (negative amount) a color
fn shade(color: Color, amount: f32) -> Color {
    Color {
        r: (color.r + amount).clamp(0.0, 1.0),
        g: (color.g + amount).clamp(0.0, 1.0),
        b: (color.b + amount).clamp(0.0, 1.0),
        a: color.a,
    }
}

pub fn card(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        text_color: Some(TEXT),
        background: Some(Background::Color(SURFACE)),
        border_radius: RADIUS.into(),
        border_width: 1.0,
        border_color: BORDER,
    }
}

pub fn list_item(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        text_color: Some(TEXT),
        background: Some(Background::Color(BACKGROUND)),
        border_radius: 8.0.into(),
        border_width: 1.0,
        border_color: BORDER,
    }
}

/// A small colored dot used as a status indicator
pub fn dot(color: Color) -> impl Fn(&Theme) -> container::Appearance {
    move |_theme| container::Appearance {
        background: Some(Background::Color(color)),
        border_radius: 4.0.into(),
        ..Default::default()
    }
}

pub enum Button {
    Primary,
    Danger,
    Secondary,
    Link,
}

impl Button {
    fn colors(&self) -> (Option<Color>, Color, Color) {
        // (background, text, border)
        match self {
            Button::Primary => (Some(ACCENT), BACKGROUND, ACCENT),
            Button::Danger => (Some(DANGER), BACKGROUND, DANGER),
            Button::Secondary => (Some(SURFACE), TEXT, BORDER),
            Button::Link => (None, MUTED, Color::TRANSPARENT),
        }
    }
}

impl button::StyleSheet for Button {
    type Style = Theme;

    fn active(&self, _style: &Theme) -> button::Appearance {
        let (background, text_color, border_color) = self.colors();
        button::Appearance {
            shadow_offset: Vector::default(),
            background: background.map(Background::Color),
            border_radius: 8.0.into(),
            border_width: 1.0,
            border_color,
            text_color,
        }
    }

    fn hovered(&self, style: &Theme) -> button::Appearance {
        let active = self.active(style);
        match self {
            Button::Link => button::Appearance {
                text_color: DANGER,
                ..active
            },
            Button::Secondary => button::Appearance {
                border_color: shade(BORDER, 0.12),
                background: Some(Background::Color(shade(SURFACE, 0.04))),
                ..active
            },
            _ => {
                let (background, _, _) = self.colors();
                button::Appearance {
                    background: background.map(|c| Background::Color(shade(c, 0.06))),
                    ..active
                }
            }
        }
    }

    fn pressed(&self, style: &Theme) -> button::Appearance {
        self.active(style)
    }
}

pub fn button_style(style: Button) -> theme::Button {
    theme::Button::custom(style)
}

pub struct TextInput;

impl text_input::StyleSheet for TextInput {
    type Style = Theme;

    fn active(&self, _style: &Theme) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(BACKGROUND),
            border_radius: 8.0.into(),
            border_width: 1.0,
            border_color: BORDER,
            icon_color: MUTED,
        }
    }

    fn focused(&self, style: &Theme) -> text_input::Appearance {
        text_input::Appearance {
            border_color: ACCENT,
            ..self.active(style)
        }
    }

    fn hovered(&self, style: &Theme) -> text_input::Appearance {
        text_input::Appearance {
            border_color: shade(BORDER, 0.12),
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Theme) -> text_input::Appearance {
        self.active(style)
    }

    fn placeholder_color(&self, _style: &Theme) -> Color {
        shade(MUTED, -0.2)
    }

    fn value_color(&self, _style: &Theme) -> Color {
        TEXT
    }

    fn disabled_color(&self, _style: &Theme) -> Color {
        MUTED
    }

    fn selection_color(&self, _style: &Theme) -> Color {
        Color { a: 0.35, ..ACCENT }
    }
}

pub fn text_input_style() -> theme::TextInput {
    theme::TextInput::Custom(Box::new(TextInput))
}

pub struct Scrollable;

impl scrollable::StyleSheet for Scrollable {
    type Style = Theme;

    fn active(&self, _style: &Theme) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: None,
            border_radius: 4.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: BORDER,
                border_radius: 4.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, style: &Theme, is_mouse_over_scrollbar: bool) -> scrollable::Scrollbar {
        let active = self.active(style);
        match is_mouse_over_scrollbar {
            true => scrollable::Scrollbar {
                scroller: scrollable::Scroller {
                    color: MUTED,
                    ..active.scroller
                },
                ..active
            },
            false => active,
        }
    }
}

pub fn scrollable_style() -> theme::Scrollable {
    theme::Scrollable::custom(Scrollable)
}
