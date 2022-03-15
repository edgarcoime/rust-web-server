use std::{thread, sync::{mpsc, Arc, Mutex}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<Job>();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // Must use smart thread-safe smart pointers
            // since workers have shared ownership and mutability of receiver 
            workers.push(Worker::new(
                id, 
                Arc::clone(&receiver)
            ));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        // Will fail if threads stop running but
        // we will always want our threads running to accept requests
        self.sender.send(job).unwrap(); 
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(
        id: usize, 
        receiver: Arc<Mutex<mpsc::Receiver<Job>>>
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();
            
            println!("Worker {} got a job; executing.", id);

            job();
        });
        Worker { id, thread }
    }
}
