extern crate gtk;
extern crate gio;

use gtk::*;
use gio::*;

mod views;

use views::MainLoginView;



fn activate(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(&app);

    let css_provider = CssProvider::new();
    css_provider.load_from_data(include_str!("glade_ui/styles.css").as_bytes());
    StyleContext::add_provider_for_screen(&window.get_screen().unwrap(),&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    let main_root = MainLoginView::new(|userid,password| {

    });

    window.add(&main_root);
    app.add_window(&window);
    window.present();
}

fn main() {
    let application = gtk::Application::new("in.fzs.democracy",gio::ApplicationFlags::FLAGS_NONE).unwrap();
    application.connect_activate(activate);
    application.run(&[]);
}
