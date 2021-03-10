use futures::channel::mpsc::UnboundedSender;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ApplicationWindowBuilder, LabelBuilder, Notebook};

use super::about::AboutDialog;
use super::dispatch::DispatchLoop;
use super::icon::icon;
use super::no_root_dialog::display_no_root_dialog;
use super::{AppAction, OverviewPage};
use crate::model::Overview;
use std::rc::Rc;
use std::sync::{Arc, Mutex, Once};

static START: Once = Once::new();

#[derive(Debug)]
pub struct App {
    application: Rc<Application>,
    overview_page: OverviewPage,
    message_sender: UnboundedSender<AppAction>,
    window: Mutex<Option<ApplicationWindow>>,
}

impl App {
    pub fn new(overview: Arc<Overview>, message_sender: UnboundedSender<AppAction>) -> Self {
        START.call_once(|| {
            if gtk::init().is_err() {
                eprintln!("failed to initialize GTK Application");
                std::process::exit(1);
            }
        });

        let application = Rc::new(
            Application::new(Some("de.hpi.felix-gohla.meminfo"), Default::default())
                .expect("Application::new failed"),
        );

        application.set_default();

        let overview_page = OverviewPage::new(overview.clone());
        let app = Self {
            application: application,
            overview_page,
            message_sender,
            window: Mutex::new(Default::default()),
        };

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

    fn build_menubar(application: &Application) {
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

        application.set_app_menu(Some(&menu));
        application.set_menubar(Some(&menu_bar));
    }

    fn add_accelerators(application: &Application) {
        application.set_accels_for_action("app.about", &["F1"]);
        application.set_accels_for_action("app.reload", &["F5"]);
        application.set_accels_for_action("app.quit", &["<Primary>Q"]);
    }

    fn add_actions(
        application: &Application,
        window: &ApplicationWindow,
        overview_page: &OverviewPage,
        about_dialog: AboutDialog,
    ) {
        let reload = gio::SimpleAction::new("reload", None);
        {
            let overview_page_clone = overview_page.clone();
            reload.connect_activate(move |_, _| {
                overview_page_clone.update();
            });
        }

        let quit = gio::SimpleAction::new("quit", None);
        quit.connect_activate(glib::clone!(@weak window => move |_, _| {
            window.close();
        }));

        let about = gio::SimpleAction::new("about", None);
        about.connect_activate(move |_, _| {
            about_dialog.show();
        });

        // We need to add all the actions to the application so they can be taken into account.
        application.add_action(&reload);
        application.add_action(&about);
        application.add_action(&quit);
    }

    fn build_ui(window: &ApplicationWindow, overview_page: &OverviewPage) {
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

        App::build_notebook(&v_box, overview_page);
        window.add(&v_box);
        window.show_all();
    }

    fn build_notebook(container: &gtk::Box, overview_page: &OverviewPage) {
        let notebook = Notebook::new();

        let overview_label = LabelBuilder::new().label("Overview").build();
        notebook.append_page(overview_page.page(), Some(&overview_label));
        notebook.show_all();

        container.pack_start(&notebook, true, true, 0);
    }

    fn handle(&mut self, action: AppAction) {
        match action {
            AppAction::AppInit => {
                println!("well, init.");
            }
            AppAction::ShowNoRootDialog => {
                self.show_no_root_dialog();
            }
            AppAction::MeminfoUpdate => {
                println!("I should update...");
            }
        }
    }

    fn show_no_root_dialog(&self) {
        let window_lock = self.window.lock().unwrap();
        let window = window_lock.as_ref().expect("Window has been initialized.");
        if display_no_root_dialog(window) {
            println!("restart");
        }
    }

    pub fn run(self, args: Vec<String>, dispatch_loop: DispatchLoop) {
        let rc_self = Rc::new(std::cell::RefCell::new(self));
        let context = glib::MainContext::default();
        let application = rc_self.borrow().application.clone();

        rc_self
            .borrow()
            .application
            .connect_startup(move |application| {
                App::build_menubar(&application);
                App::add_accelerators(&application);
            });

        let overview_page_clone = rc_self.borrow().overview_page.clone();
        let message_sender = rc_self.borrow().message_sender.clone();
        {
            let rc_self_clone = rc_self.clone();
            rc_self
                .borrow()
                .application
                .connect_activate(move |application| {
                    message_sender.unbounded_send(AppAction::AppInit).unwrap();
                    let window = App::build_window(&application);
                    let about_dialog = AboutDialog::new(&window);

                    App::add_actions(&application, &window, &overview_page_clone, about_dialog);
                    App::build_ui(&window, &overview_page_clone);
                    rc_self_clone
                        .borrow()
                        .window
                        .lock()
                        .unwrap()
                        .replace(window);
                });
        }

        context.spawn_local(async move {
            let mut app = rc_self.borrow_mut();
            dispatch_loop
                .attach(move |action| {
                    println!("here: {:?}", action);
                    app.handle(action);
                })
                .await;
        });

        context.invoke_local(move || {
            application.run(&args);
        });
    }
}
