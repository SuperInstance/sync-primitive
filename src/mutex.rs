//! Simulated mutex (mutual exclusion lock).

/// A simulated mutex that tracks lock state without real threads.
///
/// Models the state machine: unlocked ↔ locked.
#[derive(Debug)]
pub struct SimMutex {
    locked: bool,
    owner: Option<u64>,
    lock_count: u64,
    unlock_count: u64,
}

impl SimMutex {
    pub fn new() -> Self {
        Self {
            locked: false,
            owner: None,
            lock_count: 0,
            unlock_count: 0,
        }
    }

    /// Try to acquire the lock. Returns true if successful.
    pub fn try_lock(&mut self) -> bool {
        if self.locked {
            false
        } else {
            self.locked = true;
            self.lock_count += 1;
            true
        }
    }

    /// Try to acquire the lock with a thread ID.
    pub fn try_lock_with(&mut self, thread_id: u64) -> bool {
        if self.locked {
            false
        } else {
            self.locked = true;
            self.owner = Some(thread_id);
            self.lock_count += 1;
            true
        }
    }

    /// Unlock the mutex. Returns true if it was locked.
    pub fn unlock(&mut self) -> bool {
        if self.locked {
            self.locked = false;
            self.owner = None;
            self.unlock_count += 1;
            true
        } else {
            false
        }
    }

    /// Check if locked.
    pub fn is_locked(&self) -> bool {
        self.locked
    }

    /// Get the owner thread ID, if locked.
    pub fn owner(&self) -> Option<u64> {
        self.owner
    }

    /// Total successful locks.
    pub fn lock_count(&self) -> u64 {
        self.lock_count
    }

    /// Total unlocks.
    pub fn unlock_count(&self) -> u64 {
        self.unlock_count
    }
}

impl Default for SimMutex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_unlocked() {
        let m = SimMutex::new();
        assert!(!m.is_locked());
        assert!(m.owner().is_none());
    }

    #[test]
    fn lock_unlock_cycle() {
        let mut m = SimMutex::new();
        assert!(m.try_lock());
        assert!(m.is_locked());
        assert!(m.unlock());
        assert!(!m.is_locked());
    }

    #[test]
    fn double_lock_fails() {
        let mut m = SimMutex::new();
        m.try_lock();
        assert!(!m.try_lock());
    }

    #[test]
    fn unlock_when_unlocked_fails() {
        let mut m = SimMutex::new();
        assert!(!m.unlock());
    }

    #[test]
    fn owner_tracking() {
        let mut m = SimMutex::new();
        assert!(m.try_lock_with(42));
        assert_eq!(m.owner(), Some(42));
        m.unlock();
        assert!(m.owner().is_none());
    }

    #[test]
    fn counters() {
        let mut m = SimMutex::new();
        m.try_lock();
        m.unlock();
        m.try_lock();
        m.unlock();
        assert_eq!(m.lock_count(), 2);
        assert_eq!(m.unlock_count(), 2);
    }
}
