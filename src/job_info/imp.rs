use gtk::ffi::{GtkBox, GtkDropDown};
use gtk::{glib, CompositeTemplate, DropDown, StringList};
use gtk::gio::ListModel;
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use crate::glib::subclass::InitializingObject;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/jameshendry/rnotifyd/job_info.ui")]
pub struct JobInfo {
    #[template_child]
    pub action_type: TemplateChild<DropDown>,

    //#[template_child]
    //pub frequency: TemplateChild<GtkBox>,

    #[template_child]
    pub notify_definition: TemplateChild<gtk::Box>,
}

#[glib::object_subclass]
impl ObjectSubclass for JobInfo {
    const NAME: &'static str = "RnotifydJobInfo";
    type Type = super::JobInfo;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }

}

// Trait shared by all GObjects
impl ObjectImpl for JobInfo {
    fn constructed(&self) {
        self.parent_constructed();

        let string_list = StringList::from_iter(vec!["hi", "there"]);
        let list_model = ListModel::from(string_list);
        self.action_type.set_model(Some(&list_model));
    }
}

// Trait shared by all widgets
impl WidgetImpl for JobInfo {}

impl BoxImpl for JobInfo {}