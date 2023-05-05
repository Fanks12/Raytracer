use crate::types::base::color::Color;
use crate::types::config::scene::Scene;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::thread::JoinHandle;

pub struct Renderer {
    threads: Vec<Arc<(Mutex<bool>, Condvar)>>,
    handles: Vec<JoinHandle<()>>,
    pub colors: Vec<Color>
}

impl Renderer {
    pub fn new(scene: &Scene) -> Self {
        let number_of_threads: usize = thread::available_parallelism().unwrap().get();
        let mut threads: Vec<Arc<(Mutex<bool>, Condvar)>> = Vec::with_capacity(number_of_threads);
        let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(number_of_threads);

        Renderer::create_threads(number_of_threads, &mut threads, &mut handles);

        Self { colors: Vec::with_capacity(scene.resolution.width * scene.resolution.height), threads: threads, handles: handles }
    }

    fn create_threads(number_of_threads:usize, threads: &mut Vec<Arc<(Mutex<bool>, Condvar)>>, handles: &mut Vec<JoinHandle<()>>) {
        for i in 0..number_of_threads {
            let arc = Arc::new((Mutex::new(true), Condvar::new()));
            let arc_thread = arc.clone();
            let thread_id = i;
            let num_threads = threads.len();

            threads.push(arc);
            let handle = thread::spawn(move || {
                let (mutex, cvar) = &*arc_thread;

                loop {
                    let mut running = mutex.lock().unwrap();
                    running = cvar.wait(running).unwrap();
                    if !*running {
                        break;
                    }
                    println!("thread_id: {} number_of_threads: {}", thread_id, num_threads);
                }

                println!("Thread {} ended", thread_id);
            });
            handles.push(handle);
        }
    }

    pub fn render(&mut self) {
        self.colors.clear();
        for arc in &self.threads {
            let mutex = &arc.0;
            let cvar = &arc.1;
            _ = mutex.lock().unwrap();
            cvar.notify_one();
        }
    }

    pub fn terminate(&mut self) {
        for arc in &self.threads {
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
