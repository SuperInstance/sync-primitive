# sync-primitive

Userspace synchronization primitive simulator for research and education.

Implements simulated sync primitives that model the behavior of real OS-level
synchronization without requiring actual threads:

- **Mutex** — Mutual exclusion with lock/unlock
- **Semaphore** — Counting semaphore with wait/signal
- **RWLock** — Readers-writer lock with shared/exclusive access
- **Barrier** — N-thread barrier synchronization
- **Spinlock** — Busy-wait lock simulation

## Usage

```rust
use sync_primitive::mutex::SimMutex;
use sync_primitive::semaphore::SimSemaphore;

let mut mtx = SimMutex::new();
assert!(mtx.try_lock()); // succeeds
assert!(!mtx.try_lock()); // already locked
mtx.unlock();
```

These are simulation types for educational purposes — they model the state
machine of each primitive without actual thread synchronization.

No external dependencies — pure `std`.

## License

MIT
