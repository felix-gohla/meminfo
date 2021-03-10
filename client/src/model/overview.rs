use std::sync::atomic::AtomicUsize;

#[derive(Debug, Default)]
pub struct Overview {
    /// The total number of bytes available to the system.
    pub ram_total: AtomicUsize,
    /// The number of bytes being free and not used by the system.
    pub ram_free: AtomicUsize,
}
