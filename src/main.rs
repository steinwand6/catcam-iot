use std::{fs::File, io::Read, process::Command};

fn main() -> Result<(), ()> {
    let bytes = get_image_bytes()?;
    println!("{:?}", bytes);
    Ok(())
}

fn get_image_bytes() -> Result<Vec<u8>, ()> {
    let file_name = "capture.jpeg";
    capture_image(file_name)?;
    let mut image_file = match File::open(file_name) {
        Ok(f) => f,
        Err(_e) => {
            return Err(());
        }
    };
    let mut buf = Vec::new();
    match image_file.read_to_end(&mut buf) {
        Ok(_) => Ok(buf),
        Err(_e) => Err(()),
    }
}

fn capture_image(file_name: &str) -> Result<(), ()> {
    let libcam = Command::new("libcamera-jpeg")
        .args(["-o", file_name])
        .output();
    match libcam {
        Ok(_) => println!("libcamera-jpeg: {}", file_name),
        Err(e) => {
            eprintln!("{:?}", e);
            return Err(());
        }
    }
    Ok(())
}
