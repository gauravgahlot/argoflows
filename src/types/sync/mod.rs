use serde::{Deserialize, Serialize};

/// `Synchronization` holds synchronization lock configuration.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Synchronization {
    #[serde(rename = "mutex", skip_serializing_if = "Option::is_none")]
    pub mutex: Option<Box<Mutex>>,

    #[serde(rename = "semaphore", skip_serializing_if = "Option::is_none")]
    pub semaphore: Option<Box<SemaphoreRef>>,
}

impl Synchronization {
    pub fn new() -> Self {
        Synchronization {
            ..Default::default()
        }
    }
}

mod mutex;
pub use self::mutex::Mutex;

mod mutex_holding;
pub use self::mutex_holding::MutexHolding;

mod mutex_status;
pub use self::mutex_status::MutexStatus;

mod semaphore_ref;
pub use self::semaphore_ref::SemaphoreRef;

mod semaphore_holding;
pub use self::semaphore_holding::SemaphoreHolding;

mod semaphore_status;
pub use self::semaphore_status::SemaphoreStatus;

mod synchronization_status;
pub use self::synchronization_status::SynchronizationStatus;
