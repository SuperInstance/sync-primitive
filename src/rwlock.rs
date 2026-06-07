//! Simulated readers-writer lock.

/// Lock state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RwState {
    Unlocked,
    ReadLock(usize), // number of readers
    WriteLock,       // exclusive writer
}

/// A simulated RWLock.
///
/// Allows multiple concurrent readers OR a single exclusive writer.
#[derive(Debug)]
pub struct SimRwLock {
    state: RwState,
    total_read_locks: u64,
    total_write_locks: u64,
}

impl SimRwLock {
    pub fn new() -> Self {
        Self {
            state: RwState::Unlocked,
            total_read_locks: 0,
            total_write_locks: 0,
        }
    }

    /// Try to acquire a read lock. Succeeds if not write-locked.
    pub fn try_read_lock(&mut self) -> bool {
        match self.state {
            RwState::Unlocked => {
                self.state = RwState::ReadLock(1);
                self.total_read_locks += 1;
                true
            }
            RwState::ReadLock(n) => {
                self.state = RwState::ReadLock(n + 1);
                self.total_read_locks += 1;
                true
            }
            RwState::WriteLock => false,
        }
    }

    /// Release a read lock. Returns true if successful.
    pub fn read_unlock(&mut self) -> bool {
        match self.state {
            RwState::ReadLock(1) => {
                self.state = RwState::Unlocked;
                true
            }
            RwState::ReadLock(n) if n > 1 => {
                self.state = RwState::ReadLock(n - 1);
                true
            }
            _ => false,
        }
    }

    /// Try to acquire a write lock. Succeeds only if unlocked.
    pub fn try_write_lock(&mut self) -> bool {
        match self.state {
            RwState::Unlocked => {
                self.state = RwState::WriteLock;
                self.total_write_locks += 1;
                true
            }
            _ => false,
        }
    }

    /// Release a write lock.
    pub fn write_unlock(&mut self) -> bool {
        if self.state == RwState::WriteLock {
            self.state = RwState::Unlocked;
            true
        } else {
            false
        }
    }

    /// Current state.
    pub fn state(&self) -> RwState {
        self.state
    }

    /// Number of active readers (0 if not read-locked).
    pub fn reader_count(&self) -> usize {
        match self.state {
            RwState::ReadLock(n) => n,
            _ => 0,
        }
    }

    /// Total read locks acquired.
    pub fn total_read_locks(&self) -> u64 {
        self.total_read_locks
    }

    /// Total write locks acquired.
    pub fn total_write_locks(&self) -> u64 {
        self.total_write_locks
    }
}

impl Default for SimRwLock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_unlocked() {
        let rw = SimRwLock::new();
        assert_eq!(rw.state(), RwState::Unlocked);
    }

    #[test]
    fn multiple_readers() {
        let mut rw = SimRwLock::new();
        assert!(rw.try_read_lock());
        assert!(rw.try_read_lock());
        assert_eq!(rw.reader_count(), 2);
        assert!(rw.read_unlock());
        assert_eq!(rw.reader_count(), 1);
        assert!(rw.read_unlock());
        assert_eq!(rw.state(), RwState::Unlocked);
    }

    #[test]
    fn write_blocks_reads() {
        let mut rw = SimRwLock::new();
        rw.try_write_lock();
        assert!(!rw.try_read_lock());
    }

    #[test]
    fn reads_block_writes() {
        let mut rw = SimRwLock::new();
        rw.try_read_lock();
        assert!(!rw.try_write_lock());
    }

    #[test]
    fn write_unlock_then_read() {
        let mut rw = SimRwLock::new();
        rw.try_write_lock();
        rw.write_unlock();
        assert!(rw.try_read_lock());
    }

    #[test]
    fn counters() {
        let mut rw = SimRwLock::new();
        rw.try_read_lock();
        rw.try_read_lock();
        rw.read_unlock();
        rw.read_unlock();
        rw.try_write_lock();
        assert_eq!(rw.total_read_locks(), 2);
        assert_eq!(rw.total_write_locks(), 1);
    }
}
