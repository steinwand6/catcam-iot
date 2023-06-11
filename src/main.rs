use std::{env, fs::File, io::Read, process::Command};

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
        .args(get_options())
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

fn get_options() -> Vec<String> {
    let ev = env::var("PC_IOT_EV").unwrap_or("0.5".to_string());
    let shutter = env::var("PC_IOT_SHUTTER").unwrap_or("2000000".to_string());
    let width = env::var("PC_IOT_WIDTH").unwrap_or("500".to_string());
    let height = env::var("PC_IOT_HEIGHT").unwrap_or("500".to_string());

    let libcam_args = [
        "--nopreview",
        "--ev",
        &ev,
        "--shutter",
        &shutter,
        "--width",
        &width,
        "--height",
        &height,
        "--brightness",
        "0.2",
    ];
    libcam_args.map(|e| e.to_string()).to_vec()
}
