use rppal::gpio::{Gpio, Trigger};
use rumqtt::{MqttClient, MqttOptions, QoS};
use std::{env, error::Error, fs::read};

mod capture;

const GPIO_NO: u8 = 27;

const MQTT_HOST: &str = "cat-camera";
const MQTTS_PORT: u16 = 8883;
const MQTT_KEEP_ALIVE: u16 = 30;
const MQTT_RECONNECT_INTERVAL: u64 = 60;
const MQTT_PUB_TOPIC: &str = "cat-camera/capture";

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
                    .publish(MQTT_PUB_TOPIC, qos, retain, payload)
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
    let ca_path = env::var("AWS_IOT_CA_PATH").unwrap();
    let client_crt_path = env::var("AWS_IOT_CLIENT_CERT_PATH").unwrap();
    let client_key_path = env::var("AWS_IOT_CLIENT_KEY_PATH").unwrap();
    let ca = read(ca_path).unwrap();
    let client_crt = read(client_crt_path).unwrap();
    let client_key = read(client_key_path).unwrap();

    let endpoint = env::var("AWS_IOT_ENDPOINT").unwrap();

    MqttOptions::new(MQTT_HOST, endpoint, MQTTS_PORT)
        .set_ca(ca)
        .set_client_auth(client_crt, client_key)
        .set_keep_alive(MQTT_KEEP_ALIVE)
        .set_reconnect_opts(rumqtt::ReconnectOptions::Always(MQTT_RECONNECT_INTERVAL))
}
