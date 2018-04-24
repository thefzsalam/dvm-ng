extern crate gtk;
use gtk::prelude::*;

use views::IView;

const UI_FILE: &str = include_str!("../glade_ui/main_login.glade");

/* TODO:
    1. Show a red `invalid password` depending on the result from try_login(str,str) -> bool
    2. Add a title saying welcome to EVM.
    3. (Least priority) Add a cool background image.
*/

pub struct MainLoginView {
    user_id_entry: gtk::Entry,
    password_entry: gtk::Entry,
    login_button: gtk::Button,
    root_container: gtk::Container,
    navigator: &'static Navigator
}

impl MainLoginView {

    pub fn new(navigator: &'static Navigator) -> MainLoginView {

        let builder = gtk::Builder::new_from_string(UI_FILE);
        let main_login_view :   MainLoginView = MainLoginView {
            user_id_entry   :   builder.get_object("user_id_entry").unwrap(),
            password_entry  :   builder.get_object("password_entry").unwrap(),
            login_button    :   builder.get_object("login_button").unwrap(),
            root_container  :   builder.get_object("root_container").unwrap(),
            navigator
        };

        let user_id_entry_clone     = main_login_view.user_id_entry.clone();
        let password_entry_clone    = main_login_view.password_entry.clone();
        let navigator_clone         = main_login_view.navigator;

        main_login_view.login_button.connect_clicked(move |_| {
            println!("{}",
                if navigator_clone.try_login(
                    user_id_entry_clone.get_text().unwrap(),
                    password_entry_clone.get_text().unwrap()
                ) { "Success!"}
                else {"Failed!"}
            );
        });

        main_login_view
    }

}

impl IView for MainLoginView {
    fn get_root_container(&self) -> gtk::Container {
        self.root_container.clone()
    }
}

pub trait Navigator {
    fn try_login(&self, user_id: String, password: String) -> bool;
}

