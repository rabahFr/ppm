/*
mod structs;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;
use std::io::Read;
use std::iter::Iterator;
use std::io::Write;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
pub fn save_file(img: structs::Image, filename: String) {
    let path_img = Path::new(&filename);
    let format : String = String::from("P3 \n");
    let dimension : String = format!("{} {} \n", &img.width, &img.height);
    let max_pix_color_value : String = String::from("255 \n");
    
    let mut file = match File::create(&filename) {
        Err(e) => panic!("couldn't create file : {}", e),
        Ok(file) => file,
    };
    
    file.write_all(format.as_bytes());
    file.write_all(dimension.as_bytes());
    file.write_all(max_pix_color_value.as_bytes());
    
    for i in 0..img.height {
        for j in 0..img.width {
            file.write_all(img.get_pixel(i as usize, j as usize).display().as_bytes());
        }
        file.write_all(b"\n");
    }
}

#[pymodule]
fn imgppm(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    //m.add_wrapped(wrap_pyfunction!(read_file))?;
    m.add_wrapped(wrap_pyfunction!(save_file))?;
    Ok(())
}*/