use std::error::Error;
use std::fs::File;
use std::io::Write;
use crate::types::base::color::Color;

pub fn write_ppm_file(file_name: &String, width: usize, height: usize, colors: Vec<Color>) -> Result<(), Box<dyn Error>> {
    let file_open = File::create(file_name);
    let mut file_handle: File;
    match file_open {
        Ok(file) => file_handle = file,
        Err(err) => return Err(Box::new(err)),
    }

    let header_write = writeln!(file_handle, "P6 {} {} 255", width, height);
    match header_write {
        Ok(_) => (),
        Err(err) => return Err(Box::new(err)),
    }

    let mut bytes: Vec<u8> = Vec::with_capacity(colors.len() * 3);
    for color in colors {
        bytes.push(color.r);
        bytes.push(color.g);
        bytes.push(color.b);
    }

    match file_handle.write_all(&bytes) {
        Ok(_) => (),
        Err(err) => return Err(Box::new(err)),
    }
    Ok(())
}
