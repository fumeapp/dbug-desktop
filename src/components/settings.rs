use crate::gui::Message;
use iced::widget::{column, container, radio, scrollable, text};
use iced::{Element, Fill, Theme};

/// Creates the settings modal content with theme selection
pub fn settings_modal<'a>(current_theme: Theme) -> Element<'a, Message> {
    // Find the current theme index in Theme::ALL
    let current_index = Theme::ALL
        .iter()
        .position(|t| t.to_string() == current_theme.to_string())
        .unwrap_or(0);

    container(
        column![
            text("Select Theme").size(18).style(move |_theme| {
                text::Style {
                    color: current_theme.palette().text.into(),
                }
            }),
            scrollable(
                container(
                    column(
                        Theme::ALL
                            .iter()
                            .enumerate()
                            .map(|(idx, theme)| {
                                container(
                                    radio(
                                        theme.to_string(),
                                        idx,
                                        Some(current_index),
                                        Message::ThemeChanged,
                                    )
                                    .width(Fill)
                                    .style(|_, status| radio::Style {
                                        border_color: theme
                                            .extended_palette()
                                            .background
                                            .strong
                                            .color,
                                        text_color: theme.palette().text.into(),
                                        ..radio::default(theme, status)
                                    })
                                    .spacing(10),
                                )
                                .width(Fill)
                                .padding(10)
                                .style(move |_| container::Style {
                                    background: Some(
                                        theme.extended_palette().background.weak.color.into(),
                                    ),
                                    border: iced_core::border::rounded(5),
                                    ..container::Style::default()
                                })
                                .into()
                            })
                            .collect::<Vec<Element<Message>>>()
                    )
                    .spacing(10)
                )
                .padding(iced_core::Padding {
                    right: 15.0,
                    top: 5.0,
                    bottom: 5.0,
                    ..iced_core::Padding::default()
                })
            )
        ]
        .spacing(10),
    )
    .width(360)
    .height(400)
    .padding(10)
    .style(|theme| container::Style {
        background: Some(theme.extended_palette().background.base.color.into()),
        border: iced_core::border::rounded(5),
        ..container::Style::default()
    })
    .into()
}
