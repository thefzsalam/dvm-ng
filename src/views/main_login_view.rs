extern crate gtk;

const UI_FILE: &str = include_str!("../glade_ui/main_login.glade");

use views::IView;

pub struct MainLoginView {
    userIdEntry: gtk::Entry,
    passwordEntry: gtk::Entry,
    loginButton: gtk::Button,
    rootContainer: gtk::Container
}

impl MainLoginView {
    fn new(dbActionsHandler: &DBActionsHandler) {
        let builder = gtk::Builder::new_from_string(UI_FILE);
        let mainLoginView = MainLoginView {
            userIdEntry: builder.get_object("userIdEntry").unwrap(),
            passwordEntryEntry: builder.get_object("passwordEntry").unwrap(),
            loginButton: builder.get_object("loginButton").unwrap(),
            rootContainer: builder.get_object("root").unwrap()
        };
        mainLoginView.loginButton.connect_click(move |_| {
            println!("Hello world!");
        });
        mainLoginView
    }
}

impl IView for MainLoginView {
    fn getRootContainer(&self) {
        self.rootContainer
    }
}

pub trait DBActionsHandler {
    fn tryLogin(userId: String, password: String) -> bool;
}