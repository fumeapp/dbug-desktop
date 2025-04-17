use crate::components::json_highlight::highlight_json;
use crate::components::styles;
use crate::gui::Message;
use crate::storage::Storage;
use chrono::{DateTime, Utc};
use core::time::Duration;
use iced::widget::{button, column, container, row, scrollable, stack, svg, text};
use iced::{Element, Fill, Theme};
use millisecond::prelude::*;
use std::collections::HashSet;

/// Converts a timestamp ID into a human-readable relative time string
fn human_readable_time(id: &str) -> String {

    id.parse::<i64>()
        .ok()
        .and_then(DateTime::<Utc>::from_timestamp_millis)
        .map(|time| Utc::now().signed_duration_since(time)).map_or_else(|| "Invalid timestamp".to_string(), |duration| Duration::from_millis(duration.num_milliseconds() as u64).relative())

}

/// Creates a scrollable display of all received JSON payloads
pub fn payload_list<'a>(
    storage: &Storage,
    expanded_id: Option<&String>,
    theme: &Theme,
    collapsed_json_lines: &HashSet<usize>,
) -> Element<'a, Message> {
    let storage_rows = column(
        storage
            .get_all()
            .iter()
            .map(|(id, value)| {
                let is_expanded = expanded_id == Some(id);
                let timestamp = human_readable_time(id);

                if is_expanded {
                    // Pretty print the JSON with proper indentation
                    let pretty_json = serde_json::to_string_pretty(value).unwrap_or_else(|_| format!("{value:?}"));

                    // Use syntax highlighting for JSON with the current theme
                    let highlighted_json = highlight_json(
                        &pretty_json,
                        theme,
                        collapsed_json_lines,
                    );

                    let close_svg = svg(svg::Handle::from_memory(
                        include_bytes!("../../assets/icons/mdi--caret-down.svg").as_slice(),
                    ))
                    .width(Fill)
                    .height(Fill)
                    .style(styles::svg_style_secondary);

                    let delete_svg = svg(svg::Handle::from_memory(
                        include_bytes!("../../assets/icons/mdi--trash-can.svg").as_slice(),
                    ))
                    .width(Fill)
                    .height(Fill)
                    .style(styles::svg_style_primary);

                    // For expanded items, use a container with similar styling but not a button
                    container(
                        stack![
                            highlighted_json,
                            container(row![
                                container(text(timestamp).size(10.0))
                                    .padding(3.0)
                                    .align_x(iced::alignment::Horizontal::Right)
                                    .align_y(iced::alignment::Vertical::Bottom)
                                    .width(Fill),
                                button(delete_svg)
                                    .style(button::danger)
                                    .width(18)
                                    .height(18)
                                    .padding(1)
                                    .on_press(Message::DeletePayload(id.clone())),
                                button(close_svg)
                                    .width(18)
                                    .height(18)
                                    .padding(0)
                                    .on_press(Message::TogglePayload(id.clone()))
                            ].spacing(5))
                            .align_top(Fill)
                            .align_right(Fill)
                            .width(Fill),
                        ]
                        .width(Fill),
                    )
                    .padding(10)
                    .width(Fill)
                    .style(styles::container_code)
                    .into()
                } else {
                    // Create SVGs for buttons in collapsed view
                    let expand_svg = svg(svg::Handle::from_memory(
                        include_bytes!("../../assets/icons/mdi--caret-up.svg").as_slice(), // Use caret-right for expand
                    ))
                    .width(Fill)
                    .height(Fill)
                    .style(styles::svg_style_secondary);

                    let delete_svg = svg(svg::Handle::from_memory(
                        include_bytes!("../../assets/icons/mdi--trash-can.svg").as_slice(),
                    ))
                    .width(Fill)
                    .height(Fill)
                    .style(styles::svg_style_primary); // Use primary style for delete

                    // Use a container to hold the collapsed view elements
                    // Wrap the container in a button to make the whole row clickable
                    button(
                        container(
                            row![
                                // Payload preview (limited height)
                                container(text(format!("{value}")).size(14).height(18.0)).width(Fill),
                                // Timestamp
                                container(text(timestamp).size(10.0))
                                    .padding(4.0)
                                    .align_x(iced::alignment::Horizontal::Right)
                                    .align_y(iced::alignment::Vertical::Center),
                                // Delete button (remains clickable)
                                button(delete_svg)
                                    .style(button::danger)
                                    .width(18)
                                    .height(18)
                                    .padding(1)
                                    .on_press(Message::DeletePayload(id.clone())),
                                // Expand button icon (now just visual)
                                // The actual expand action is on the parent button
                                container(expand_svg) // Keep the icon visually
                                    .width(18)
                                    .height(18)
                                    .padding(0)
                            ]
                            .spacing(5)
                        )
                        .padding(10)
                        .width(Fill)
                        .style(styles::container_code_closed)
                    )
                    .style(button::text) // Make the wrapper button invisible
                    .width(Fill) // Ensure the button fills the width
                    .on_press(Message::TogglePayload(id.clone())) // Attach toggle action here
                    .padding(0)
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
