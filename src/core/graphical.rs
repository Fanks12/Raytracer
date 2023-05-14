use nannou::{prelude::*, wgpu::Texture, image::{DynamicImage, RgbaImage}};
use std::collections::HashSet;
use crate::core::renderer::Renderer;
use crate::core::tracer::raytracer;

struct Model {
    renderer: Option<Renderer>,
    texture: Option<Texture>,
    keys: HashSet<Key>
}

pub fn run_raytracer() {
    nannou::app(model)
        .event(event)
        .update(update)
        .view(view)
        .run();
}

fn model(app: &App) -> Model {
    match raytracer() {
        Ok(result) => {
            match result {
                Some(renderer) => {
                    let width: u32;
                    let height: u32;

                    {
                        let scene = renderer.scene.read().unwrap();
                        width = scene.resolution.width as u32;
                        height = scene.resolution.height as u32;
                    }
                    app.new_window()
                        .size(width, height)
                        .build()
                        .unwrap();
                    let image = RgbaImage::from_raw(width, height, vec![0; (width * height * 4) as usize]).unwrap();
                    let texture = Texture::from_image(&app.main_window(), &DynamicImage::ImageRgba8(image));
                    return Model { renderer: Some(renderer), texture: Some(texture), keys: HashSet::default() }
                },
                None => ()
            }
        },
        Err(err) => {
            println!("{}", err);
        }
    }
    app.quit();
    Model { renderer: None, texture: None, keys: HashSet::default() }
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
    if let Some(renderer) = &mut model.renderer {
        match event {
            Closed => {
                renderer.terminate();
                app.quit();
            },
            KeyPressed(key) => {
                if !model.keys.contains(&key) {
                    model.keys.insert(key);
                }
            },
            KeyReleased(key) => drop(model.keys.remove(&key)),
            _ => ()
        }
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if let Some(renderer) = &mut model.renderer {
        if !model.keys.is_empty() {
            let mut scene = renderer.scene.write().unwrap();
            if model.keys.contains(&Key::Z) || model.keys.contains(&Key::Up) {
                scene.camera.forward();
            }
            if model.keys.contains(&Key::S) || model.keys.contains(&Key::Down) {
                scene.camera.backward();
            }
            if model.keys.contains(&Key::D) || model.keys.contains(&Key::Right) {
                scene.camera.right();
            }
            if model.keys.contains(&Key::Q) || model.keys.contains(&Key::Left) {
                scene.camera.left();
            }
            if model.keys.contains(&Key::Space) {
                scene.camera.up();
            }
            if model.keys.contains(&Key::LShift) {
                scene.camera.down();
            }
        }
        renderer.render();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if let Some(renderer) = &model.renderer {
        if let Some(texture) = &model.texture {
            texture.upload_data(app.main_window().device(), &mut *frame.command_encoder(), &renderer.get_raw_colors(true));
            draw.texture(&texture);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
