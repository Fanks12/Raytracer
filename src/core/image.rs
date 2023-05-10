use std::error::Error;
use std::fs::File;
use std::io::Write;
use crate::types::base::color::Color;

pub fn write_ppm_file(file_name: &String, width: usize, height: usize, colors: Vec<Vec<Color>>) -> Result<(), Box<dyn Error>> {
    let file_open = File::create(file_name);
    let mut file_handle: File;
    match file_open {
        Ok(file) => file_handle = file,
        Err(err) => return Err(Box::new(err)),
    }

    let header_write = writeln!(file_handle, "P3 {} {} 255", width, height);
    match header_write {
        Ok(_) => (),
        Err(err) => return Err(Box::new(err)),
    }

    for line in colors {
        for color in line {
            let color_write = writeln!(file_handle, "{} {} {}", color.r, color.g, color.b);
            match color_write {
                Ok(_) => (),
                Err(err) => return Err(Box::new(err)),
            }
        }
    }
    Ok(())
}
