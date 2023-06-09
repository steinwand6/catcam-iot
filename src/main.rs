use std::process::Command;

fn main() -> Result<(), ()> {
    libcam("test.jpeg")
}

fn libcam(file_name: &str) -> Result<(), ()> {
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
