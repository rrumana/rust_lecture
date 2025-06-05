//! Section 7: Fearless Concurrency
//! ===============================
//! 
//! This section demonstrates Rust's concurrency features:
//! - Thread spawning and joining
//! - Message passing with channels
//! - Shared state with Arc and Mutex
//! - Async/await programming
//! - Concurrent data structures

#![allow(unused)]

/// Demo 7a: Basic Threading - Spawning and joining threads
pub fn demo_basic_threading() {
    println!("=== Demo 7a: Basic Threading ===");
    
    use std::thread;
    use std::time::Duration;
    
    // Simple thread spawn
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread: count {}", i);
            thread::sleep(Duration::from_millis(100));
        }
        "Thread completed"
    });
    
    // Main thread work
    for i in 1..=3 {
        println!("Main: count {}", i);
        thread::sleep(Duration::from_millis(150));
    }
    
    // Wait for thread to complete and get result
    let result = handle.join().unwrap();
    println!("Thread result: {}", result);
    
    // Multiple threads
    let mut handles = vec![];
    
    for i in 0..3 {
        let handle = thread::spawn(move || {
            println!("Worker thread {} starting", i);
            thread::sleep(Duration::from_millis(100 * (i + 1) as u64));
            println!("Worker thread {} finished", i);
            i * 10
        });
        handles.push(handle);
    }
    
    // Collect all results
    let results: Vec<i32> = handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    
    println!("All thread results: {:?}", results);
    println!();
}

/// Demo 7b: Message Passing - Communication between threads using channels
pub fn demo_message_passing() {
    println!("=== Demo 7b: Message Passing ===");
    
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    
    // Single producer, single consumer
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let messages = vec!["hello", "from", "the", "thread"];
        
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Receive messages
    for received in rx {
        println!("Received: {}", received);
    }
    
    // Multiple producers
    let (tx, rx) = mpsc::channel();
    
    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let message = format!("Message from thread {}", i);
            tx_clone.send(message).unwrap();
        });
    }
    
    // Drop the original sender so the receiver knows when all senders are done
    drop(tx);
    
    // Collect all messages
    let messages: Vec<String> = rx.iter().collect();
    println!("All messages: {:?}", messages);
    
    // Bounded channel (synchronous)
    let (tx, rx) = mpsc::sync_channel(2);  // Buffer size of 2
    
    let producer = thread::spawn(move || {
        for i in 1..=5 {
            println!("Sending {}", i);
            tx.send(i).unwrap();
            println!("Sent {}", i);
        }
    });
    
    thread::sleep(Duration::from_millis(500));  // Let producer get ahead
    
    let consumer = thread::spawn(move || {
        for received in rx {
            println!("Processing {}", received);
            thread::sleep(Duration::from_millis(200));  // Slow consumer
        }
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
    println!();
}

/// Demo 7c: Shared State - Using Arc and Mutex for shared data
pub fn demo_shared_state() {
    println!("=== Demo 7c: Shared State ===");
    
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    // Simple counter with mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented counter", i);
        });
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter value: {}", *counter.lock().unwrap());
    
    // Shared data structure
    use std::collections::HashMap;
    
    let shared_map = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];
    
    // Writer threads
    for i in 0..5 {
        let map = Arc::clone(&shared_map);
        let handle = thread::spawn(move || {
            let mut map = map.lock().unwrap();
            map.insert(format!("key{}", i), i * 10);
            println!("Thread {} wrote to map", i);
        });
        handles.push(handle);
    }
    
    // Reader thread
    let map_reader = Arc::clone(&shared_map);
    let reader_handle = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(100));  // Wait for writers
        let map = map_reader.lock().unwrap();
        println!("Map contents: {:?}", *map);
    });
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    reader_handle.join().unwrap();
    println!();
}

/// Demo 7d: Deadlock Prevention and Advanced Patterns
pub fn demo_advanced_concurrency() {
    println!("=== Demo 7d: Advanced Concurrency Patterns ===");
    
    use std::sync::{Arc, Mutex, RwLock};
    use std::thread;
    use std::time::Duration;
    
    // RwLock for multiple readers, single writer
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    
    // Multiple reader threads
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let data = data.read().unwrap();
            println!("Reader {}: {:?}", i, *data);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }
    
    // Single writer thread
    let data_writer = Arc::clone(&data);
    let writer_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));  // Let readers start
        let mut data = data_writer.write().unwrap();
        data.push(6);
        println!("Writer: added element");
    });
    
    for handle in handles {
        handle.join().unwrap();
    }
    writer_handle.join().unwrap();
    
    // Scoped threads (if std::thread::scope was stable)
    // This pattern ensures all spawned threads complete before the scope ends
    let mut data = vec![1, 2, 3];
    
    // Simulate scoped threads with manual joining
    let data_ptr = &mut data as *mut Vec<i32>;
    let handles: Vec<_> = (0..3).map(|i| {
        thread::spawn(move || {
            // In real scoped threads, we could safely access data
            println!("Scoped thread {} working", i);
            thread::sleep(Duration::from_millis(50));
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("All scoped threads completed, data: {:?}", data);
    
    // Work-stealing pattern simulation
    use std::sync::mpsc;
    
    let (work_tx, work_rx) = mpsc::channel();
    let work_rx = Arc::new(Mutex::new(work_rx));
    
    // Send work items
    let sender_handle = thread::spawn(move || {
        for i in 1..=20 {
            work_tx.send(i).unwrap();
        }
    });
    
    // Worker threads competing for work
    let mut worker_handles = vec![];
    for worker_id in 0..3 {
        let work_rx = Arc::clone(&work_rx);
        let handle = thread::spawn(move || {
            loop {
                let work_item = {
                    let rx = work_rx.lock().unwrap();
                    rx.try_recv()
                };
                
                match work_item {
                    Ok(item) => {
                        println!("Worker {} processing item {}", worker_id, item);
                        thread::sleep(Duration::from_millis(50));
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        thread::sleep(Duration::from_millis(10));
                        continue;
                    }
                    Err(mpsc::TryRecvError::Disconnected) => break,
                }
            }
            println!("Worker {} finished", worker_id);
        });
        worker_handles.push(handle);
    }
    
    sender_handle.join().unwrap();
    thread::sleep(Duration::from_millis(500));  // Let workers finish
    
    for handle in worker_handles {
        handle.join().unwrap();
    }
    println!();
}

/// Demo 7e: Async/Await Basics (using tokio-like patterns)
pub fn demo_async_basics() {
    println!("=== Demo 7e: Async/Await Basics ===");
    
    // Note: This is a simplified demonstration of async concepts
    // In real code, you'd use tokio or another async runtime
    
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use std::time::{Duration, Instant};
    
    // Simple future that completes after a delay
    struct DelayFuture {
        when: Instant,
    }
    
    impl DelayFuture {
        fn new(duration: Duration) -> Self {
            DelayFuture {
                when: Instant::now() + duration,
            }
        }
    }
    
    impl Future for DelayFuture {
        type Output = ();
        
        fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            if Instant::now() >= self.when {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }
    }
    
    // Simulate async function
    async fn async_task(id: u32) -> String {
        println!("Async task {} starting", id);
        
        // Simulate async work (in real code, this would be DelayFuture::new())
        std::thread::sleep(Duration::from_millis(100));
        
        println!("Async task {} completed", id);
        format!("Result from task {}", id)
    }
    
    // Since we can't easily run async code without a runtime in this demo,
    // we'll show the concepts with blocking equivalents
    println!("Simulating async tasks:");
    
    let start = Instant::now();
    
    // Sequential execution (blocking)
    let result1 = {
        println!("Task 1 starting");
        std::thread::sleep(Duration::from_millis(100));
        println!("Task 1 completed");
        "Result from task 1"
    };
    
    let result2 = {
        println!("Task 2 starting");
        std::thread::sleep(Duration::from_millis(100));
        println!("Task 2 completed");
        "Result from task 2"
    };
    
    println!("Sequential results: {}, {}", result1, result2);
    println!("Sequential time: {:?}", start.elapsed());
    
    // Concurrent execution (simulated)
    let start = Instant::now();
    let handle1 = std::thread::spawn(|| {
        println!("Concurrent task 1 starting");
        std::thread::sleep(Duration::from_millis(100));
        println!("Concurrent task 1 completed");
        "Result from concurrent task 1"
    });
    
    let handle2 = std::thread::spawn(|| {
        println!("Concurrent task 2 starting");
        std::thread::sleep(Duration::from_millis(100));
        println!("Concurrent task 2 completed");
        "Result from concurrent task 2"
    });
    
    let result1 = handle1.join().unwrap();
    let result2 = handle2.join().unwrap();
    
    println!("Concurrent results: {}, {}", result1, result2);
    println!("Concurrent time: {:?}", start.elapsed());
    println!();
}

/// Demo 7f: Thread Safety and Send/Sync Traits
pub fn demo_thread_safety() {
    println!("=== Demo 7f: Thread Safety and Send/Sync ===");
    
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::rc::Rc;
    
    // Send: types that can be transferred between threads
    // Sync: types that can be shared between threads (via &T)
    
    // Arc<T> is Send + Sync when T is Send + Sync
    let shared_data = Arc::new(Mutex::new(vec![1, 2, 3]));
    
    let handles: Vec<_> = (0..3).map(|i| {
        let data = Arc::clone(&shared_data);
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data.push(i + 10);
            println!("Thread {} modified shared data", i);
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final shared data: {:?}", *shared_data.lock().unwrap());
    
    // Rc<T> is NOT Send or Sync - this won't compile:
    // let rc_data = Rc::new(vec![1, 2, 3]);
    // thread::spawn(move || {
    //     println!("{:?}", rc_data);  // ERROR: Rc is not Send
    // });
    
    // Custom type demonstrating Send/Sync
    struct ThreadSafeCounter {
        count: Arc<Mutex<i32>>,
    }
    
    impl ThreadSafeCounter {
        fn new() -> Self {
            ThreadSafeCounter {
                count: Arc::new(Mutex::new(0)),
            }
        }
        
        fn increment(&self) {
            let mut count = self.count.lock().unwrap();
            *count += 1;
        }
        
        fn get(&self) -> i32 {
            *self.count.lock().unwrap()
        }
    }
    
    // ThreadSafeCounter is automatically Send + Sync because Arc<Mutex<T>> is
    let counter = Arc::new(ThreadSafeCounter::new());
    
    let handles: Vec<_> = (0..5).map(|_| {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {
            for _ in 0..10 {
                counter.increment();
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Thread-safe counter final value: {}", counter.get());
    
    // Demonstrating thread-local storage
    use std::cell::RefCell;
    
    thread_local! {
        static THREAD_LOCAL_DATA: RefCell<Vec<i32>> = RefCell::new(Vec::new());
    }
    
    let handles: Vec<_> = (0..3).map(|i| {
        thread::spawn(move || {
            THREAD_LOCAL_DATA.with(|data| {
                let mut data = data.borrow_mut();
                data.push(i);
                data.push(i * 10);
                println!("Thread {} local data: {:?}", i, *data);
            });
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Main thread's thread-local data is separate
    THREAD_LOCAL_DATA.with(|data| {
        let mut data = data.borrow_mut();
        data.push(999);
        println!("Main thread local data: {:?}", *data);
    });
    println!();
}

/// Run all demos in sequence
pub fn run_all_demos() {
    println!("🦀 RUST LECTURE - SECTION 7: FEARLESS CONCURRENCY 🦀");
    println!("======================================================");
    println!();
    
    demo_basic_threading();
    demo_message_passing();
    demo_shared_state();
    demo_advanced_concurrency();
    demo_async_basics();
    demo_thread_safety();
    
    println!("✅ Section 7 complete!");
    println!("💡 Key takeaway: Rust's type system prevents data races and ensures memory safety in concurrent code!");
}