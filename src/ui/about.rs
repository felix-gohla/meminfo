use glib::object::IsA;
use gtk::prelude::*;
use gtk::{AboutDialogBuilder, Window};
use std::fmt;

use super::icon::icon;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone)]
pub struct AboutDialog {
    dialog: gtk::AboutDialog,
}

impl AboutDialog {
    pub fn new<T: IsA<Window>>(window: &T) -> Self {
        let icon_buf = icon(64);

        let dialog = AboutDialogBuilder::new()
            .title("About MemInfo")
            .authors(vec!["Felix Gohla (HPI)".to_string()])
            .copyright("Copyright © 2021 by Felix Gohla")
            .version(VERSION)
            .icon(&icon_buf)
            .logo(&icon_buf)
            .website("https://osm.hpi.de/")
            .website_label("Operating Systems And Middleware")
            .transient_for(window)
            .modal(true)
            .build();
        dialog.connect_delete_event(|dlg, _| dlg.hide_on_delete());
        AboutDialog { dialog }
    }

    pub fn show(&self) {
        let dialog = &self.dialog;
        dialog.show_all();
    }
}

impl fmt::Debug for AboutDialog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AboutDialog").finish()
    }
}
