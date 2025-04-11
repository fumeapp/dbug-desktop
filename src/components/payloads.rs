use crate::components::json_highlight::highlight_json;
use crate::gui::Message;
use crate::storage::Storage;
use iced::widget::{button, column, container, scrollable, text};
use iced::{Element, Fill, Theme};

/// Creates a scrollable display of all received JSON payloads
pub fn payload_list<'a>(
    storage: &Storage,
    expanded_id: Option<&String>,
    theme: &Theme,
) -> Element<'a, Message> {
    let storage_rows = column(
        storage
            .get_all()
            .iter()
            .map(|(id, value)| {
                let is_expanded = expanded_id == Some(id);

                if is_expanded {
                    // Pretty print the JSON with proper indentation
                    let pretty_json = match serde_json::to_string_pretty(value) {
                        Ok(formatted) => formatted,
                        Err(_) => format!("{:?}", value),
                    };

                    // Use syntax highlighting for JSON with the current theme
                    let highlighted_json = highlight_json(&pretty_json, theme);

                    // For expanded items, use a container with similar styling but not a button
                    container(column![highlighted_json].spacing(5).width(Fill))
                        .padding(10)
                        .width(Fill)
                        .style(|theme: &Theme| {
                            let palette = theme.extended_palette();
                            container::Style {
                                background: Some(palette.background.base.color.into()),
                                border: iced_core::border::rounded(5)
                                    .color(palette.secondary.base.color)
                                    .width(1.0),
                                ..container::Style::default()
                            }
                        })
                        .into()
                } else {
                    // For non-expanded items, use a button with secondary styling
                    let content = text(format!("{}", value));

                    button(container(content).width(Fill))
                        .width(Fill)
                        .style(button::secondary)
                        .on_press(Message::TogglePayload(id.clone()))
                        .into()
                }
            })
            .collect::<Vec<_>>(),
    )
    .spacing(10)
    .padding(10);

    scrollable(container(storage_rows).padding(iced_core::Padding {
        right: 5.0,
        ..Default::default()
    }))
    .width(Fill)
    .height(Fill)
    .into()
}
