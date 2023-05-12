use nannou::{prelude::*, wgpu::Texture, image::{DynamicImage, RgbImage}};
use std::sync::{Arc, Mutex};
use crate::core::renderer::Renderer;

struct Model { renderer: Arc<Mutex<Renderer>> }

pub fn run_graphical(renderer: Renderer) {
    let width: u32 = renderer.scene.resolution.width as u32;
    let height: u32 = renderer.scene.resolution.height as u32;
    let renderer = Arc::new(Mutex::new(renderer));
    let renderer2 = renderer.clone();
    let running = Arc::new(Mutex::new(true));
    let running2 = running.clone();
    let render = std::thread::spawn(move || {
        loop {
            {
                let running = running.lock().unwrap();
                if !*running {
                    break;
                }
            }
            {
                let mut renderer = renderer2.lock().unwrap();
                renderer.render();
            }
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    });

    nannou::app(Arc::new(move |app| {
        app.new_window()
            .size(width, height)
            .build()
            .unwrap();
        Model { renderer: renderer.clone() }
    }))
        .event(event)
        .view(view)
        .run();
    {
        let mut running = running2.lock().unwrap();
        *running = false;
    }
    render.join().unwrap();
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let renderer = model.renderer.lock().unwrap();
    let width: u32 = renderer.scene.resolution.width as u32;
    let height: u32 = renderer.scene.resolution.height as u32;
    let image = RgbImage::from_raw(width, height, renderer.get_raw_colors()).unwrap();
    let texture = Texture::from_image(&app.main_window(), &DynamicImage::ImageRgb8(image));

    draw.texture(&texture);
    draw.to_frame(app, &frame).unwrap();
}
