use crate::types::base::color::Color;
use crate::types::config::scene::Scene;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::thread::JoinHandle;

pub struct Renderer {
    num_threads: usize,
    countdown: Arc<(Mutex<usize>, Condvar)>,
    threads: Vec<Arc<(Mutex<bool>, Condvar)>>,
    handles: Vec<JoinHandle<()>>,
    pub colors: Vec<Color>
}

impl Renderer {
    pub fn new(scene: &Scene) -> Self {
        let num_threads = thread::available_parallelism().unwrap().get();
        let mut renderer = Self {
            num_threads: num_threads,
            countdown: Arc::new((Mutex::new(0), Condvar::new())),
            threads: Vec::with_capacity(num_threads),
            handles: Vec::with_capacity(num_threads),
            colors: Vec::with_capacity(scene.resolution.width * scene.resolution.height)
        };
        renderer.create_threads();
        renderer
    }

    fn create_threads(&mut self) {
        let wait_start = Arc::new((Mutex::new(self.num_threads), Condvar::new()));
        let (mutex_wait, cvar_wait) = &*wait_start;
        let remaining = mutex_wait.lock().unwrap();

        for i in 0..self.num_threads {
            let arc = Arc::new((Mutex::new(true), Condvar::new()));
            let arc_thread = arc.clone();
            let thread_id = i;
            let num_threads = self.threads.len();
            let countdown = self.countdown.clone();
            let wait_start_thread = wait_start.clone();

            self.threads.push(arc);
            let handle = thread::spawn(move || {
                let (mutex, cvar) = &*arc_thread;

                loop {
                    let mut running = mutex.lock().unwrap();
                    {
                        let (mutex_wait, cvar_wait) = &*wait_start_thread;
                        let mut remaining = mutex_wait.lock().unwrap();
                        *remaining -= 1;
                        if *remaining == 0 {
                            cvar_wait.notify_one();
                        }
                    }
                    running = cvar.wait(running).unwrap();
                    if !*running {
                        break;
                    }

                    println!("thread_id: {} num_threads: {}", thread_id, num_threads);

                    let (mutex_count, cvar_count) = &*countdown;
                    let mut remaining = mutex_count.lock().unwrap();
                    *remaining -= 1;
                    if *remaining == 0 {
                        cvar_count.notify_one();
                    }
                }

                println!("Thread {} ended", thread_id);
            });
            self.handles.push(handle);
        }
        _ = cvar_wait.wait(remaining).unwrap();
    }

    pub fn render(&mut self) {
        println!("render begin");

        let (mutex_count, cvar_count) = &*self.countdown;
        let mut remaining = mutex_count.lock().unwrap();
        *remaining = self.num_threads;

        self.colors.clear();
        for arc in &self.threads {
            let mutex = &arc.0;
            let cvar = &arc.1;
            _ = mutex.lock().unwrap();
            cvar.notify_one();
        }

        _ = cvar_count.wait(remaining).unwrap();

        println!("render end");
    }

    pub fn terminate(&mut self) {
        for arc in &mut self.threads.drain(..) {
            let mutex = &arc.0;
            let cvar = &arc.1;
            let mut running = mutex.lock().unwrap();
            *running = false;
            cvar.notify_one();
        }
        for handle in &mut self.handles.drain(..) {
            handle.join().unwrap();
        }
    }
}
