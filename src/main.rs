use camera_capture;
use image;

use std::fs::File;
use std::path::Path;

fn main() {
    let cam_builder = camera_capture::create(0).unwrap();

    let mut cam = cam_builder.fps(30.0).unwrap().resolution(1280, 720).unwrap().start().unwrap();
    let img = cam.next().unwrap();

    let file_name = "test.png";
    let path = Path::new(&file_name);
    let _ = &mut File::create(&path).unwrap();
    img.save(&path).unwrap();

    println!("img saved to {}", file_name);

}