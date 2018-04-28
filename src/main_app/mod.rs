use views;

use gtk;
use gtk::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;

mod data_entry_view;
use self::data_entry_view::*;

pub struct MainApp {
    window: gtk::Window,
    data_entry_screen: RefCell<Option<gtk::Container>>
}

pub enum Screen {
    DataEntry
}

impl MainApp {

    pub fn new(window: gtk::Window) -> Rc<Self> {
        Rc::from(MainApp {
            window,
            data_entry_screen: RefCell::from(None)
        })
    }


    pub fn start(self: Rc<Self>) {
        self.open_screen(Screen::DataEntry);
        self.window.show_all();
    }

    pub fn open_screen(self: &Rc<Self>, screen: Screen) {
        if let Some(child) = self.window.get_child() {
            self.window.remove(&child);
        }
        self.window.add(
            match screen {
                Screen::DataEntry => {
                    if self.data_entry_screen.borrow().is_none() {
                        *self.data_entry_screen.borrow_mut() =
                            Some(views::data_entry_view::load_ui_logic(Rc::clone(&self)));
                    }
                    self.data_entry_screen.borrow()
                }
            }.as_ref().unwrap()
        );
    }
}

