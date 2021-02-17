use super::StackedBar;
use crate::model::Overview;
use gtk::prelude::*;
use gtk::{Label, LabelBuilder, Orientation};
use std::sync::atomic::Ordering;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct OverviewPage {
    box_: gtk::Box,
    label: Label,
    overview: Arc<Overview>,
}

impl OverviewPage {
    pub fn new(overview: Arc<Overview>) -> Self {
        let container = gtk::Box::new(Orientation::Vertical, 0);
        let label = LabelBuilder::new()
            .name("max-ram")
            .label(&format!("{}", overview.max_ram.load(Ordering::Relaxed)))
            .build();
        container.pack_start(&label, false, false, 0);

        let sb = StackedBar::new(6);
        sb.set_property_height_request(64);
        container.pack_start(&sb, false, true, 0);

        OverviewPage {
            box_: container,
            label,
            overview,
        }
    }

    pub fn page(&self) -> &gtk::Box {
        &self.box_
    }

    pub fn update(&self) {
        self.label.set_text(&format!(
            "{}",
            self.overview.max_ram.load(Ordering::Relaxed)
        ));
    }
}
