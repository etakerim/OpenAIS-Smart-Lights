use coap_lite::{CoapRequest, RequestType as Method};
use cbor_data::{CborBuilder, Encoder, Writer, value::Number};
use std::net::SocketAddr;
use coap::CoAPClient;
use std::time::Duration;

use crate::discovery::Discovery;

const RECV_TIMEOUT: u64 = 1; // 5s


struct LightControl {
    destination: String
}

impl LightControl {

    pub fn new(ip: &String) -> Self {
        Self {
            destination: format!("coap://{}/", ip)
        }
    }

    pub fn query_status(&self) -> Option<bool> {
        const PATH: &str = "s/4001/1/100";

        let url_status = format!("{}{}", self.destination, PATH);
        let response = CoAPClient::get_with_timeout(
            &url_status, Duration::from_secs(RECV_TIMEOUT)
        );

        if !response.is_ok() {
            println!("Error: Light '{}' is not available", url_status);
            return None;
        }

        let mut buffer = Vec::new();
        let cbor = CborBuilder::with_scratch_space(&mut buffer)
            .write_canonical(response.unwrap().message.payload.as_ref())
            .unwrap();

        let status = cbor.decode().as_bool().unwrap();
        return Some(status);
    }

    pub fn turn_switch(&self, status: bool) {
        const PATH: &str = "s/4001/1/117";

        let cbor = CborBuilder::new().encode_dict(|builder| {
            builder.with_key("0", |builder| builder.encode_bool(status));
            builder.with_key("2", |builder| builder.encode_u64(100));
        });
        let payload = cbor.into_vec();

        let url_switch = format!("{}{}", self.destination, PATH);
        println!("Request: {}", url_switch);

        let response = CoAPClient::post_with_timeout(
            &url_switch, payload, Duration::from_secs(RECV_TIMEOUT)
        );

        if !response.is_ok() {
            println!("Error: Light '{}' is not available", url_switch);
        }
    }

    pub fn step_intensity(&self, status: bool) {
        const PATH: &str = "s/4001/1/120";

        let cbor = CborBuilder::new().encode_dict(|builder| {
            builder.with_key("0", |builder| builder.encode_bool(status));
            builder.with_key("1", |builder| builder.encode_f64(0.2));
            builder.with_key("2", |builder| builder.encode_u64(100));
        });
        let payload = cbor.into_vec();

        let url_switch = format!("{}{}", self.destination, PATH);
        println!("Request: {}", url_switch);

        let response = CoAPClient::post_with_timeout(
            &url_switch, payload, Duration::from_secs(RECV_TIMEOUT)
        );

        if !response.is_ok() {
            println!("Error: Light '{}' is not available", url_switch);
        }
    }
}


pub struct NetworkService {
    resources: Discovery
}


impl NetworkService {

    pub fn new() -> Self {
        let mut resources = Discovery::new("topology.yaml".to_string());
        resources.load_topology();

        Self {
            resources: resources
        }
    }

    pub fn handle_event(&mut self, request: &CoapRequest<SocketAddr>) -> Vec<u8> {
        let route = &request.get_path();

        match request.get_method() {
            &Method::Post => {
                println!("POST: {}", route);
                // 1) Check if origin is push button (source address) - cbor attribute
                // println!("Origin: {:?}", request.source.unwrap()); .port(), .ip()
                // 2) Parse CBOR to act on received event (CLICK = change state, HOLD = dim)

                let mut buffer = Vec::new();
                let cbor = CborBuilder::with_scratch_space(&mut buffer)
                    .write_canonical(request.message.payload.as_ref())
                    .unwrap();
            
                let arg = cbor.decode().make_static();
        
                if let cbor_data::CborValue::Number(Number::Int(x)) = arg {
                    let pressed = x as u16;
                    println!("Status: {:?}", pressed);
            
                    // 3) Send to all registered lights
                    for ip in self.resources.lights() {
                        let light = LightControl::new(&ip);

                        let status = light.query_status();
                        if let Some(x) = status {
                            if pressed == 1 {          // CLICK
                                light.turn_switch(!x)
                            } else if pressed == 2 {   // HOLD
                                light.step_intensity(!x)
                            }
                        }
                    }
                }

                b"Ok".to_vec()
            },
            _ => {
                b"Method not allowed".to_vec()
            }
        }
    }
}