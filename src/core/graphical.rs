use nannou::{prelude::*, wgpu::Texture, image::{DynamicImage, RgbImage}};
use std::sync::{Arc, Mutex};
use crate::core::renderer::Renderer;

struct Model { renderer: Arc<Mutex<Renderer>> }

pub fn run_graphical(renderer: Renderer) {
    let width: u32 = renderer.scene.resolution.width as u32;
    let height: u32 = renderer.scene.resolution.height as u32;
    let renderer = Arc::new(Mutex::new(renderer));
    let renderer2 = renderer.clone();

    nannou::app(Arc::new(move |app| {
        app.new_window()
            .size(width, height)
            .build()
            .unwrap();
        Model { renderer: renderer.clone() }
    }))
        .event(event)
        .update(update)
        .view(view)
        .run();
    let mut renderer = renderer2.lock().unwrap();
    renderer.terminate();
}

fn event(app: &App, _model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id, simple } => {
            drop(id);
            match simple {
                Some(e) => {
                    match e {
                        WindowEvent::Closed => app.quit(),
                        _ => ()
                    }
                },
                _ => ()
            }
        },
        _ => ()
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut renderer = model.renderer.lock().unwrap();
    renderer.render();
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
