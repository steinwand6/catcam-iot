use rppal::gpio::{Gpio, Trigger};
use rumqtt::{MqttClient, MqttOptions, QoS};
use std::{env, error::Error};

mod capture;

const GPIO_NO: u8 = 27;

fn main() -> Result<(), Box<dyn Error>> {
    let mut pir = Gpio::new()?.get(GPIO_NO)?.into_input();
    pir.set_interrupt(Trigger::RisingEdge)?;

    let mqtt_options = get_mqtt_options();
    let (mut mqtt_client, _notifications) = MqttClient::start(mqtt_options).unwrap();

    loop {
        match pir.poll_interrupt(true, None) {
            Ok(_) => {
                let bytes = capture::get_image_bytes()?;
                println!("{:?}", bytes);

                let payload = "hello AWS IoT!";
                let qos = QoS::AtLeastOnce;
                let retain = false;
                mqtt_client
                    .publish("cat-camera/capture", qos, retain, payload)
                    .unwrap();
            }
            e => {
                eprint!("{:?}", e);
                break;
            }
        }
    }

    Ok(())
}

fn get_mqtt_options() -> MqttOptions {
    let ca = include_bytes!("../cert/AmazonRootCA1.pem").to_vec();
    let client_crt = include_bytes!("../cert/cat-camera.crt").to_vec();
    let client_key = include_bytes!("../cert/private.key").to_vec();
    let mqtt_host = "cat-camera";
    let endpoint = env::var("AWS_IOT_ENDPOINT").unwrap();

    MqttOptions::new(mqtt_host, endpoint, 8883)
        .set_ca(ca)
        .set_client_auth(client_crt, client_key)
        .set_keep_alive(10)
        .set_reconnect_opts(rumqtt::ReconnectOptions::Always(5))
}
