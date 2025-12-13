use std::sync::Arc;

use iced::theme::Custom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub toggle_mod: Option<String>,
    pub toggle_key: Option<String>,
    pub buffer_rules: Option<Buffer>,
    pub theme: Option<Theme>,
    pub placeholder: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            toggle_mod: Some("ALT".to_string()),
            toggle_key: Some("Space".to_string()),
            buffer_rules: Some(Buffer::default()),
            theme: Some(Theme::default()),
            placeholder: Some(String::from("Time to be productive!")),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Theme {
    pub text_color: Option<(f32, f32, f32)>,
    pub background_color: Option<(f32, f32, f32)>,
    pub background_opacity: Option<f32>,
    pub blur: Option<bool>,
    pub show_icons: Option<bool>,
    pub show_scroll_bar: Option<bool>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            text_color: Some((0.95, 0.95, 0.96)),
            background_color: Some((0.11, 0.11, 0.13)),
            background_opacity: Some(1.),
            blur: Some(false),
            show_icons: Some(true),
            show_scroll_bar: Some(true),
        }
    }
}

impl Theme {
    pub fn to_iced_theme(&self) -> iced::Theme {
        let default = Self::default();
        let text_color = self.text_color.unwrap_or(default.text_color.unwrap());
        let bg_color = self
            .background_color
            .unwrap_or(default.background_color.unwrap());
        let palette = iced::theme::Palette {
            background: iced::Color {
                r: bg_color.0,
                g: bg_color.1,
                b: bg_color.2,
                a: self.background_opacity.unwrap_or(1.),
            },
            text: iced::Color {
                r: text_color.0,
                g: text_color.1,
                b: text_color.2,
                a: 1.,
            },
            primary: iced::Color {
                r: 0.22,
                g: 0.55,
                b: 0.96,
                a: 1.0,
            },
            danger: iced::Color {
                r: 0.95,
                g: 0.26,
                b: 0.21,
                a: 1.0,
            },
            warning: iced::Color {
                r: 1.0,
                g: 0.76,
                b: 0.03,
                a: 1.0,
            },
            success: iced::Color {
                r: 0.30,
                g: 0.69,
                b: 0.31,
                a: 1.0,
            },
        };
        iced::Theme::Custom(Arc::new(Custom::new("RustCast Theme".to_string(), palette)))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Buffer {
    pub clear_on_hide: Option<bool>,
    pub clear_on_enter: Option<bool>,
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            clear_on_hide: Some(true),
            clear_on_enter: Some(true),
        }
    }
}
