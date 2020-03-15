pub mod condvar;
pub mod mutex;

pub use mutex::{Mutex as SleepLock, MutexGuard as SleepGuardLock};
