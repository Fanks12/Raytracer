use nannou::{prelude::*, wgpu::Texture, image::{DynamicImage, RgbImage}};
use std::sync::{Arc, Mutex};
use crate::core::renderer::Renderer;

struct Model { renderer: Arc<Mutex<Renderer>> }

pub fn run_graphical(renderer: Renderer) {
    let renderer = Arc::new(Mutex::new(renderer));
    let renderer2 = renderer.clone();
    let width: u32;
    let height: u32;

    {
        let renderer = renderer.lock().unwrap();
        let scene = renderer.scene.read().unwrap();
        width = scene.resolution.width as u32;
        height = scene.resolution.height as u32;
    }
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

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id, simple } => {
            if let Some(event) = simple {
                window_event(app, model, id, event);
            }
        },
        _ => ()
    }
}

fn window_event(app: &App, model: &mut Model, _id: WindowId, event: WindowEvent) {
    match event {
        Closed => app.quit(),
        KeyPressed(key) => {
            match key {
                Key::Z => model.renderer.lock().unwrap().scene.write().unwrap().camera.forward(),
                Key::S => model.renderer.lock().unwrap().scene.write().unwrap().camera.backward(),
                Key::D => model.renderer.lock().unwrap().scene.write().unwrap().camera.right(),
                Key::Q => model.renderer.lock().unwrap().scene.write().unwrap().camera.left(),
                Key::Space => model.renderer.lock().unwrap().scene.write().unwrap().camera.up(),
                Key::LShift => model.renderer.lock().unwrap().scene.write().unwrap().camera.down(),
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
    let scene = renderer.scene.read().unwrap();
    let width: u32 = scene.resolution.width as u32;
    let height: u32 = scene.resolution.height as u32;
    let image = RgbImage::from_raw(width, height, renderer.get_raw_colors()).unwrap();
    let texture = Texture::from_image(&app.main_window(), &DynamicImage::ImageRgb8(image));

    draw.texture(&texture);
    draw.to_frame(app, &frame).unwrap();
}
