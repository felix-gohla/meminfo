use gtk::prelude::*;
use gtk::{ApplicationWindow, ButtonsType, MessageDialogBuilder, MessageType, ResponseType};

pub(crate) fn display_no_root_dialog(window: &ApplicationWindow) -> bool {
    let dialog = MessageDialogBuilder::new()
        .name("no-root-dialog")
        .text("No root privileges")
        .secondary_text("Meminfo needs root privileges in order to query all necessary information from the proc-filesystem.")
        .modal(true)
        .attached_to(window)
        .message_type(MessageType::Error)
        .buttons(ButtonsType::YesNo)
        .build();
    let response = dialog.run();
    dialog.emit_close();
    match response {
        ResponseType::No => false,
        ResponseType::Yes => true,
        _ => unimplemented!("This is an invalid answer from the no-root-dialog."),
    }
}
