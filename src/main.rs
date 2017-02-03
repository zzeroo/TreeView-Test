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


fn append_column(treeview: &gtk::TreeView, id: i32, title: String) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);
    // // Die Daten und das View werden über `id` Spalte des Models und
    // über die `id` Spalte des Stores verbunden.
    column.add_attribute(&cell, "text", id);
    column.set_title(&title);
    // Diverse Attribute
    column.set_resizable(false);
    column.set_clickable(false);
    treeview.append_column(&column);
}

/// Basis Setup des TreeViews
///
fn setup_treeview(treeview: &gtk::TreeView) {
    append_column(&treeview, 0, "Modbus Slave Id".to_string());
    append_column(&treeview, 1, "Type".to_string());
    append_column(&treeview, 2, "Value".to_string());
    append_column(&treeview, 3, "SI".to_string());
    append_column(&treeview, 4, "Errors".to_string());
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

    // TreeView
    let treeview = gtk::TreeView::new();

    // TreeStore
    let treestore = gtk::TreeStore::new(
        &[
        u32::static_type(),     // Modbus Slave ID
        String::static_type(),  // Type
        String::static_type(),  // Value
        String::static_type(),  // SI
        u32::static_type(),     // Error Counter
        ]
    );
    treeview.set_model(Some(&treestore));
    setup_treeview(&treeview);
    // Header verstecken
    treeview.set_headers_visible(true);

    for i in 1..20 {
        treestore.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3, 4],
            &[
                &i,
                &"Test Kombisensor",
                &"0",
                &"ppm",
                &0
            ]
        );

    }







    box_main.add(&info_bar);

    box_main.add(&treeview);

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
