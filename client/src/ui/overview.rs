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
        let label = LabelBuilder::new().name("max-ram").label("---").build();
        container.pack_start(&label, false, false, 0);

        let sb = StackedBar::new(2);
        sb.set_property_height_request(48);
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
        let free = self.overview.ram_free.load(Ordering::Relaxed);
        let total = self.overview.ram_total.load(Ordering::Relaxed);
        self.label.set_text(&format!(
            "{} / {} ({:.2}%)",
            total - free,
            total,
            (total - free) as f64 * 100f64 / total as f64,
        ));
    }
}
