use crate::gui::Message;
use crate::storage::Storage;
use iced::widget::{column, container, row, scrollable, text};
use iced::{Element, Fill, Theme};

/// Creates a scrollable display of all received JSON payloads
pub fn payload_list<'a>(storage: &Storage) -> Element<'a, Message> {
    let payload_rows = column(
        storage
            .get_all()
            .iter()
            .map(|(_, value)| {
                container(row![text(format!("{}", value))])
                    .padding(10)
                    .width(Fill)
                    .style(|theme: &Theme| {
                        let palette = theme.extended_palette();
                        container::Style {
                            background: Some(palette.background.weak.color.into()),
                            border: iced_core::border::rounded(5),
                            ..container::Style::default()
                        }
                    })
                    .into()
            })
            .collect::<Vec<_>>(),
    )
    .spacing(10)
    .padding(10);

    scrollable(payload_rows)
        .width(Fill)
        .spacing(0)
        .height(Fill)
        .into()
}
