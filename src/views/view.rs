extern crate gtk;

pub trait IView {
    fn get_root_container(&self) -> gtk::Container;
}