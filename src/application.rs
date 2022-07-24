use glib::clone;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::{gio, glib};

use crate::config::{APP_ID, VERSION};
use crate::TabletsettingsWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct TabletsettingsApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for TabletsettingsApplication {
        const NAME: &'static str = "TabletsettingsApplication";
        type Type = super::TabletsettingsApplication;
        type ParentType = gtk4::Application;
    }

    impl ObjectImpl for TabletsettingsApplication {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl ApplicationImpl for TabletsettingsApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self, application: &Self::Type) {
            // Get the current window or create one if necessary
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = TabletsettingsWindow::new(application);
                window.upcast()
            };

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for TabletsettingsApplication {}
    }

glib::wrapper! {
    pub struct TabletsettingsApplication(ObjectSubclass<imp::TabletsettingsApplication>)
        @extends gio::Application, gtk4::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl TabletsettingsApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::new(&[("application-id", &application_id), ("flags", flags)])
            .expect("Failed to create TabletsettingsApplication")
    }

    fn setup_gactions(&self) {
        let quit_action = gio::SimpleAction::new("quit", None);
        quit_action.connect_activate(clone!(@weak self as app => move |_, _| {
            app.quit();
        }));
        self.add_action(&quit_action);

        let about_action = gio::SimpleAction::new("about", None);
        about_action.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about();
        }));
        self.add_action(&about_action);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let dialog = gtk4::AboutDialog::builder()
            .transient_for(&window)
            .modal(true)
            .program_name("tabletsettings")
            .license_type(gtk4::License::Gpl30)
            .website("https://github.com/avargas05/tabletsettings")
            .logo_icon_name(APP_ID)
            .version(VERSION)
            .authors(vec!["avargas05".into()])
            .artists(vec!["avargas05".into()])
            .build();

        dialog.present();
    }
}
