mod clock;

use iced::{
    mouse,
    widget::{self, button, canvas::Cache, container, scrollable, text, vertical_space},
    Alignment, Application, Length, Settings,
};
use iced_router::{history_trait::History, Route, Router};
use once_cell::sync::Lazy;

static SCROLLABLE_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);

fn main() {
    StringRouter::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
    .unwrap()
}

#[derive(Debug)]
struct StringRouter {
    router: iced_router::Router<String>,
    now: time::OffsetDateTime,
    clock: Cache,
    value: i32,
}

#[derive(Debug, Clone)]
enum Pages {
    Home,
    CLock,
    Counter,
    Scrollable,
}

#[derive(Debug, Clone)]
enum Navgate {
    Forward,
    Back,
}

#[derive(Debug, Clone)]
enum Message {
    SetPage(Pages),
    IncrementPressed,
    DecrementPressed,
    ScrollToEnd,
    ScrollToBeginning,
    Scrolled(scrollable::RelativeOffset),
    Nav(Navgate),
    Events(iced::Event),
}

impl iced::Application for StringRouter {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            StringRouter {
                router: Router::new(Route::new("home".to_string(), "Home")),
                now: time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
                clock: Default::default(),
                value: 0,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        self.router.title()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::events().map(Message::Events)
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::SetPage(page) => match page {
                Pages::Home => self
                    .router
                    .push_state(Route::new("home".to_string(), "home")),
                Pages::CLock => self
                    .router
                    .push_state(Route::new("clock".to_string(), "clock")),
                Pages::Counter => self
                    .router
                    .push_state(Route::new("counter".to_string(), "counter")),
                Pages::Scrollable => self.router.push_state(
                    Route::new("scrollable".to_string(), "scrollable")
                        .set_scrollable(SCROLLABLE_ID.clone(), scrollable::RelativeOffset::START),
                ),
            },
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::ScrollToEnd => {
                self.router.set_scroll(scrollable::RelativeOffset::END);
                return scrollable::snap_to(SCROLLABLE_ID.clone(), scrollable::RelativeOffset::END);
            }
            Message::ScrollToBeginning => {
                self.router.set_scroll(scrollable::RelativeOffset::START);
                return scrollable::snap_to(
                    SCROLLABLE_ID.clone(),
                    scrollable::RelativeOffset::START,
                );
            }
            Message::Scrolled(offset) => self.router.set_scroll(offset),
            Message::Nav(nav) => match nav {
                Navgate::Forward => self.router.forward(),
                Navgate::Back => self.router.back(),
            },
            Message::Events(event) => {
                if let iced::Event::Mouse(mouse::Event::ButtonPressed(event)) = event {
                    match event {
                        mouse::Button::Other(32) => self.router.back(),
                        mouse::Button::Other(64) => {
                            self.router.forward();
                        }
                        _ => {}
                    }
                }
            }
        }
        self.router.update()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let menu = widget::row![
            widget::row![
                button("Home").on_press(Message::SetPage(Pages::Home)),
                button("Counter").on_press(Message::SetPage(Pages::Counter)),
                button("Clock").on_press(Message::SetPage(Pages::CLock)),
                button("Solar system").on_press(Message::SetPage(Pages::Scrollable))
            ],
            widget::horizontal_space(Length::Fill),
            widget::row![
                button("Back").on_press(Message::Nav(Navgate::Back)),
                button("Forward").on_press(Message::Nav(Navgate::Forward))
            ]
        ];

        match self.router.page().as_str() {
            "home" => {
                let content = container("Home page Try out the Router")
                    .center_x()
                    .center_y()
                    .width(Length::Fill)
                    .height(Length::Fill);
                widget::column![menu, content].into()
            }
            "clock" => {
                let canvas = widget::canvas(self as &Self)
                    .width(Length::Fill)
                    .height(Length::Fill);

                let content = container(canvas)
                    .center_x()
                    .center_y()
                    .width(Length::Fill)
                    .height(Length::Fill);
                widget::column![menu, content].into()
            }
            "counter" => {
                let content = container(
                    widget::column![
                        button("Increment").on_press(Message::IncrementPressed),
                        text(self.value).size(50),
                        button("Decrement").on_press(Message::DecrementPressed)
                    ]
                    .padding(20)
                    .align_items(Alignment::Center),
                )
                .center_x()
                .center_y()
                .width(Length::Fill)
                .height(Length::Fill);
                widget::column![menu, content].into()
            }
            "scrollable" => {
                let content = container(
                    scrollable(
                        widget::column![
                            button("Scroll to end")
                                .padding(10)
                                .on_press(Message::ScrollToEnd),
                            text("Beginning!"),
                            vertical_space(1200),
                            text("Middle!"),
                            vertical_space(1200),
                            text("End!"),
                            button("Scroll to beginning")
                                .padding(10)
                                .on_press(Message::ScrollToBeginning),
                        ]
                        .width(Length::Fill)
                        .align_items(Alignment::Center)
                        .padding([40, 0, 40, 0])
                        .spacing(40),
                    )
                    .height(Length::Fill)
                    .id(SCROLLABLE_ID.clone())
                    .on_scroll(Message::Scrolled),
                )
                .center_x()
                .center_y()
                .width(Length::Fill)
                .height(Length::Fill);
                widget::column![menu, content].into()
            }
            &_ => widget::column![menu].into(),
        }
    }
}
