use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;
use std::io::Read;
use std::iter::Iterator;
use std::io::Write;

#[derive(Clone, Debug, Copy)]
pub struct Pixel {
    red : u8,
    blue: u8,
    green: u8,
}

impl Pixel {
    pub fn new(red: u8, blue: u8, green: u8) -> Pixel{
        Pixel{
            red: red,
            blue: blue,
            green: green,
        }
    }

    pub fn display(&self)-> String{
        return format!("{} {} {} ", self.red, self.blue,self.green);
    }

    pub fn invert(&mut self) {
        self.red = 255 - &self.red;
        self.blue = 255 - &self.blue;
        self.green =  255 - &self.green;
    }

    pub fn grey(&mut self){
        let average : u8 = (&self.red + &self.blue + &self.green)/3;
        self.red = average;
        self.blue = average;
        self.green = average;
    }
}

pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn new(h : &usize, w : &usize)-> Image {
        return Image {
                width : *h,
                height : *w,
                pixels : Vec::new()
        }
    }

    pub fn new_with_file(filename: &Path) -> Image {
        if filename.is_file() && filename.extension().unwrap()=="ppm" {
            return read_file(&filename)
        }
        else {
            panic!("can't load image !");
        }
    }


    pub fn save(img: Image, filename: &Path) {
        if filename.extension().unwrap()!="ppm" {
            panic!("Wrong extension for the file !");
        }
        save_file(img, filename);
        println!("Successfully saved");
    }

    pub fn get_pixel(&self, x : usize, y : usize) -> Pixel {
        let index : usize = self.width*x+y;
        return self.pixels[index];
    }

    pub fn grey_image(&mut self){
        for x in 0..self.pixels.len(){
            self.pixels[x].grey();
        }
    }
    
    pub fn invert(&mut self){
        for x in 0..self.pixels.len(){
            self.pixels[x].invert();
        }
    }
    
    pub fn to_string(&self){
        for i in 0..self.height{
            for j in 0..self.width{
                print!("{:?} - ", self.get_pixel(i, j).display());
            }
            println!("");
        }
    }
}

fn get_char_at_index(my_string : &String, index :usize) -> char{
    match my_string.chars().nth(index) {
        Some(c) => return c,
        None => panic!("No character at index : {}", index)
    }
}

pub fn read_file(filename: &Path) -> Image {
        
    let mut file = match File::open(&filename) {
        Err(e) => panic!("couldn't open file : {}", e),
        Ok(file) => file,
    };

    let mut init : bool = false;

    let mut img : Image = Image {
        height : 0,
        width : 0,
        pixels : Vec::new()
    };

    let buf_reader = BufReader::new(file);
    let mut h : usize = 0;
    let mut w : usize = 0;
    for line in buf_reader.lines() {
        let l = line.unwrap();
        if get_char_at_index(&l, 0)!='#'{
            let str_list = l.split_whitespace();
            let vec = str_list.collect::<Vec<&str>>();
            match vec.len() {
                1 => {
                    if get_char_at_index(&String::from(vec[0]), 0)=='P' {
                        println!("Format : {}", vec[0]);
                    }else {
                        println!("maximum value for each color : {} ", vec[0]);
                        if u8::from_str(vec[0]).unwrap()>255 {
                            panic!("The maximum value for the color is too big!");
                        }
                    }
                },
                2 => {
                    h = usize::from_str(vec[1]).unwrap();
                    w = usize::from_str(vec[0]).unwrap();

                    img = Image {
                        height : h,
                        width : w,
                        pixels : Vec::new()
                    };
                    init = true;
                    println!("Init with size {} x {}", w, h);
                },
                _ => {
                    if init == true {
                            for x in (0..vec.len()).step_by(3) {
                                let red : u8 = match u8::from_str(vec[x as usize]) {
                                    Err(e) => panic!("error in unwrap red vec {}", e),
                                    Ok(r) => r,
                                };
                                let blue : u8 = match u8::from_str(vec[x+1 as usize]) {
                                    Err(e) => panic!("error in unwrap blue vec {}", e),
                                    Ok(r) => r,
                                };
                                let green : u8 = match u8::from_str(vec[x+2 as usize]) {
                                    Err(e) => panic!("error in unwrap green vec {}", e),
                                    Ok(r) => r,
                                };
                                
                                let pix : Pixel = Pixel::new(red, blue, green);
                                img.pixels.push(pix);
                                
                            }
                    }else{
                        panic!("The image wasn't initialized");
                    }
                }

            }
        }
    }
    
    return img;
}

pub fn save_file(img: Image, filename: &Path) {
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



#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use test::Bencher;

    #[bench]
    fn bench_par_new_with_file_small_file(b: &mut Bencher) {
        b.iter(|| Image::new_with_file(Path::new("mandelbrot.ppm")))
    }

    #[bench]
    fn bench_new_with_file_large_file(b: &mut Bencher) {
        b.iter(|| Image::new_with_file(Path::new("big_mandelbrot.ppm")))
    }
}
