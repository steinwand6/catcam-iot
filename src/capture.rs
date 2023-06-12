use std::{env, error::Error, fmt::Display, fs::File, io::Read, process::Command};

type Result<T> = std::result::Result<T, ImageCaptureError>;

#[derive(Debug)]
pub enum ImageCaptureError {
    FileOpen(String),
    FileRead(String),
    CommandExecution(String),
}

impl Error for ImageCaptureError {}

impl Display for ImageCaptureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ImageCaptureError::*;
        match self {
            FileOpen(msg) => write!(f, "Failed to open file: {msg}"),
            FileRead(msg) => write!(f, "Failed to read file: {msg}"),
            CommandExecution(msg) => write!(f, "Failed to execute command: {msg}"),
        }
    }
}

pub fn get_image_bytes() -> Result<Vec<u8>> {
    let file_name = "capture.jpeg";
    capture_image(file_name)?;
    let mut image_file = match File::open(file_name) {
        Ok(f) => f,
        Err(e) => {
            return Err(ImageCaptureError::FileOpen(e.to_string()));
        }
    };
    let mut buf = Vec::new();
    match image_file.read_to_end(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(ImageCaptureError::FileRead(e.to_string())),
    }
}

fn capture_image(file_name: &str) -> Result<()> {
    let libcam = Command::new("libcamera-jpeg")
        .args(["-o", file_name])
        .args(get_options())
        .output();
    match libcam {
        Ok(_) => println!("libcamera-jpeg: {file_name}"),
        Err(e) => {
            eprintln!("{:?}", e);
            return Err(ImageCaptureError::CommandExecution(e.to_string()));
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
