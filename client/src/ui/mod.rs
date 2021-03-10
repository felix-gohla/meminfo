mod about;
pub mod app;
pub mod dispatch;
mod icon;
mod no_root_dialog;
mod overview;
mod stacked_bar;

use overview::OverviewPage;
use stacked_bar::StackedBar;

/// An action that can be sent to the App's dispatch loop.
#[derive(Clone, Debug)]
pub enum AppAction {
    /// The application initialized.
    AppInit,
    /// Show a dialog that the application is not being run as root.
    ShowNoRootDialog,
    /// The values for memory information did change.
    MeminfoUpdate,
}
