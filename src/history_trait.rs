use iced_native::widget::scrollable::RelativeOffset;

use crate::Route;

pub enum ScrollRestoration {
    Manuel,
    Auto,
}

pub enum Nav<T> {
    Number(i32),
    Page(T),
}

impl<T> Into<Nav<T>> for i32 {
    fn into(self) -> Nav<T> {
        Nav::Number(self)
    }
}

pub trait History<T> {
    type State;

    // lenth of all items in history
    fn lenth(&self) -> usize;

    // returns the state of the current page
    fn state(&self) -> &Self::State;

    // put in title fn to autmaticlly change the title when the page is changed
    fn title(&self) -> String;

    // return the current page
    fn page(&self) -> T;

    // set current page scroll offset
    fn set_scroll(&mut self, off_set: RelativeOffset);

    // go back one in history
    fn back(&mut self);
    // go forward one in history
    fn forward(&mut self);
    // go to forward or backward or push to state
    fn go(&mut self, nav: Nav<T>);

    fn push_state(&mut self, route: Route<T>);

    fn replace_state(&mut self, route: Route<T>);

    fn scroll<Message: 'static>(&self) -> iced_native::Command<Message>;

    fn update<Message: 'static>(&mut self) -> iced_native::Command<Message>;
}
