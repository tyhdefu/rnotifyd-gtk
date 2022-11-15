mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct AddJobButton(ObjectSubclass<imp::AddJobButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}