use rppal::gpio::{Gpio, Trigger};
use rumqtt::{MqttClient, MqttOptions, QoS};
use std::{env, error::Error, fmt::Display, fs::read};

mod capture;

const GPIO_NO: u8 = 27;

const MQTT_HOST: &str = "cat-camera";
const MQTTS_PORT: u16 = 8883;
const MQTT_KEEP_ALIVE: u16 = 30;
const MQTT_RECONNECT_INTERVAL: u64 = 60;
const MQTT_PUB_TOPIC: &str = "cat-camera/capture";

#[derive(Debug)]
enum MqttError {
    EnvVar(String),
    FileRead(String),
    Establish(rumqtt::ConnectError),
    Publish,
}

impl Error for MqttError {}

impl Display for MqttError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use MqttError::*;
        match self {
            EnvVar(var) => write!(f, "Failed to get environment variable: {var}"),
            FileRead(file) => write!(f, "Failed to read file: {file}"),
            Establish(e) => write!(f, "Failed to establish MQTT connection: {e}"),
            Publish => write!(f, "Failed to publish"),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut pir = Gpio::new()?.get(GPIO_NO)?.into_input();
    pir.set_interrupt(Trigger::RisingEdge)?;

    let mqtt_options = get_mqtt_options()?;
    let (mut mqtt_client, _notifications) = match MqttClient::start(mqtt_options) {
        Ok(res) => res,
        Err(e) => return Err(Box::new(MqttError::Establish(e))),
    };

    loop {
        match pir.poll_interrupt(true, None) {
            Ok(_) => {
                let bytes = capture::get_image_bytes()?;
                println!("{:?}", bytes);
                publish_mqtt(&mut mqtt_client, bytes)?;
            }
            e => {
                eprint!("{:?}", e);
                break;
            }
        }
    }

    Ok(())
}

fn publish_mqtt<T: Into<Vec<u8>>>(client: &mut MqttClient, payload: T) -> Result<(), MqttError> {
    let qos = QoS::AtLeastOnce;
    let retain = false;
    if let Err(_) = client.publish(MQTT_PUB_TOPIC, qos, retain, payload) {
        return Err(MqttError::Publish);
    }
    Ok(())
}

fn get_mqtt_options() -> Result<MqttOptions, MqttError> {
    let ca_path = get_env_var("AWS_IOT_CA_PATH")?;
    let client_crt_path = get_env_var("AWS_IOT_CLIENT_CERT_PATH")?;
    let client_key_path = get_env_var("AWS_IOT_CLIENT_KEY_PATH")?;
    let ca = read_file(&ca_path)?;
    let client_crt = read_file(&client_crt_path)?;
    let client_key = read_file(&client_key_path)?;

    let endpoint = get_env_var("AWS_IOT_ENDPOINT")?;

    Ok(MqttOptions::new(MQTT_HOST, endpoint, MQTTS_PORT)
        .set_ca(ca)
        .set_client_auth(client_crt, client_key)
        .set_keep_alive(MQTT_KEEP_ALIVE)
        .set_reconnect_opts(rumqtt::ReconnectOptions::Always(MQTT_RECONNECT_INTERVAL)))
}

fn get_env_var(var_key: &str) -> Result<String, MqttError> {
    match env::var(var_key) {
        Ok(val) => Ok(val),
        Err(_) => Err(MqttError::EnvVar(var_key.to_string())),
    }
}

fn read_file(file: &str) -> Result<Vec<u8>, MqttError> {
    match read(file) {
        Ok(bytes) => Ok(bytes),
        Err(_) => Err(MqttError::FileRead(file.to_string())),
    }
}
