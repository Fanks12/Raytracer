use crate::types::base::color::Color;
use crate::types::config::scene::Scene;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::thread::JoinHandle;

pub struct Renderer {
    num_threads: usize,
    scene: Arc<Scene>,
    countdown: Arc<(Mutex<usize>, Condvar)>,
    threads: Vec<Arc<(Mutex<bool>, Condvar)>>,
    handles: Vec<JoinHandle<()>>,
    pub colors: Arc<Mutex<Vec<Color>>>
}

impl Renderer {
    pub fn new(scene: &Arc<Scene>) -> Self {
        let num_threads = thread::available_parallelism().unwrap().get();
        let mut renderer = Self {
            num_threads: num_threads,
            scene: scene.clone(),
            countdown: Arc::new((Mutex::new(0), Condvar::new())),
            threads: Vec::with_capacity(num_threads),
            handles: Vec::with_capacity(num_threads),
            colors: Arc::new(Mutex::new(vec![Color::default(); scene.resolution.width * scene.resolution.height]))
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
                let width = scene.resolution.width as usize;
                let height = scene.resolution.height as usize;
                let size = width * height;
                let diff = size as f64 / num_threads as f64;
                let start = (diff * thread_id as f64).floor() as usize;
                let end = (diff * (thread_id + 1) as f64).floor() as usize;
                let mut colors = vec![Color::default(); end - start];

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

                    for i in start..end {
                        let x = i % width;
                        let y = i / width;
                        let xf = x as f64 / width as f64;
                        let yf = y as f64 / height as f64;
                        colors[i - start] = scene.camera.ray(xf, yf).first_hit(&scene.primitives);
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
        _ = cvar_wait.wait(remaining).unwrap();
    }

    pub fn render(&mut self) {
        let (mutex_count, cvar_count) = &*self.countdown;
        let mut remaining = mutex_count.lock().unwrap();
        *remaining = self.num_threads;

        for arc in &self.threads {
            let mutex = &arc.0;
            let cvar = &arc.1;
            _ = mutex.lock().unwrap();
            cvar.notify_one();
        }

        _ = cvar_count.wait(remaining).unwrap();
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

    pub fn get_color(&self, x: usize, y: usize) -> Color {
        let mutex = &*self.colors;
        let colors = mutex.lock().unwrap();
        (&(*colors)[(*self.scene).resolution.width * y + x]).clone()
    }

    pub fn get_colors(&self) -> Vec<Color> {
        let mutex = &*self.colors;
        let colors = mutex.lock().unwrap();
        (&*colors).clone()
    }
}
