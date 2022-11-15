use std::cell::Cell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use gtk::{Button, ComboBox, ComboBoxText, Entry, gio, Label, ListBox, ListBoxRow, Orientation, Paned, PolicyType, ScrolledWindow, SelectionMode, Widget};
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gtk::ffi::GtkListBox;
use gtk::glib::{clone, MainContext, PRIORITY_DEFAULT};
use crate::clone::Downgrade;
use crate::clone::Upgrade;
use crate::main_window::MainWindow;

mod main_window;
mod add_job_button;
mod job_info;
mod section;

const APP_ID: &str = "org.jameshendry.Rnotifyd";

fn main() {
    gio::resources_register_include!("compiled.gresource")
        .expect("Failed to register resources");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_templated_ui);
    // Run the application
    app.run();
}

fn build_templated_ui(app: &Application) {
    let main_window = MainWindow::new(app);
    main_window.present();
}