use image::ImageReader;
use image::imageops::FilterType;
use std::process::Command;
use std::io::{stdout, Write};

fn show_frame(path: &str){
    let img = ImageReader::open(path).unwrap().decode().unwrap().to_luma8();
    let img = image::imageops::resize(&img, 120, 40, FilterType::Triangle);
    let ramp = "█▓▒░ ";
    let chars: Vec<char> = ramp.chars().collect();

    for y in 0..img.height(){
        for x in 0..img.width(){
            let b = img.get_pixel(x,y)[0];
            let idx = (b as usize * (chars.len() - 1)) / 255;
            print!("{}", chars[idx] as char);
        }
        println!();
    }
}

fn main() {
    std::fs::create_dir_all("frames").unwrap();
    let url = "https://www.youtube.com/watch?v=FtutLA63Cp8";

    Command::new("yt-dlp")
    .args([
        "--no-playlist",
        "--merge-output-format",
        "mp4",
        "-o",
        "video",
        url,
    ])
    .status()
    .unwrap();
    Command::new("ffmpeg").args(["-y","-i", "video.mp4", "frames/frame_%05d.jpg",]).status().unwrap();

    let mut i = 1;
    loop {
        let path = format!("frames/frame_{:05}.jpg", i);
        if !std::path::Path::new(&path).exists() {
            break;
        }
        print!("\x1B[2J\x1B[H");
        show_frame(&path);
        stdout().flush().unwrap();
        std::fs::remove_file(&path).unwrap();
        i += 1;
    }
    std::fs::remove_file("video.mp4").unwrap();
}
