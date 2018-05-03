extern crate gtk;
use gtk::prelude::*;
use ::models::*;
use std::rc::Rc;

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
    data_entry_view: DataEntryViewContainer,
    generic_list_view: GenericListView,
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

pub fn load_ui_logic (navigator: &'static Navigator) -> gtk::Container
{
    let ui = load_all_ui_components();
    ui.data_entry_view.root_container.pack_end(&ui.generic_list_view.root_container,true, true, 0);

    // looks like the move keyword here doesn't prevent the reuse of the reference navigator.
    // probably it is cloning the *reference* while moving.
    ui.data_entry_view.home_button.connect_clicked(move |_| {
        navigator.go_home();
    });

    ui.data_entry_view.root_container.upcast::<gtk::Container>()
}

fn load_all_ui_components() -> UI {
    UI{
        data_entry_view: DataEntryViewContainer::build(
            &gtk::Builder::new_from_string(uifiles::TOPLEVEL)),
        generic_list_view: GenericListView::build(
            &gtk::Builder::new_from_string(uifiles::GENERIC_LIST_VIEW)),
    }
}

fn load_election_sessions_editing(generic_list_view: &GenericListView,
                                  election_sessions_db: &CRUDHandler<ElectionSession>) {
    /* TODO */
}




pub trait Navigator {
    fn go_home(&self);
}

pub trait CRUDHandler<T> {
    fn get_all      (&self) -> Vec<T>;
    fn insert_one   (&self, to_insert: &T);
    fn update_one   (&self, to_update: &T);
    fn delete_many  (&self, list_to_delete: &Vec<T>);
}
