use gtk;
use gtk::prelude::*;
mod delayed_initializer;
use self::delayed_initializer::*;
use views;

mod data_entry_view_impl;

/**
* MainApp supervises the application logic, which is split into modules.
*
* There must be only one instance of MainApp.
*
* It uses unsafe blocks to create 'static reference to itself.
*    Gtk widgets accepts only Fn + 'static as callbacks (for example, `gtk::Button::connect_clicked()`)
*    But it is impossible to obtain 'static reference to a variable which is declared in `fn main()`.
*    If `gtk::main()` is called from `fn main()`, then all variables declared inside `fn main()`
*    will exist until `gtk::main()` exits, which means until the application quits.
*    If MainApp was declared from `fn main()` then we can safely assume that all reference to it
*    will be valid till the end of the program.
*    We use this fact to override the borrow checker and provide `&'static MainApp` to the UI modules.
*
*
*/
pub struct MainApp {
    window: gtk::Window,
    views: DelayedInitializer<Views>
}

struct Views {
    data_entry: views::data_entry_view::DataEntryView
}

impl MainApp {

    /**
     * Take ownership of a window and return a heap allocated instance of MainApp.
     *
     * If we simply instantiate a MainApp and move it out of the constructor by returning,
     * all references created by get_static_ref() will be invalidated by moving out of the stack
     * frame of constructed function.
     * This had caused SIGSEGV when the views tried to use the passed static_ref.
     * This is solved by heap allocating MainApp, giving it a permanent address.
     */
    pub fn new(window: gtk::Window) -> Box<MainApp> {
        // members of `Views` need `&'static MainApp` in their constructors, which won't be available
        // until after MainApp is constructed. This is because they need a reference to the
        // trait objects like `Navigator`, `CRUDHandler`, and currently they are implemented by MainApp.
        let mut main_app = Box::from(MainApp {
            window,
            views: DelayedInitializer::new_uninitialized()
        });
        let static_ref = main_app.get_static_ref();
        let views = Views {
            data_entry: views::data_entry_view::DataEntryView::load(static_ref)
        };
        main_app.views.initialize(views);
        main_app
    }

    /**
     * This function must be called from `fn main()` before `gtk::main()` to initialize the UI
     * and app logic.
     */
    pub fn start(&self) {
        self.open_view(&self.views.data_entry);
        self.window.show_all();
    }

    /**
     * Loads an IView into current window, replacing the existing one.
     * Used by implementations of view traits to switch between views.
     */
    pub fn open_view(&self, view: &views::IView) {
        // Remove the child of `self.window` if it is not empty
        if let Some(child) = self.window.get_child() {
            self.window.remove(&child);
        }
        self.window.add(&view.get_root_container());
    }

    /**
     * Unsafely creates `&'static MainApp`, for passing into view modules,
     * which require them for use in button callbacks and other UI event callbacks.
     * *NOTE:* If the instance of MainApp is moved, all references returned by get_static_ref() will
     * be invalidated. This is not checked by borrow checker.
     */
    fn get_static_ref(&self) -> &'static Self {
        let p = self as *const Self;
        unsafe {
            &*p
        }
    }
}
