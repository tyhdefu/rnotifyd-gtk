use gtk::glib;

mod imp;

glib::wrapper! {
    pub struct JobInfo(ObjectSubclass<imp::JobInfo>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Orientable;
}
