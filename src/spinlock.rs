//! Simulated spinlock.

/// A simulated spinlock.
///
/// Models the busy-wait behavior: `try_lock` attempts acquisition,
/// `spin_lock` simulates spinning until acquired.
#[derive(Debug)]
pub struct SimSpinlock {
    locked: bool,
    spin_count: u64,
    total_acquisitions: u64,
}

impl SimSpinlock {
    pub fn new() -> Self {
        Self {
            locked: false,
            spin_count: 0,
            total_acquisitions: 0,
        }
    }

    /// Try to acquire the lock. Returns true if successful.
    pub fn try_lock(&mut self) -> bool {
        if self.locked {
            self.spin_count += 1;
            false
        } else {
            self.locked = true;
            self.total_acquisitions += 1;
            true
        }
    }

    /// Simulate spinning until the lock is acquired.
    /// Call `unlock` from an external "thread" between retries.
    /// Returns the number of spins before acquisition.
    pub fn spin_lock(&mut self) -> u64 {
        let mut spins = 0;
        while self.locked {
            spins += 1;
            self.spin_count += 1;
            // In simulation, external code should call unlock() between attempts
            // For testing, we just break after counting
            if spins > 1000 {
                break; // prevent infinite loop in tests
            }
        }
        if !self.locked {
            self.locked = true;
            self.total_acquisitions += 1;
        }
        spins
    }

    /// Unlock the spinlock.
    pub fn unlock(&mut self) {
        self.locked = false;
    }

    /// Is the spinlock currently locked?
    pub fn is_locked(&self) -> bool {
        self.locked
    }

    /// Total number of spin iterations.
    pub fn spin_count(&self) -> u64 {
        self.spin_count
    }

    /// Total successful acquisitions.
    pub fn total_acquisitions(&self) -> u64 {
        self.total_acquisitions
    }
}

impl Default for SimSpinlock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_unlocked() {
        let s = SimSpinlock::new();
        assert!(!s.is_locked());
    }

    #[test]
    fn lock_unlock() {
        let mut s = SimSpinlock::new();
        assert!(s.try_lock());
        assert!(s.is_locked());
        s.unlock();
        assert!(!s.is_locked());
    }

    #[test]
    fn contention_spins() {
        let mut s = SimSpinlock::new();
        s.try_lock(); // lock it
        assert!(!s.try_lock()); // spin attempt fails
        assert_eq!(s.spin_count(), 1);
        assert!(!s.try_lock()); // another spin
        assert_eq!(s.spin_count(), 2);
    }

    #[test]
    fn spin_lock_immediate() {
        let mut s = SimSpinlock::new();
        let spins = s.spin_lock(); // unlocked, should be 0 spins
        assert_eq!(spins, 0);
        assert!(s.is_locked());
    }

    #[test]
    fn unlock_reacquire() {
        let mut s = SimSpinlock::new();
        s.try_lock();
        s.unlock();
        assert!(s.try_lock());
        assert_eq!(s.total_acquisitions(), 2);
    }
}
