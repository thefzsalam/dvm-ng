use views;

pub struct MainApp {

}

impl views::main_login_view::Navigator for MainApp {
    fn try_login(&self, user_id: String, password: String) -> bool {
        user_id == "fzn" && password == "fzn"
    }
}

impl views::home_view::Navigator for MainApp {
    fn start_voting(&self) {
        println!("start_voting");
    }
    fn open_data_entry(&self) {
        println!("open_data_entry");
    }
    fn open_administration(&self) {
        println!("open_administration");
    }
    fn view_results(&self) {
        println!("view_results");
    }
    fn logout(&self) {
        println!("logout");
    }
}