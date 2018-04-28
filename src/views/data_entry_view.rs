extern crate gtk;
use gtk::prelude::*;


use ::models::*;

use std::rc::Rc;

/* TODO:
    Use weak pointers to check for GObject memory leakage.
*/

mod uifiles {
    pub const TOPLEVEL: &str             = include_str!("../glade_ui/data_entry_toplevel.glade");
    pub const GENERIC_LIST_VIEW: &str    = include_str!("../glade_ui/generic_list_view.glade");
    pub const ELECTION_SESSION_DATA_MODEL: &str = include_str!("../glade_ui/election_session_data_model.glade");
    pub const CONSTITUENCY_DATA_MODEL: &str = include_str!("../glade_ui/constituency_data_model.glade");
    pub const CANDIDATE_DATA_MODEL: &str = include_str!("../glade_ui/candidate_data_model.glade");
    pub const DATA_MODEL_COMMON: &str    = include_str!("../glade_ui/data_model_common.glade");
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

ui_struct!(
struct DataModelCommonView {
    root_container: gtk::Box,
    action_title_label: gtk::Label,
    action_button: gtk::Button,
    cancel_button: gtk::Button
});

ui_struct!(
struct CandidateDataModelView {
    root_container: gtk::Container,
    election_session_label: gtk::Label, election_session_entry: gtk::Entry,
    name_label: gtk::Label, name_entry: gtk::Entry,
    constituency_label: gtk::Label, constituency_combo_box: gtk::ComboBox
});

struct UIComponents {
    data_entry_view: DataEntryViewContainer,
    generic_list_view: GenericListView,
    data_model_common_view: DataModelCommonView,
    candidate_data_model_view: CandidateDataModelView
}

/* database is open as long as the app is alive.
   so it makes sense to use static lifetime here.
   it is independent of whether the return value of load_ui_logic is reused or not
*/
struct CRUDHandlers {
    election_sessions_db: &'static CRUDHandler<ElectionSession>,
    constituencies_db   : &'static CRUDHandler<Constituency>,
    candidates_db       : &'static CRUDHandler<Candidate>
}

pub fn load_ui_logic<T:Navigator+'static>(navigator_rc: Rc<T>) -> gtk::Container {
    let ui = load_all_ui_components();
    ui.data_entry_view.root_container.pack_end(&ui.generic_list_view.root_container,true, true, 0);
    ui.data_model_common_view.root_container.pack_start(&ui.candidate_data_model_view.root_container, false, false, 0);
    ui.generic_list_view.right_pane.pack_start(&ui.data_model_common_view.root_container, false, false, 0);

    let navigator_rc_clone = Rc::clone(&navigator_rc);
    ui.data_entry_view.home_button.connect_clicked(move |_| {
        navigator_rc_clone.go_home();
    });

    ui.data_entry_view.root_container.upcast::<gtk::Container>()
}

fn load_all_ui_components() -> UIComponents {
    UIComponents{
        data_entry_view: DataEntryViewContainer::build(
            &gtk::Builder::new_from_string(uifiles::TOPLEVEL)),
        generic_list_view: GenericListView::build(
            &gtk::Builder::new_from_string(uifiles::GENERIC_LIST_VIEW)),
        data_model_common_view: DataModelCommonView::build(
            &gtk::Builder::new_from_string(uifiles::DATA_MODEL_COMMON)),
        candidate_data_model_view: CandidateDataModelView::build(
            &gtk::Builder::new_from_string(uifiles::CANDIDATE_DATA_MODEL))
    }
}

fn load_election_sessions_editing(generic_list_view: &GenericListView,
                                  election_sessions_db: &CRUDHandler<ElectionSession>) {
    /* TODO */
}




pub trait Navigator {
    fn go_home(self: &Rc<Self>);
}

pub trait CRUDHandler<T> {
    fn get_all      (&self) -> Vec<T>;
    fn insert_one   (&self, to_insert: &T);
    fn update_one   (&self, to_update: &T);
    fn delete_many  (&self, list_to_delete: &Vec<T>);
}
