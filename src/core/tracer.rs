use std::error;
use std::sync::Arc;
use crate::core::image::write_ppm_file;
use crate::core::options::OptsArgs;
use crate::core::parser::parse_file;
use crate::core::renderer::Renderer;
use crate::types::config::scene::Scene;

pub fn raytrace(options: OptsArgs) -> Result<(), Box<dyn error::Error>> {
    let result = parse_file(&options.scene_file);
    match result {
        Ok(_) => start_raytracer(options, result.unwrap()),
        Err(err) => Err(err),
    }
}

fn start_raytracer(options: OptsArgs, scene: Scene) -> Result<(), Box<dyn error::Error>> {
    let scene = Arc::new(scene);
    let mut renderer = Renderer::new(&scene);
    let result = exec_raytracing(options, scene, &mut renderer);

    renderer.terminate();
    result
}

fn exec_raytracing(options: OptsArgs, scene: Arc<Scene>, renderer: &mut Renderer) -> Result<(), Box<dyn error::Error>> {
    let points_x = scene.resolution.width;
    let points_y = scene.resolution.height;

    if options.external_file.is_some() {
        renderer.render();
        return write_ppm_file(&options.external_file.unwrap(), points_x, points_y, renderer.get_colors());
    }
    // TODO: Graphical implementation
    println!("Graphic visualization not implemented yet, please use --export");
    Ok(())
}
