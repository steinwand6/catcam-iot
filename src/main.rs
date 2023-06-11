use std::error::Error;

mod capture;

fn main() -> Result<(), Box<dyn Error>> {
    let bytes = capture::get_image_bytes()?;
    println!("{:?}", bytes);
    Ok(())
}
