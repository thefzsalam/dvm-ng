extern crate gtk;
use gtk::prelude::*;
use ::models::*;
use std::rc::Rc;
use views::IView;

mod uifiles {
    pub const TOPLEVEL: &str            = include_str!("../../glade_ui/data_entry_toplevel.glade");
    pub const GENERIC_LIST_VIEW: &str   = include_str!("../../glade_ui/generic_list_view.glade");
}

ui_struct!(
struct DataEntryViewContainer {
    home_button: gtk::Button,
    election_sessions_button: gtk::Button,
    constituencies_button: gtk::Button,
    candidates_button: gtk::Button,
    id_cards_button: gtk::Button,
    root_container: gtk::Box
});

ui_struct!(
struct GenericListView {
    root_container: gtk::Paned,
    list_box: gtk::ListBox,
    combo_box: gtk::ComboBox,
    right_pane: gtk::Box
});

pub struct DataEntryView {
    data_entry_view_container: DataEntryViewContainer,
    generic_list_view: GenericListView,
    navigator: &'static Navigator
}

impl DataEntryView {
    pub fn load(navigator: &'static Navigator) -> Self{
        let view = DataEntryView {
            data_entry_view_container: DataEntryViewContainer::build(
                &gtk::Builder::new_from_string(uifiles::TOPLEVEL)),
            generic_list_view: GenericListView::build(
                &gtk::Builder::new_from_string(uifiles::GENERIC_LIST_VIEW)),
            navigator
        };
        view.data_entry_view_container.root_container.pack_end(
            &view.generic_list_view.root_container,
            true,
            true,
        0);

        // looks like the move keyword here doesn't prevent the reuse of the reference navigator.
        // probably it is cloning the *reference* while moving.
        view.data_entry_view_container.home_button.connect_clicked(move |_| {
            navigator.go_home();
        });

        view
    }
}

impl IView for DataEntryView {
    fn get_root_container(&self) -> gtk::Container {
        self.data_entry_view_container.root_container
            .clone()
            .upcast::<gtk::Container>()
    }
}

/* database is open as long as the app is alive.
   so it makes sense to use static lifetime here.
   it is independent of whether the return value of load_ui_logic is reused or not
*/
/*struct CRUDHandlers {
//    election_sessions_db: &'static CRUDHandler<ElectionSession>,
//    constituencies_db   : &'static CRUDHandler<Constituency>,
//    candidates_db       : &'static CRUDHandler<Candidate>
}*/

pub trait Navigator {
    fn go_home(&self);
}

pub trait CRUDHandler<T> {
    fn get_all      (&self) -> Vec<T>;
    fn insert_one   (&self, to_insert: &T);
    fn update_one   (&self, to_update: &T);
    fn delete_many  (&self, list_to_delete: &Vec<T>);
}
