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

    let scrolled_window = gtk::ScrolledWindow::new(None, None);
    scrolled_window.set_min_content_height(400);

    let list_box = gtk::ListBox::new();
    for i in 1..20 {
        let row1 = gtk::ListBoxRow::new();
        let row2 = gtk::ListBoxRow::new();
        let row3 = gtk::ListBoxRow::new();

        let kombisensor_type = gtk::TextView::new();
        kombisensor_type.get_buffer().unwrap().set_text(&format!("Test Kombisensor {}", i));
        kombisensor_type.set_editable(false);
        kombisensor_type.set_left_margin(5);
        kombisensor_type.set_right_margin(5);
        kombisensor_type.set_halign(gtk::Align::Start);

        let sensor1_type = gtk::TextView::new();
        sensor1_type.get_buffer().unwrap().set_text(&format!("NO2 Sensor {}", i));
        sensor1_type.set_editable(false);
        sensor1_type.set_left_margin(15);
        sensor1_type.set_right_margin(15);
        sensor1_type.set_halign(gtk::Align::Start);

        let sensor2_type = gtk::TextView::new();
        sensor2_type.get_buffer().unwrap().set_text(&format!("CO Sensor {}", i));
        sensor2_type.set_editable(false);
        sensor2_type.set_left_margin(15);
        sensor2_type.set_right_margin(15);
        sensor2_type.set_halign(gtk::Align::Start);

        row1.add(&kombisensor_type);
        row2.add(&sensor1_type);
        row3.add(&sensor2_type);

        list_box.insert(&row1, -1);
        list_box.insert(&row2, -1);
        list_box.insert(&row3, -1);
    }





    box_main.add(&info_bar);
    box_main.add(&list_box);

    scrolled_window.add(&box_main);

    window.add(&scrolled_window);
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
