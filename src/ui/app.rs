use gdk_pixbuf::Pixbuf;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    AboutDialogBuilder, Application, ApplicationWindow, ApplicationWindowBuilder, LabelBuilder,
    Notebook,
};

use super::stacked_bar::StackedBar;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn icon(width: i32) -> Pixbuf {
    let icon_bytes = include_bytes!("../resources/ram-memory.png");
    let icon_stream = gio::MemoryInputStream::from_bytes(&glib::Bytes::from(icon_bytes));
    Pixbuf::from_stream_at_scale::<gio::MemoryInputStream, gio::Cancellable>(
        &icon_stream,
        width,
        width,
        true,
        None,
    )
    .expect("Icon should be convertible to Pixbuf")
}

fn build_window(app: &Application) -> ApplicationWindow {
    ApplicationWindowBuilder::new()
        .application(app)
        .title("MemInfo")
        .icon(&icon(128))
        .show_menubar(true)
        .width_request(640)
        .height_request(480)
        .default_width(640)
        .default_height(480)
        .build()
}

fn build_menubar(app: &Application) {
    let menu = gio::Menu::new();
    let menu_bar = gio::Menu::new();
    let more_menu = gio::Menu::new();
    let settings_menu = gio::Menu::new();
    let submenu = gio::Menu::new();

    // The first argument is the label of the menu item whereas the second is the action name. It'll
    // makes more sense when you'll be reading the "add_actions" function.
    menu.append(Some("Quit"), Some("app.quit"));

    settings_menu.append(Some("Sub another"), Some("app.sub_another"));
    submenu.append(Some("Sub sub another"), Some("app.sub_sub_another"));
    submenu.append(Some("Sub sub another2"), Some("app.sub_sub_another2"));
    settings_menu.append_submenu(Some("Sub menu"), &submenu);
    menu_bar.append_submenu(Some("_Another"), &settings_menu);

    more_menu.append(Some("About"), Some("app.about"));
    menu_bar.append_submenu(Some("?"), &more_menu);

    app.set_app_menu(Some(&menu));
    app.set_menubar(Some(&menu_bar));
}

fn build_notebook(box_: &gtk::Box) {
    let notebook = Notebook::new();

    let overview_label = LabelBuilder::new().label("Overview").build();
    notebook.append_page(&build_overview_page(), Some(&overview_label));
    notebook.show_all();

    box_.pack_start(&notebook, true, true, 0);
}

fn build_overview_page() -> gtk::Box {
    let page = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let sb = StackedBar::new(6);
    sb.set_property_height_request(64);
    page.pack_start(&sb, false, true, 0);

    page
}

fn show_about(window: &ApplicationWindow) {
    let icon_buf = icon(64);

    let p = AboutDialogBuilder::new()
        .title("About MemInfo")
        .authors(vec!["Felix Gohla (HPI)".to_string()])
        .copyright("Copyright © 2021 by Felix Gohla")
        .version(VERSION)
        .icon(&icon_buf)
        .logo(&icon_buf)
        .website("https://osm.hpi.de/")
        .website_label("Operating Systems And Middleware")
        .transient_for(window)
        .build();
    p.show_all();
}

fn add_actions(app: &Application, window: &ApplicationWindow) {
    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(glib::clone!(@weak window => move |_, _| {
        window.close();
    }));

    let about = gio::SimpleAction::new("about", None);
    about.connect_activate(glib::clone!(@weak window => move |_, _| {
        show_about(&window);
    }));

    // We need to add all the actions to the application so they can be taken into account.
    app.add_action(&about);
    app.add_action(&quit);
}

fn build_ui(app: &Application) {
    let window = build_window(app);

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    build_notebook(&v_box);
    window.add(&v_box);

    build_menubar(&app);
    add_actions(&app, &window);

    window.show_all();
}

fn add_accelerators(app: &Application) {
    app.set_accels_for_action("app.about", &["F1"]);
    app.set_accels_for_action("app.quit", &["<Primary>Q"]);
}

pub fn create_application() -> Application {
    let application = Application::new(Some("de.hpi.felix-gohla.meminfo"), Default::default())
        .expect("Application::new failed");

    application.connect_startup(|app| {
        add_accelerators(app);
    });
    application.connect_activate(|app| {
        build_ui(app);
    });
    application
}
