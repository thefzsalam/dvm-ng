extern crate gtk;

pub trait IView {
    fn getRootContainer(&self) -> gtk::Container;
}