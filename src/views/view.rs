extern crate gtk;

pub trait IView<'a> {
    fn get_root_container(&'a self) -> &'a gtk::Container;
}