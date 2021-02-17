use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ApplicationWindowBuilder, LabelBuilder, Notebook};

use super::about::AboutDialog;
use super::icon::icon;
use super::OverviewPage;
use crate::model::Overview;
use std::sync::{Arc, Once};
use std::rc::Rc;

static START: Once = Once::new();

#[derive(Debug, Clone)]
pub struct App {
    application: Application,
    window: Option<ApplicationWindow>,
    about_dialog: Rc<Option<AboutDialog>>,
    overview_page: Rc<OverviewPage>,
}

impl App {
    pub fn new(overview: Arc<Overview>) -> Arc<Self> {
        START.call_once(|| {
            if gtk::init().is_err() {
                eprintln!("failed to initialize GTK Application");
                std::process::exit(1);
            }
        });

        let application = Application::new(Some("de.hpi.felix-gohla.meminfo"), Default::default())
            .expect("Application::new failed");

        application.set_default();

        let overview_page = Rc::new(OverviewPage::new(overview));

        let app = Arc::new(App {
            application: application,
            window: None,
            about_dialog: Rc::new(None),
            overview_page,
        });

        {
            let app_clone = app.clone();
            app.application.connect_startup(move |_| {
                println!("application startup");
                app_clone.build_menubar();
                app_clone.add_accelerators();
            });
        }

        {
            let app_clone = app.clone();
            app.application.connect_activate(move |application| {
                println!("application activate");
                let mut app_clone = app_clone.clone();
                let mut app_clone = Arc::make_mut(&mut app_clone);
                let window = App::build_window(&application);
                let about_dialog = AboutDialog::new(&window);

                app_clone.window = Some(window);
                app_clone.about_dialog = Rc::new(Some(about_dialog));

                app_clone.add_actions();
                app_clone.build_ui();
            });
        }
        app
    }

    fn build_window(application: &Application) -> ApplicationWindow {
        ApplicationWindowBuilder::new()
            .application(application)
            .title("MemInfo")
            .icon(&icon(128))
            .show_menubar(true)
            .width_request(640)
            .height_request(480)
            .default_width(640)
            .default_height(480)
            .build()
    }

    fn build_menubar(&self) {
        let menu = gio::Menu::new();
        let menu_bar = gio::Menu::new();
        let more_menu = gio::Menu::new();
        let settings_menu = gio::Menu::new();
        let submenu = gio::Menu::new();

        // The first argument is the label of the menu item whereas the second is the action name. It'll
        // makes more sense when you'll be reading the "add_actions" function.
        menu.append(Some("Reload"), Some("app.reload"));
        menu.append(Some("Quit"), Some("app.quit"));

        settings_menu.append(Some("Sub another"), Some("app.sub_another"));
        submenu.append(Some("Sub sub another"), Some("app.sub_sub_another"));
        submenu.append(Some("Sub sub another2"), Some("app.sub_sub_another2"));
        settings_menu.append_submenu(Some("Sub menu"), &submenu);
        menu_bar.append_submenu(Some("_Another"), &settings_menu);

        more_menu.append(Some("About"), Some("app.about"));
        menu_bar.append_submenu(Some("?"), &more_menu);

        self.application.set_app_menu(Some(&menu));
        self.application.set_menubar(Some(&menu_bar));
    }

    fn add_accelerators(&self) {
        self.application.set_accels_for_action("app.about", &["F1"]);
        self.application
            .set_accels_for_action("app.reload", &["F5"]);
        self.application
            .set_accels_for_action("app.quit", &["<Primary>Q"]);
    }

    fn add_actions(&self) {
        let window = self
            .window
            .as_ref()
            .expect("ApplicationWindow has been initialized for adding actions.");

        let reload = gio::SimpleAction::new("reload", None);
        {
            let overview_page_clone = self.overview_page.clone();
            reload.connect_activate(move |_, _| {
                overview_page_clone.update();
            });
        }

        let quit = gio::SimpleAction::new("quit", None);
        quit.connect_activate(glib::clone!(@weak window => move |_, _| {
            window.close();
        }));

        let about = gio::SimpleAction::new("about", None);
        let dialog = self.about_dialog.clone();
        about.connect_activate(move |_, _| {
            dialog
                .as_ref()
                .as_ref()
                .expect("AboutDialog has been initialized.")
                .show();
        });

        // We need to add all the actions to the application so they can be taken into account.
        self.application.add_action(&reload);
        self.application.add_action(&about);
        self.application.add_action(&quit);
    }

    fn build_ui(&self) {
        let window = self
            .window
            .as_ref()
            .expect("ApplicationWindow has been initialized.");
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

        self.build_notebook(&v_box);
        window.add(&v_box);
        window.show_all();
    }

    fn build_notebook(&self, container: &gtk::Box) {
        let notebook = Notebook::new();

        let overview_label = LabelBuilder::new().label("Overview").build();
        notebook.append_page(self.overview_page.page(), Some(&overview_label));
        notebook.show_all();

        container.pack_start(&notebook, true, true, 0);
    }

    pub fn run(&self, args: &Vec<String>) {
        self.application.run(args);
    }
}
