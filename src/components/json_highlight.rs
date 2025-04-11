use crate::gui::Message;
use iced::widget::{column, row, text};
use iced::{Color, Element, Theme};
use serde_json::Value;

fn color_for_json(value: &Value, theme: &Theme) -> Color {
    let palette = theme.extended_palette();
    match value {
        Value::String(_) => palette.primary.base.color,
        Value::Number(_) => palette.secondary.base.color,
        Value::Bool(_) => palette.primary.strong.color,
        Value::Null => palette.secondary.weak.color,
        _ => palette.primary.weak.color,
    }
}

fn render_json(value: &Value, theme: &Theme) -> Element<'static, Message> {
    let palette = theme.extended_palette();
    let structural_color = palette.background.strong.color;

    match value {
        Value::Object(map) => {
            let primary_color = palette.primary.base.color;
            column(
                map.iter()
                    .map(|(k, v)| {
                        let key = k.clone();
                        let color = primary_color;
                        row![
                            text("{").style(move |_| iced::widget::text::Style {
                                color: Some(structural_color)
                            }),
                            text(format!("\"{}\"", key))
                                .style(move |_| iced::widget::text::Style { color: Some(color) }),
                            text(":").style(move |_| iced::widget::text::Style {
                                color: Some(structural_color)
                            }),
                            render_json(v, theme),
                            text("}").style(move |_| iced::widget::text::Style {
                                color: Some(structural_color)
                            }),
                        ]
                        .spacing(5)
                        .into()
                    })
                    .collect::<Vec<_>>(),
            )
            .spacing(5)
            .into()
        }
        Value::Array(arr) => column(
            arr.iter()
                .map(|v| {
                    row![
                        text("[").style(move |_| iced::widget::text::Style {
                            color: Some(structural_color)
                        }),
                        render_json(v, theme),
                        text("]").style(move |_| iced::widget::text::Style {
                            color: Some(structural_color)
                        }),
                    ]
                    .spacing(5)
                    .into()
                })
                .collect::<Vec<_>>(),
        )
        .spacing(5)
        .into(),
        _ => {
            let val_str = value.to_string();
            let color = color_for_json(value, theme);
            text(val_str)
                .style(move |_| iced::widget::text::Style { color: Some(color) })
                .into()
        }
    }
}

pub fn highlight_json(json: &str, theme: &Theme) -> Element<'static, Message> {
    let parsed_json: Value = serde_json::from_str(json).unwrap_or(Value::Null);
    render_json(&parsed_json, theme)
}
