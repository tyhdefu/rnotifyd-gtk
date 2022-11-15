use gtk::glib;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Default)]
pub struct AddJobButton;

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for AddJobButton {
    const NAME: &'static str = "RnotifydAddJobButton";
    type Type = super::AddJobButton;
    type ParentType = gtk::Button;
}

// Trait shared by all GObjects
impl ObjectImpl for AddJobButton {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl WidgetImpl for AddJobButton {}

// Trait shared by all buttons
impl ButtonImpl for AddJobButton {}