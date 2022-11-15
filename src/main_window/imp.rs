use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Stack};
use crate::add_job_button::AddJobButton;
use crate::clone;
use crate::job_info::JobInfo;
use crate::section::Section;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/jameshendry/rnotifyd/window.ui")]
pub struct MainWindow {
    #[template_child]
    pub job_list: TemplateChild<Stack>,

    #[template_child]
    pub add_job_button: TemplateChild<AddJobButton>
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "RnotifydGtkAppWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        AddJobButton::ensure_type();
        JobInfo::ensure_type();
        Section::ensure_type();

        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for MainWindow {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        self.add_job_button.connect_clicked(clone!(@weak self as window => move |_| {
            println!("Hello from main window.");
            window.obj().new_job();
        }));

        /*let scrolled_window: ScrolledWindow = self.paned.start_child()
            .expect("Couldn't get start child.")
            .downcast()
            .expect("Child was not a scrolled window");

        let list_box: ListBox = scrolled_window.child()
            .expect("Couldn't get child.")
            .downcast()
            .expect("Child was not a list box");

        let button: Button = list_box.first_child()
            .expect("Couldn't get first child.")
            .downcast()
            .expect("Wasn't a button.");
        button.connect_clicked(|button| {
            println!("I got clicked.");
        });*/
        //let list_box = (self.paned.start_child().unwrap() as ListBox);
        //list_box.append()
    }
}

// Trait shared by all widgets
impl WidgetImpl for MainWindow {}

// Trait shared by all windows
impl WindowImpl for MainWindow {}

// Trait shared by all application windows
impl ApplicationWindowImpl for MainWindow {}