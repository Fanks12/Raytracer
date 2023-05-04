use std::fs::File;
use std::io::Write;
use crate::types::base::color::Color;

fn write_ppm_file(file_name: &String, width: usize, height: usize, colors: Vec<Color>) -> std::io::Result<()> {
    let mut file = File::create(file_name.clone() + ".ppm")?;
    writeln!(&mut file, "P6 {} {} 255", width, height)?;
    for color in colors {
        write!(&mut file, "{}{}{}", color.r as char, color.g as char, color.b as char)?;
    }
    Ok(())
}

pub fn export_ppm_file(file_name: &String, width: usize, height: usize, colors: Vec<Color>) -> Result<(), String> {
    let size = width * height;
    if size != colors.len() {
        return Err(String::from("Invalid size"));
    }
    match write_ppm_file(file_name, width, height, colors) {
        Ok(()) => Ok(()),
        Err(x) => Err(x.to_string())
    }
}
