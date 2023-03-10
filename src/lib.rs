use history_trait::Nav;
use iced_native::widget::scrollable::RelativeOffset;
use state::State;

pub mod history_trait;
pub mod state;

#[derive(Debug)]
pub struct Route<T> {
    data: State,
    title: String,
    page: T,
    scrollable: Option<(iced_native::widget::scrollable::Id, RelativeOffset)>,
}

impl<T> Route<T> {
    pub fn new(page: impl Into<T>, title: impl Into<String>) -> Self {
        Self {
            data: State::None,
            title: title.into(),
            page: page.into(),
            scrollable: None,
        }
    }

    pub fn set_state(self, state: State) -> Self {
        Self {
            data: state,
            ..self
        }
    }

    pub fn set_scrollable(
        self,
        id: iced_native::widget::scrollable::Id,
        offset: RelativeOffset,
    ) -> Self {
        Self {
            scrollable: Some((id, offset)),
            ..self
        }
    }
}

#[derive(Debug)]
pub struct Router<T> {
    history: Vec<Route<T>>,
    current: Option<Route<T>>,
    future: Vec<Route<T>>,
    update: bool,
}

impl<T> Router<T> {
    pub fn new(route: Route<T>) -> Self {
        Self {
            history: Vec::new(),
            current: Some(route),
            future: Vec::new(),
            update: false,
        }
    }

    fn go_back(&mut self, number: usize) {
        let lenth = self.history.len();
        if number != 0 && number <= lenth {
            let mut removed_routes: Vec<Route<T>> =
                self.history.drain((lenth - number)..(lenth - 1)).collect();
            self.current = Some(removed_routes.remove(0));
            removed_routes.reverse();
            self.future.append(&mut removed_routes);
        }
    }
    fn go_forward(&mut self, number: usize) {
        let lenth = self.future.len();
        if number != 0 && number <= lenth {
            let mut removed_routes: Vec<Route<T>> =
                self.future.drain((lenth - number)..(lenth - 1)).collect();
            self.current = Some(removed_routes.remove(0));
            removed_routes.reverse();
            self.history.append(&mut removed_routes);
        }
    }
}

impl<T> history_trait::History<T> for Router<T>
where
    T: Clone,
{
    type State = State;

    fn lenth(&self) -> usize {
        self.history.len() + self.future.len() + 1
    }

    fn title(&self) -> String {
        self.current.as_ref().unwrap().title.clone()
    }

    fn page(&self) -> T {
        self.current.as_ref().unwrap().page.clone()
    }

    fn state(&self) -> &State {
        &self.current.as_ref().unwrap().data
    }

    fn set_scroll(&mut self, off_set: RelativeOffset) {
        if let Some(scroll) = &mut self.current.as_mut().unwrap().scrollable {
            scroll.1 = off_set
        }
    }

    fn back(&mut self) {
        if self.history.len() > 0 {
            let current = self.current.take().unwrap();
            self.future.push(current);
            self.current = self.history.pop();
        }
        self.update = true;
    }

    fn forward(&mut self) {
        if self.future.len() > 0 {
            let current = self.current.take().unwrap();
            self.history.push(current);
            self.current = self.future.pop();
        }
        self.update = true;
    }

    fn go(&mut self, nav: impl Into<history_trait::Nav>) {
        match nav.into() {
            Nav::Number(number) => {
                if number > 0 {
                    self.go_forward(number as usize)
                }
                if number < 0 {
                    self.go_back(number.abs() as usize)
                }
            }
            _ => {} // todo
                    // Nav::Url(url) => {
                    //     let current = self.current.take().unwrap();
                    //     self.current = Some(Route {
                    //         data: State::None,
                    //         title: current.title.clone(),
                    //         url,
                    //         scrollable: RelativeOffset::default(),
                    //     });
                    //     self.history.push(current);
                    // }
        }
        self.update = true;
    }

    fn push_state(&mut self, route: Route<T>) {
        let current = self.current.take().unwrap();
        self.current = Some(route);
        self.history.push(current);
        self.update = true;
    }

    fn replace_state(&mut self, route: Route<T>) {
        self.current = Some(route);
    }

    fn scroll<Message: 'static>(&self) -> iced_native::Command<Message> {
        let scroll = &self.current.as_ref().unwrap().scrollable;
        if let Some((id, offset)) = scroll {
            iced_native::widget::scrollable::snap_to(id.clone(), offset.clone())
        } else {
            iced_native::Command::none()
        }
    }

    fn update<Message: 'static>(&mut self) -> iced_native::Command<Message> {
        if self.update {
            self.update = false;
            self.scroll()
        } else {
            iced_native::Command::none()
        }
    }
}
