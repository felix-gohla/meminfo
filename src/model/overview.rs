use std::sync::atomic::AtomicUsize;

#[derive(Debug, Default)]
pub struct Overview {
    /// The total number of bytes available to the system.
    pub max_ram: AtomicUsize,
}
