use iced::event::{self, Event};
use iced::keyboard;
use iced::keyboard::key;
use iced::widget::{
    self, button, center, column, container, horizontal_space, mouse_area,
    opaque, row, stack, text, svg,
};
use iced::{Bottom, Color, Element, Fill, Subscription, Task};

use crate::settings::{Settings, Theme};
use crate::storage::Storage;

pub fn gui() -> iced::Result {
    iced::application("Modal - Iced", App::update, App::view)
        .subscription(App::subscription)
        .run()
}

struct App {
    show_modal: bool,
    settings: Settings,
    storage: Storage,
}

impl Default for App {
    fn default() -> Self {
        Self {
            show_modal: false,
            settings: Settings::load(),
            storage: Storage::new().expect("Failed to initialize storage"),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    ShowModal,
    HideModal,
    ThemeSelected(Theme),
    Event(Event),
}

impl App {
    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ShowModal => {
                self.show_modal = true;
                Task::none()
            }
            Message::HideModal => {
                self.hide_modal();
                Task::none()
            }
            Message::ThemeSelected(theme) => {
                self.settings.theme = theme;
                if let Err(e) = self.settings.save() {
                    eprintln!("Failed to save settings: {}", e);
                }
                self.hide_modal();
                Task::none()
            }
            Message::Event(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                                    key: keyboard::Key::Named(key::Named::Tab),
                                    modifiers,
                                    ..
                                }) => {
                    if modifiers.shift() {
                        widget::focus_previous()
                    } else {
                        widget::focus_next()
                    }
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                                    key: keyboard::Key::Named(key::Named::Escape),
                                    ..
                                }) => {
                    self.hide_modal();
                    Task::none()
                }
                _ => Task::none(),
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let handle = svg::Handle::from_path("src/assets/icons/mdi--mixer-settings.svg");
        let content = container(
            column![
                row![
                    horizontal_space(),
                    button(svg(handle).width(20).height(20)).on_press(Message::ShowModal)
                ]
                .height(Fill),
                column(
                    self.storage.get_all().iter().map(|(_, value)| {
                        container(
                            row![
                                text(format!("{}", value))
                            ]
                            .spacing(10)
                        )
                        .style(container::rounded_box)
                        .padding(10)
                        .into()
                    }).collect::<Vec<_>>()
                )
                .spacing(10)
                .padding(10),
                row![
                    horizontal_space()
                ]
                .align_y(Bottom)
                .height(Fill),
            ]
                .height(Fill),
        )
            .padding(10);

        if self.show_modal {
            let theme_selection = container(
                column![
                    text("Select Theme").size(24),
                    row![
                        button(text("Dark")).on_press(Message::ThemeSelected(Theme::Dark)),
                        button(text("Light")).on_press(Message::ThemeSelected(Theme::Light)),
                        button(text("System")).on_press(Message::ThemeSelected(Theme::System)),
                    ]
                    .spacing(10)
                ]
                .spacing(20),
            )
                .width(300)
                .padding(10)
                .style(container::rounded_box);

            modal(content, theme_selection, Message::HideModal)
        } else {
            content.into()
        }
    }
}

impl App {
    fn hide_modal(&mut self) {
        self.show_modal = false;
    }
}

fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    ]
        .into()
}
