use glib::Object;
use gtk::prelude::*;
use gtk::{gio, glib, Application, TemplateChild, Button, Paned, Orientation, Label, Stack};
use gtk::ffi::GtkStack;
use gtk::subclass::prelude::*;
use crate::gio::subclass::prelude::*;
use crate::glib::subclass::InitializingObject;

mod imp;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    pub fn new_job(&self) {
        println!("New job");
        let new = Label::new(Some("It was a dark and stormy night"));
        let items = self.imp().job_list.pages().n_items();
        self.imp().job_list.add_titled(&new, None, "new-item");
    }
}