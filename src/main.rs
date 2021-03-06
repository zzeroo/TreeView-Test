extern crate gdk;
extern crate glib;
extern crate gobject_sys;
extern crate gtk_sys;
extern crate gtk;
extern crate hyper;
extern crate serde_json;
extern crate xmz_server;

use gtk::prelude::*;
use gdk::enums::key;


// make moving clones into closures more convenient
#[macro_export]
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}


fn main() {
    gtk::init().unwrap_or_else(|_| panic!("phoronix-reader: failed to initialize GTK."));

    // Disable Animationen
    // http://stackoverflow.com/questions/39271852/infobar-only-shown-on-window-change/39273438#39273438
    // https://gitter.im/gtk-rs/gtk?at=57c8681f6efec7117c9d6b5e
    unsafe{
        use glib::translate::ToGlibPtr;
        gobject_sys::g_object_set (gtk_sys::gtk_settings_get_default() as *mut gobject_sys::GObject,
        "gtk-enable-animations".to_glib_none().0, 0, 0);
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_default_size(1024, 600);
    window.set_title("xMZ-Mod-Touch-GUI");

    #[cfg(not(feature = "dev"))]
    window.fullscreen();

    // Fenster Elemente
    let box_main = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let info_bar = gtk::InfoBar::new();
    let info_bar_button = info_bar.add_button(&"Close", 0).unwrap();

    { // Hide InfoBar
        info_bar.connect_response(clone!(info_bar => move |info_bar, _| info_bar.hide() ));
    }








    box_main.add(&info_bar);

    window.add(&box_main);
    window.show_all();
    
    info_bar.hide();

    // Quit the program when the program has been exited
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Define custom actions on keypress
    window.connect_key_press_event(move |_, key| {
        match key.get_keyval() {
            key::Escape => gtk::main_quit(),
            _ => ()
        }
        gtk::Inhibit(false)
    });

    // 1Sekunden Timer
    gtk::timeout_add(3000, clone!(info_bar => move || {
        info_bar.show();

        glib::Continue(true)
    }));


    gtk::main();
}
