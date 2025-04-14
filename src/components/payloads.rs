use crate::components::custom_highlighter::{Highlight, Highlighter, Settings};
use crate::components::styles;
use crate::gui::Message;
use crate::storage::Storage;
use chrono::{DateTime, Utc};
use core::time::Duration;
use iced::widget::{button, column, container, row, scrollable, stack, svg, text, text_editor, Column};
use iced::{Element, Fill, Theme};
use millisecond::prelude::*;

/// Converts a timestamp ID into a human-readable relative time string
fn human_readable_time(id: &str) -> String {
    id.parse::<i64>()
        .ok()
        .and_then(DateTime::<Utc>::from_timestamp_millis)
        .map(|time| Utc::now().signed_duration_since(time))
        .map_or_else(
            || "Invalid timestamp".to_string(),
            |duration| Duration::from_millis(duration.num_milliseconds() as u64).relative(),
        )
}

pub fn highlight_json<'a>(
    content: &'a text_editor::Content,
    _theme: &Theme,
) -> Element<'a, Message> {
    Column::new()
        .push(text_editor(content).highlight_with::<Highlighter>(
            Settings::new(vec![], Highlight::default_style, "json"),
            Highlight::to_format,
        ))
        .into()
}

/// Creates a scrollable display of all received JSON payloads
pub fn payload_list<'a>(
    storage: &Storage,
    expanded_id: Option<&String>,
    theme: &Theme,
    content: &'a text_editor::Content,
) -> Element<'a, Message> {
    let all_payloads = storage.get_all(); // Cache storage results

    let storage_rows = column(
        all_payloads
            .iter()
            .map(|(id, value)| {
                let is_expanded = expanded_id == Some(id);
                let timestamp = human_readable_time(id);

                if is_expanded {
                    let close_svg = svg(svg::Handle::from_memory(
                        include_bytes!("../../assets/icons/mdi--close.svg").as_slice(),
                    ))
                    .width(Fill)
                    .height(Fill)
                    .style(styles::svg_style_secondary);

                    let delete_svg = svg(svg::Handle::from_memory(
                        include_bytes!("../../assets/icons/mdi--trash-can.svg").as_slice(),
                    ))
                    .width(Fill)
                    .height(Fill)
                    .style(styles::svg_style_danger);

                    // For expanded items, use a container with similar styling but not a button
                    container(
                        stack![
                            highlight_json(content, theme),
                            container(row![
                                container(text(timestamp).size(10.0))
                                    .padding(3.0)
                                    .align_x(iced::alignment::Horizontal::Right)
                                    .align_y(iced::alignment::Vertical::Bottom)
                                    .width(Fill),
                                button(delete_svg)
                                    .style(button::text)
                                    .width(20)
                                    .height(20)
                                    .padding(2.0)
                                    .on_press(Message::DeletePayload(id.clone())),
                                button(close_svg)
                                    .style(button::text)
                                    .width(20)
                                    .height(20)
                                    .padding(2.0)
                                    .on_press(Message::TogglePayload(id.clone()))
                            ])
                            .align_top(Fill)
                            .align_right(Fill)
                            .width(Fill),
                        ]
                        .width(Fill),
                    )
                    .padding(10)
                    .width(Fill)
                    .style(|theme: &Theme| {
                        let palette = theme.extended_palette();
                        let mut bg_color = palette.secondary.strong.color;
                        bg_color.a = 0.05;
                        let mut border_color = palette.secondary.strong.color;
                        border_color.a = 0.1;

                        container::Style {
                            background: Some(bg_color.into()),
                            border: iced_core::border::rounded(5).color(border_color).width(1.0),
                            ..container::Style::default()
                        }
                    })
                    .into()
                } else {
                    button(
                        stack![
                            text(format!("{value}")).height(22.0),
                            container(text(timestamp).size(10.0))
                                .padding(4.0)
                                .align_x(iced::alignment::Horizontal::Right)
                                .align_y(iced::alignment::Vertical::Center)
                                .width(Fill)
                        ]
                        .width(Fill),
                    )
                    .style(button::secondary)
                    .width(Fill)
                    .on_press(Message::TogglePayload(id.clone()))
                    .into()
                }
            })
            .collect::<Vec<_>>(),
    )
    .spacing(10)
    .padding(iced_core::Padding {
        right: 5.0,
        left: 5.0,
        top: 1.0,
        bottom: 0.0,
    });

    scrollable(container(storage_rows).padding(iced_core::Padding {
        right: 5.0,
        ..Default::default()
    }))
    .direction(scrollable::Direction::Vertical(
        scrollable::Scrollbar::new().width(5).scroller_width(5),
    ))
    .id(iced::widget::scrollable::Id::new("payload_scroll"))
    .width(Fill)
    .height(Fill)
    .into()
}
