use views;

use std::rc::Rc;

impl views::data_entry_view::Navigator for super::MainApp {
    fn go_home(&self) {
        self.open_screen(super::Screen::DataEntry);
    }
}
