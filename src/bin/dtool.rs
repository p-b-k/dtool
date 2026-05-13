////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// First test of a Hello World
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use gtk::prelude::*;
use gtk::{Application, glib};
use gtk4 as gtk;

fn main() -> glib::ExitCode {
    gtk::init().expect("Uable to init GTK");

    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let builder = gtk::Builder::from_string(include_str!("../mainwin.ui"));
        let window: gtk::Window = builder.object("mainwin").expect("Unable to load mainwin");
        app.add_window(&window);

        let b_add: gtk::Button = builder.object("add").expect("Unable to get add button");
        let b_edit: gtk::Button = builder.object("edit").expect("Unable to get edit button");

        b_add.connect_clicked(|_| {
            eprintln!("Clicked!");
        });

        window.present();
    });

    println!("About to run");
    application.run()
}
