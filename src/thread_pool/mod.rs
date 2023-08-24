use std::{thread, sync::mpsc::{self, Receiver, Sender}, time::Duration};

pub mod explanation;

type Job = Box<dyn FnOnce() + 'static + Send>;

struct Worker {
    to_thread: Sender<Job>,
    from_thread: Receiver<()>,
}

impl Worker {
    fn new() -> Worker {
        let (to_thread, input) = mpsc::channel();
        let (output, from_thread) = mpsc::channel();

        thread::spawn(move || {
            loop {
                let f: Job = input.recv().unwrap();
                f();
                output.send(()).unwrap();
            }
        });

        Worker {
            to_thread,
            from_thread
        }
    }

    fn give_job<F>(&mut self, task: F)  where
        F: FnOnce() + 'static + Send
    {
        self.to_thread.send(Box::new(task)).unwrap();
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    busy_workers: Vec<Worker>
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> ThreadPool {
        assert!(num_threads > 0);

        let workers = (0..num_threads).into_iter().map(|_| Worker::new()).collect();

        ThreadPool { workers, busy_workers: Vec::with_capacity(num_threads) }
    }

    pub fn enter<F>(&mut self, task: F) where
        F: FnOnce() + 'static + Send  
    {
        let mut worker = self.get_worker();

        worker.give_job(task);

        self.busy_workers.push(worker);
    }

    fn refresh(&mut self) {
        let free_indicies: Vec<usize> = self.busy_workers.iter().enumerate().filter_map(|(i, worker)| {
            match worker.from_thread.try_recv() {
                Ok(_) => Some(i),
                _ => None,
            }
        }).rev().collect();

        for i in free_indicies {
            let worker = self.busy_workers.remove(i);
            self.workers.push(worker);
        }
    }

    fn try_get_worker(&mut self) -> Option<Worker> {
        self.refresh();
        self.workers.pop()
    }
    
    fn get_worker(&mut self) -> Worker {
        loop {
            match self.try_get_worker() {
                Some(worker) => return worker,
                None => thread::sleep(Duration::from_millis(100)),
            }
        }
    }
}
