use coap_lite::{CoapRequest, RequestType as Method};
use cbor_data::{CborBuilder, Encoder};
use std::collections::HashMap;
use std::net::SocketAddr;
use coap::{CoAPClient};
use std::time::Duration;

use crate::switch::PushButtonSensor;
use crate::switch::PushButtonEvent;

const RECV_TIMEOUT: u64 = 10; // 10s


type GetEndpoint = fn(&NetworkService) -> Vec<u8>;
type PostEndpoint = fn(&mut NetworkService, &[u8]) -> Vec<u8>;


pub struct NetworkService {
    switch: PushButtonSensor,
    get_actions: HashMap<String, GetEndpoint>,
    post_actions: HashMap<String, PostEndpoint>
}


impl NetworkService {

    pub fn new(id: u16, switch: PushButtonSensor) -> Self {
        Self {
            switch: switch,
            get_actions: HashMap::from([
                (format!("s/4001/{}/901", id), NetworkService::description as GetEndpoint),
                (format!("s/4001/{}/202", id), NetworkService::status as GetEndpoint),
                (format!("s/4001/{}/903", id), NetworkService::group as GetEndpoint)
            ]),
            post_actions: HashMap::new()
        }
    }

    fn description(&self) -> Vec<u8> {
        let cbor = CborBuilder::default()
            .encode_str(self.switch.description());
        return cbor.into_vec();
    }

    fn status(&self) -> Vec<u8> {
        let cbor = CborBuilder::default()
            .encode_u64(self.switch.status() as u64);
        return cbor.into_vec();
    }

    fn group(&self) -> Vec<u8> {
        let cbor = CborBuilder::default()
            .encode_u64(self.switch.app_group().into());
        return cbor.into_vec();
    }

    fn get_dispatch(&self, route: &String) -> Vec<u8> {
        let func = self.get_actions.get(route).map(|x| *x);
        match func {
            Some(func) => {
               return func(self);
            },
            None => {
                return b"Not Found".to_vec();
            }
        }
    }

    fn post_dispatch(&mut self, route: &String, msg: &[u8]) -> Vec<u8> {
        let func = self.post_actions.get(route).map(|x| *x);

        match func {
            Some(func) => {
                return func(self, msg);
            },
            None => {
                return b"Not Found".to_vec();
            }
        }
    }

    pub fn handle_event(&mut self, request: &CoapRequest<SocketAddr>) -> Vec<u8> {
        let route = &request.get_path();

        match request.get_method() {
            &Method::Get => {
                println!("GET: {}", route);
                self.get_dispatch(route)
            }
            &Method::Post => {
                println!("POST: {}", route);
                self.post_dispatch(route, &request.message.payload)
            },
            _ => {
                b"Method not allowed".to_vec()
            }
        }
    }

    pub fn report_status(&mut self, command: &String) {
        match &command[..] {
            "RELEASE" => self.switch.push(PushButtonEvent::Release),
            "CLICK" => self.switch.push(PushButtonEvent::Click),
            "HOLD" => self.switch.push(PushButtonEvent::Hold),
            "STUCK" => self.switch.push(PushButtonEvent::Stuck),
            _ => ()
        }

        for ip in self.switch.ip_destinations() {
            let url = format!("coap://{}/", ip);
            println!("Client request: {}", url);
            println!("Switch status is: {:?}", self.switch.status());

            let payload = self.status();
            let response = CoAPClient::post_with_timeout(
                &url, payload, Duration::from_secs(RECV_TIMEOUT)
            );
        }
    }
}