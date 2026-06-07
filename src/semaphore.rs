//! Simulated counting semaphore.

/// A simulated counting semaphore.
///
/// Tracks an internal counter. `wait` decrements (blocks if zero),
/// `signal` increments.
#[derive(Debug)]
pub struct SimSemaphore {
    count: isize,
    max: isize,
    total_waits: u64,
    total_signals: u64,
}

impl SimSemaphore {
    /// Create a new semaphore with initial count and maximum.
    pub fn new(initial: isize, max: isize) -> Self {
        assert!(initial >= 0 && initial <= max);
        Self {
            count: initial,
            max,
            total_waits: 0,
            total_signals: 0,
        }
    }

    /// Binary semaphore (mutex-like).
    pub fn binary() -> Self {
        Self::new(1, 1)
    }

    /// Try to wait (P/decrement). Returns true if successful.
    pub fn try_wait(&mut self) -> bool {
        if self.count > 0 {
            self.count -= 1;
            self.total_waits += 1;
            true
        } else {
            false
        }
    }

    /// Signal (V/increment). Returns true if successful.
    pub fn signal(&mut self) -> bool {
        if self.count < self.max {
            self.count += 1;
            self.total_signals += 1;
            true
        } else {
            false
        }
    }

    /// Current count.
    pub fn count(&self) -> isize {
        self.count
    }

    /// Maximum count.
    pub fn max(&self) -> isize {
        self.max
    }

    /// Total successful waits.
    pub fn total_waits(&self) -> u64 {
        self.total_waits
    }

    /// Total successful signals.
    pub fn total_signals(&self) -> u64 {
        self.total_signals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_semaphore() {
        let mut s = SimSemaphore::binary();
        assert_eq!(s.count(), 1);
        assert!(s.try_wait());
        assert_eq!(s.count(), 0);
        assert!(!s.try_wait());
        assert!(s.signal());
        assert_eq!(s.count(), 1);
    }

    #[test]
    fn counting_semaphore() {
        let mut s = SimSemaphore::new(3, 5);
        assert!(s.try_wait());
        assert!(s.try_wait());
        assert!(s.try_wait());
        assert_eq!(s.count(), 0);
        assert!(!s.try_wait());
    }

    #[test]
    fn signal_at_max_fails() {
        let mut s = SimSemaphore::new(2, 2);
        assert!(!s.signal()); // already at max
    }

    #[test]
    fn wait_signal_cycle() {
        let mut s = SimSemaphore::new(1, 1);
        s.try_wait();
        s.signal();
        s.try_wait();
        assert_eq!(s.total_waits(), 2);
        assert_eq!(s.total_signals(), 1);
    }

    #[test]
    #[should_panic]
    fn invalid_initial_count() {
        SimSemaphore::new(-1, 5);
    }

    #[test]
    #[should_panic]
    fn initial_exceeds_max() {
        SimSemaphore::new(10, 5);
    }
}
