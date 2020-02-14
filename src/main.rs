#![feature(test)]
extern crate test;

mod structs;
use std::path::Path;
use std::env;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args[1]);

    if(args.len() >= 2) {
        let path_img = Path::new(&args[1]);
        let mut img = structs::Image::new_with_file(path_img);
        println!("original image: ");
        img.to_string();
        println!("grey image: ");
        img.grey_image();
        img.to_string()
    } else {
        let mut input_text = String::new();

        println!("Enter path to image: ");
        io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
        println!("{}",input_text);
        let path_img = Path::new(&input_text);
        let img = structs::Image::new_with_file(path_img);
        println!("image: ");
        img.to_string();
    }
}
