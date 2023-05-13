use crate::types::base::color::Color;
use crate::types::config::scene::Scene;
use std::sync::{Arc, Mutex, Condvar, RwLock};
use std::thread;
use std::thread::JoinHandle;

pub struct Renderer {
    num_threads: usize,
    pub scene: Arc<RwLock<Scene>>,
    countdown: Arc<(Mutex<usize>, Condvar)>,
    threads: Vec<Arc<(Mutex<bool>, Condvar)>>,
    handles: Vec<JoinHandle<()>>,
    colors: Arc<Mutex<Vec<Color>>>
}

impl Renderer {
    pub fn new(scene: Scene) -> Self {
        let width = scene.resolution.width;
        let height = scene.resolution.height;
        let num_threads = thread::available_parallelism().unwrap().get();
        let mut renderer = Self {
            num_threads: num_threads,
            scene: Arc::new(RwLock::new(scene)),
            countdown: Arc::new((Mutex::new(0), Condvar::new())),
            threads: Vec::with_capacity(num_threads),
            handles: Vec::with_capacity(num_threads),
            colors: Arc::new(Mutex::new(vec![Color::default(); width * height]))
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
            let num_threads = self.num_threads;
            let countdown = self.countdown.clone();
            let wait_start_thread = wait_start.clone();
            let colors_thread = self.colors.clone();
            let scene_thread = self.scene.clone();

            self.threads.push(arc);
            self.handles.push(thread::spawn(move || {
                let (mutex, cvar) = &*arc_thread;
                let scene = &*scene_thread;
                let width: usize;
                let height: usize;
                let size: usize;
                let diff: f64;
                let start: usize;
                let end: usize;
                let mut colors:Vec<Color>;

                {
                    let scene = scene.read().unwrap();
                    width = scene.resolution.width as usize;
                    height = scene.resolution.height as usize;
                    size = width * height;
                    diff = size as f64 / num_threads as f64;
                    start = (diff * thread_id as f64).floor() as usize;
                    end = (diff * (thread_id + 1) as f64).floor() as usize;
                    colors = vec![Color::default(); end - start];
                }
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

                    let scene = scene.read().unwrap();
                    for i in start..end {
                        let x = i % width;
                        let y = i / width;
                        let xf = x as f64 / width as f64;
                        let yf = y as f64 / height as f64;
                        colors[i - start] = scene.camera.ray(xf, yf).first_hit(&scene.primitives.0);
                    }

                    {
                        let mutex_colors = &*colors_thread;
                        let mut main_colors = mutex_colors.lock().unwrap();
                        for i in start..end {
                            (*main_colors)[i] = colors[i - start];
                        }
                    }

                    let (mutex_count, cvar_count) = &*countdown;
                    let mut remaining = mutex_count.lock().unwrap();
                    *remaining -= 1;
                    if *remaining == 0 {
                        cvar_count.notify_one();
                    }
                }
            }));
        }
        drop(cvar_wait.wait(remaining).unwrap());
    }

    pub fn render(&mut self) {
        let (mutex_count, cvar_count) = &*self.countdown;
        let mut remaining = mutex_count.lock().unwrap();
        *remaining = self.num_threads;

        for arc in &self.threads {
            let mutex = &arc.0;
            let cvar = &arc.1;
            drop(mutex.lock().unwrap());
            cvar.notify_one();
        }

        drop(cvar_count.wait(remaining).unwrap());
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

    pub fn get_raw_colors(&self, with_alpha: bool) -> Vec<u8> {
        let mutex = &*self.colors;
        let colors = mutex.lock().unwrap();
        let scene = self.scene.read().unwrap();
        let channels = if with_alpha { 4 } else { 3 };
        let mut bytes: Vec<u8> = Vec::with_capacity(scene.resolution.width * scene.resolution.height * channels);

        for color in colors.iter() {
            bytes.push(color.r);
            bytes.push(color.g);
            bytes.push(color.b);
            if with_alpha {
                bytes.push(255);
            }
        }
        bytes
    }
}
