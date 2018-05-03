use views;
use gtk;
use gtk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
mod data_entry_view;

/**
* MainApp supervises the application logic, which is split into modules.
*
* There must be only one instance of MainApp.
*
* Interior mutability pattern is employed using RefCell to lazy load the UI submodules.
*
* It uses unsafe blocks to create 'static reference to itself.
*    Gtk widgets accepts only Fn + 'static as callbacks (for example, `gtk::Button::connect_clicked()`)
*    But it is impossible to obtain 'static reference to a variable which is declared in `fn main()`.
*    If `gtk::main()` is called from `fn main()`, then all variables declared inside `fn main()`
*    will exist until `gtk::main()` exits, which means until the application quits.
*    If MainApp was declared from `fn main()` then we can safely assume that all reference to it
*    will be valid till the end of the program.
*    We use this fact to override the borrow checker and provide `&'static MainApp` to the UI modules.
*
*
*/
pub struct MainApp {
    window: gtk::Window,
    data_entry_screen: RefCell<Option<gtk::Container>>
}

pub enum Screen {
    DataEntry
}

impl MainApp {

    pub fn new(window: gtk::Window) -> Self {
        MainApp {
            window,
            data_entry_screen: RefCell::from(None)
        }
    }

    pub fn start(&self) {
        self.open_screen(Screen::DataEntry);
        self.window.show_all();
    }

    pub fn open_screen(&self, screen: Screen) {
        if let Some(child) = self.window.get_child() {
            self.window.remove(&child);
        }
        self.window.add(
            match screen {
                Screen::DataEntry => {
                    if self.data_entry_screen.borrow().is_none() {
                        *self.data_entry_screen.borrow_mut() =
                            Some(views::data_entry_view::load_ui_logic(self.get_static_ref()));
                    }
                    self.data_entry_screen.borrow()
                }
            }.as_ref().unwrap()
        );
    }

    fn get_static_ref(&self) -> &'static Self {
        let p = self as *const Self;
        unsafe {
            &*p
        }
    }
}

