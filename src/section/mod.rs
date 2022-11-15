use gtk::glib;

mod imp;

glib::wrapper! {
    pub struct Section(ObjectSubclass<imp::Section>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Orientable;
}