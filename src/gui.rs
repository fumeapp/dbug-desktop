use iced::event::Event;
use iced::keyboard::key;
use iced::{keyboard, Length, Theme};

use crate::components;
use crate::gui::Message::Server;
use crate::server;
use crate::server::ServerMessage;
use crate::settings::Settings;
use crate::storage::Storage;
use iced::widget::{self, button, column, container, horizontal_space, row, svg};
use iced::{Bottom, Element, Fill, Subscription, Task};

/// Initializes and runs the GUI application
pub fn gui() -> iced::Result {
    iced::application("dbug desktop", App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .run()
}

/// Application state and logic
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

/// Messages used for application state updates
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) enum Message {
    ShowModal,
    HideModal,
    Event(Event),
    Server(ServerMessage),
    ThemeChanged(usize),
}

impl App {
    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(server::listen).map(Server)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Server(server_message) => {
                println!("{:?}", server_message);
                match server_message {
                    ServerMessage::PayloadReceived(value) => {
                        if let Err(e) = self.storage.add_json(&value) {
                            eprintln!("Failed to store payload: {}", e);
                        }
                    }
                }
                Task::none()
            }
            Message::ShowModal => {
                self.show_modal = true;
                Task::none()
            }
            Message::HideModal => {
                self.hide_modal();
                Task::none()
            }
            Message::ThemeChanged(index) => {
                if let Some(theme) = Theme::ALL.get(index).cloned() {
                    self.settings.set_theme(theme);
                    if let Err(e) = self.settings.save() {
                        eprintln!("Failed to save settings: {}", e);
                    }
                }
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

    /// Returns the current theme
    fn theme(&self) -> Theme {
        self.settings.theme()
    }

    /// Renders the application view
    fn view(&self) -> Element<Message> {
        let handle = svg::Handle::from_path("src/assets/icons/mdi--mixer-settings.svg");

        let svg_widget = svg(handle).style(|theme: &Theme, _| svg::Style {
            color: theme.palette().text.into(),
            ..svg::Style::default()
        });

        // Use the payloads component
        let payloads = components::payload_list(&self.storage);

        let content = container(
            column![
                row![
                    horizontal_space(),
                    button(svg_widget.width(20).height(20))
                        .style(button::secondary)
                        .on_press(Message::ShowModal)
                ]
                .padding(10) // Add padding to the entire row
                .height(Length::Shrink),
                payloads,
                row![horizontal_space()]
                    .align_y(Bottom)
                    .height(Length::Shrink),
            ]
            .height(Fill),
        );

        if self.show_modal {
            // Get the current theme
            let current_theme = self.theme();

            // Use the settings modal component
            let settings_content = components::settings_modal(current_theme);

            components::modal(content, settings_content, Message::HideModal)
        } else {
            content.into()
        }
    }
}

impl App {
    /// Hides the modal dialog
    fn hide_modal(&mut self) {
        self.show_modal = false;
    }
}
