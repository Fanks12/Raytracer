use std::error;
use std::sync::Arc;
use crate::core::graphical::run_graphical;
use crate::core::image::write_ppm_file;
use crate::core::options::OptsArgs;
use crate::core::parser::parse_file;
use crate::core::renderer::Renderer;
use crate::types::config::scene::Scene;

pub fn raytrace(options: OptsArgs) -> Result<(), Box<dyn error::Error>> {
    let result = parse_file(&options.scene_file);
    match result {
        Ok(_) => exec_raytracing(options, result.unwrap()),
        Err(err) => Err(err),
    }
}

fn exec_raytracing(options: OptsArgs, scene: Scene) -> Result<(), Box<dyn error::Error>> {
    let points_x = scene.resolution.width;
    let points_y = scene.resolution.height;
    let scene = Arc::new(scene);
    let mut renderer = Renderer::new(&scene);

    if options.external_file.is_some() {
        renderer.render();
        let bytes = renderer.get_raw_colors();
        renderer.terminate();
        return write_ppm_file(&options.external_file.unwrap(), points_x, points_y, bytes);
    }
    run_graphical(renderer);
    Ok(())
}
