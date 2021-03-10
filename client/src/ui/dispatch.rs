use super::AppAction;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::StreamExt;

#[derive(Debug)]
pub struct DispatchLoop {
    receiver: UnboundedReceiver<AppAction>,
    sender: UnboundedSender<AppAction>,
}

impl DispatchLoop {
    pub fn new() -> Self {
        let (sender, receiver) = futures::channel::mpsc::unbounded();
        Self { receiver, sender }
    }

    /// Create a new dispatcher for sending messages to the app.
    pub fn make_dispatcher(&self) -> UnboundedSender<AppAction> {
        self.sender.clone()
    }

    pub async fn attach(self, mut handler: impl FnMut(AppAction)) {
        self.receiver
            .for_each(|action| {
                let result = handler(action);
                async move { result }
            })
            .await;
    }
}
