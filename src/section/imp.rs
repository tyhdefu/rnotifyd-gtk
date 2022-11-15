use std::cell::{Cell, RefCell};
use gtk::ffi::GtkBox;
use gtk::{glib, CompositeTemplate, DropDown, StringList, Label};
use gtk::gio::ListModel;
use gtk::glib::ParamSpecString;
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use once_cell::sync::Lazy;
use crate::glib::{BindingFlags, ParamSpec, Value};
use crate::glib::subclass::InitializingObject;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/jameshendry/rnotifyd/section.ui")]
pub struct Section {
    #[template_child]
    pub title: TemplateChild<Label>,

    title_str: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for Section {
    const NAME: &'static str = "RnotifydSection";
    type Type = super::Section;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }

}

// Trait shared by all GObjects
impl ObjectImpl for Section {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
            Lazy::new(|| vec![ParamSpecString::builder("title").build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "title" => {
                let title: String = value.get().expect("Value needs to be a string.");
                self.title_str.replace(title);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "title" => {
                self.title_str.borrow().to_value()
            }
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.bind_property("title", &*self.title, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Section {}

impl BoxImpl for Section {}