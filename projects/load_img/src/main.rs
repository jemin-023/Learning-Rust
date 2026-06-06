use image::ImageReader;
use image::imageops::FilterType;

fn main(){
    let img = ImageReader::open("cat.jpg").unwrap().decode().unwrap().to_luma8();
    let ramp = "@$B%8&WM#*oahkbdpqwmZO0QLCJUXcvunxrjft/|()1{}[]?-_+~<>i!lI;:,^`'. ";
    let chars = ramp.as_bytes();

    let img = image::imageops::resize(
        &img,
        100,
        60,
        FilterType::Nearest,
    );

    for y in 0..img.height(){
        for x in 0..img.width(){
            let b = img.get_pixel(x,y)[0];
            let idx = (b as usize * 65) / 255;
            print!("{}", chars[idx] as char);
        }
        println!();
    }

}