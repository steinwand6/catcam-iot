use rppal::gpio::{Gpio, Trigger};
use std::error::Error;

mod capture;

const GPIO_NO: u8 = 27;

fn main() -> Result<(), Box<dyn Error>> {
    let mut pir = Gpio::new()?.get(GPIO_NO)?.into_input();
    pir.set_interrupt(Trigger::RisingEdge)?;
    loop {
        match pir.poll_interrupt(true, None) {
            Ok(_) => {
                let bytes = capture::get_image_bytes()?;
                println!("{:?}", bytes);
            }
            e => {
                eprint!("{:?}", e);
                break;
            }
        }
    }

    Ok(())
}
