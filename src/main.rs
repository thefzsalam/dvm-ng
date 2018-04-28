#![feature(arbitrary_self_types)]
extern crate gtk;
use gtk::*;

mod views;
mod models;

mod main_app;
use main_app::*;

use std::rc::Rc;


fn main() {
    gtk::init().unwrap();
    let window = Window::new(WindowType::Toplevel);
    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    __load_stylesheet(&window);
    let main_app:Rc<MainApp> = Rc::from(MainApp::new(window));
    main_app.start();
    gtk::main();
}

fn __load_stylesheet(_window: &gtk::Window) {
    let css_provider    = CssProvider::new();
    let _result      = css_provider.load_from_data(include_str!("glade_ui/styles.css").as_bytes());
    StyleContext::add_provider_for_screen(&_window.get_screen().unwrap(), &css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}

