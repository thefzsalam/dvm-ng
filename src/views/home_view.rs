extern crate gtk;
use gtk::prelude::*;

use views::IView;

const UI_FILE: &str = include_str!("../glade_ui/home.glade");

pub struct HomeView {
    start_voting_button: gtk::Button,
    data_entry_button: gtk::Button,
    administration_button: gtk::Button,
    view_results_button: gtk::Button,
    logout_button: gtk::Button,
    root_container: gtk::Container,
    navigator: &'static Navigator
}

impl HomeView {
    pub fn new(navigator: &'static Navigator) -> Self {
        let builder = gtk::Builder::new_from_string(UI_FILE);
        let home_view = HomeView {
            start_voting_button: builder.get_object("start_voting_button").unwrap(),
            data_entry_button: builder.get_object("data_entry_button").unwrap(),
            administration_button: builder.get_object("administration_button").unwrap(),
            view_results_button: builder.get_object("view_results_button").unwrap(),
            logout_button: builder.get_object("logout_button").unwrap(),
            root_container: builder.get_object("root_container").unwrap(),
            navigator
        };

        let navigator_clone = navigator;
        home_view.start_voting_button.connect_clicked(move |_| {
            navigator_clone.start_voting();
        });
        home_view.data_entry_button.connect_clicked(move |_| {
            navigator_clone.open_data_entry();
        });
        home_view.administration_button.connect_clicked(move |_| {
            navigator_clone.open_administration();
        });
        home_view.view_results_button.connect_clicked(move |_| {
            navigator_clone.view_results();
        });
        home_view.logout_button.connect_clicked(move |_| {
            navigator_clone.logout();
        });
        home_view
    }
}

impl IView for HomeView {
    fn get_root_container(&self) -> gtk::Container {
        self.root_container.clone()
    }
}

pub trait Navigator {
    fn start_voting(&self);
    fn open_data_entry(&self);
    fn open_administration(&self);
    fn view_results(&self);
    fn logout(&self);
}