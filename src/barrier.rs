//! Simulated barrier synchronization.

/// A simulated N-thread barrier.
///
/// Threads "arrive" at the barrier. When all N have arrived, the barrier
/// releases everyone and can be reused.
#[derive(Debug)]
pub struct SimBarrier {
    capacity: usize,
    waiting: usize,
    generation: u64,
    total_arrivals: u64,
    total_releases: u64,
}

impl SimBarrier {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            waiting: 0,
            generation: 0,
            total_arrivals: 0,
            total_releases: 0,
        }
    }

    /// A thread arrives. Returns `true` if this arrival triggers a release.
    pub fn arrive(&mut self) -> bool {
        self.waiting += 1;
        self.total_arrivals += 1;
        if self.waiting >= self.capacity {
            self.waiting = 0;
            self.generation += 1;
            self.total_releases += 1;
            true
        } else {
            false
        }
    }

    /// Current generation (number of times barrier has been released).
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Number of threads currently waiting.
    pub fn waiting(&self) -> usize {
        self.waiting
    }

    /// Barrier capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Total arrivals across all generations.
    pub fn total_arrivals(&self) -> u64 {
        self.total_arrivals
    }

    /// Total times the barrier has released.
    pub fn total_releases(&self) -> u64 {
        self.total_releases
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_barrier() {
        let b = SimBarrier::new(3);
        assert_eq!(b.capacity(), 3);
        assert_eq!(b.waiting(), 0);
        assert_eq!(b.generation(), 0);
    }

    #[test]
    fn partial_arrival_no_release() {
        let mut b = SimBarrier::new(3);
        assert!(!b.arrive());
        assert!(!b.arrive());
        assert_eq!(b.waiting(), 2);
    }

    #[test]
    fn full_arrival_triggers_release() {
        let mut b = SimBarrier::new(3);
        b.arrive();
        b.arrive();
        assert!(b.arrive()); // triggers
        assert_eq!(b.waiting(), 0);
        assert_eq!(b.generation(), 1);
    }

    #[test]
    fn reusable_after_release() {
        let mut b = SimBarrier::new(2);
        b.arrive();
        assert!(b.arrive());
        // Second generation
        assert!(!b.arrive());
        assert!(b.arrive());
        assert_eq!(b.generation(), 2);
    }

    #[test]
    fn single_thread_barrier() {
        let mut b = SimBarrier::new(1);
        assert!(b.arrive()); // immediate release
        assert!(b.arrive()); // immediate again
        assert_eq!(b.generation(), 2);
    }

    #[test]
    fn counters() {
        let mut b = SimBarrier::new(2);
        b.arrive();
        b.arrive();
        b.arrive();
        assert_eq!(b.total_arrivals(), 3);
        assert_eq!(b.total_releases(), 1);
    }
}
