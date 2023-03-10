use iced_native::widget::scrollable::RelativeOffset;

use crate::Route;

pub enum ScrollRestoration {
    Manuel,
    Auto,
}

pub enum Nav {
    Number(i32),
    Url(String),
}

impl Into<Nav> for String {
    fn into(self) -> Nav {
        Nav::Url(self)
    }
}
impl Into<Nav> for i32 {
    fn into(self) -> Nav {
        Nav::Number(self)
    }
}

pub trait History<T> {
    type State;

    fn lenth(&self) -> usize;

    fn state(&self) -> &Self::State;

    fn title(&self) -> String;

    fn page(&self) -> T;
    
    fn set_scroll(&mut self, off_set: RelativeOffset);
    
    fn back(&mut self);

    fn forward(&mut self);

    fn go(&mut self, nav: impl Into<Nav>);

    fn push_state(&mut self,  route: Route<T>);

    fn replace_state(&mut self, route: Route<T>);

    fn scroll<Message: 'static>(&self) -> iced_native::Command<Message>;

    fn update<Message: 'static>(&mut self) -> iced_native::Command<Message>;
}
