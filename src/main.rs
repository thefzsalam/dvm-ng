extern crate gtk;
extern crate gio;
use gtk::*;
use gio::*;
mod views;
use views::*;
mod main_app;
use main_app::*;

const MAIN_APP:MainApp = MainApp {};

fn main() {
    let application = gtk::Application::new("in.fzs.democracy",gio::ApplicationFlags::FLAGS_NONE).unwrap();
    application.connect_activate(activate);
    application.run(&[]);
}

fn activate(app: &gtk::Application) {
    let window          = gtk::ApplicationWindow::new(&app);
    __load_stylesheet(&window);
    let main_root       = HomeView::new(&MAIN_APP);
    window.add(&main_root.get_root_container());
    app.add_window(&window);
    window.present();
}

fn __load_stylesheet(window: &gtk::ApplicationWindow) {
    let css_provider    = CssProvider::new();
    let _result         = css_provider.load_from_data(include_str!("glade_ui/styles.css").as_bytes());
    StyleContext::add_provider_for_screen(&window.get_screen().unwrap(),&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}

