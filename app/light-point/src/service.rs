use coap_lite::{CoapRequest, RequestType as Method};
use cbor_data::{CborBuilder, Encoder, index_str, Writer, value::Number};
use std::collections::HashMap;
use std::net::SocketAddr;

use crate::light::LightPointActuator;


type GetEndpoint = fn(&NetworkService) -> Vec<u8>;
type PostEndpoint = fn(&mut NetworkService, &[u8]) -> Vec<u8>;


pub struct NetworkService {
    light: LightPointActuator,
    get_actions: HashMap<String, GetEndpoint>,
    post_actions: HashMap<String, PostEndpoint>
}


impl NetworkService {

    pub fn new(id: u16, light: LightPointActuator) -> Self {
        Self {
            light: light,
            get_actions: HashMap::from([
                (format!("s/4001/{}/901", id), NetworkService::description as GetEndpoint),
                (format!("s/4001/{}/100", id), NetworkService::status as GetEndpoint),
                (format!("s/4001/{}/101", id), NetworkService::intensity as GetEndpoint),
                (format!("s/4001/{}/903", id), NetworkService::group as GetEndpoint)
            ]),
            post_actions: HashMap::from([
                (format!("s/4001/{}/117", id), NetworkService::switch as PostEndpoint),
                (format!("s/4001/{}/118", id), NetworkService::dim as PostEndpoint),
                (format!("s/4001/{}/120", id), NetworkService::step as PostEndpoint)
            ])
        }
    }

    fn description(&self) -> Vec<u8> {
        let cbor = CborBuilder::default()
            .encode_str(self.light.description());
        return cbor.into_vec();
    }

    fn status(&self) -> Vec<u8> {
        let cbor = CborBuilder::default()
            .encode_bool(self.light.status());
        return cbor.into_vec();
    }

    fn intensity(&self) -> Vec<u8> {
        let cbor = CborBuilder::default()
            .encode_f64(self.light.intensity().into());
        return cbor.into_vec();
    }

    fn group(&self) -> Vec<u8> {
        let cbor = CborBuilder::default()
            .encode_u64(self.light.app_group().into());
        return cbor.into_vec();
    }

    fn switch(&mut self, payload: &[u8]) -> Vec<u8> {
        let mut buffer = Vec::new();
        let cbor = CborBuilder::with_scratch_space(&mut buffer)
            .write_canonical(payload.as_ref())
            .unwrap();

        let arg0 = cbor.index(index_str("0"));
        let arg1 = cbor.index(index_str("1"));
        let arg2 = cbor.index(index_str("2"));

        let mut status = None;
        let mut intensity = None;
        let mut transition = None;

        if let Some(status_x) = arg0 {
            status = Some(status_x.decode().as_bool().unwrap());
        }
        
        if let Some(intensity_x) = arg1 {
            let intensity_y = intensity_x.decode().make_static();

            if let cbor_data::CborValue::Number(Number::IEEE754(x)) = intensity_y {
                intensity = Some(x as f32);
            }
        }

        if let Some(transition_time_x) = arg2 {
            let transition_x = transition_time_x.decode().make_static();

            if let cbor_data::CborValue::Number(Number::Int(x)) = transition_x {
                transition = Some(x as u16);
            }
        }
        
        
        if (status.is_some() && intensity.is_some()) || 
            (status.is_none() && intensity.is_none()) {
            return b"Invalid".to_vec();
        }

        self.light.switch(status, intensity, transition);
        return b"Ok".to_vec();
    }

    fn dim(&mut self, payload: &[u8]) -> Vec<u8> {
        let mut buffer = Vec::new();
        let cbor = CborBuilder::with_scratch_space(&mut buffer)
            .write_canonical(payload.as_ref())
            .unwrap();

        let status = cbor.decode().as_bool().unwrap();
        self.light.dim(status);

        return b"Ok".to_vec();
    }

    fn step(&mut self, payload: &[u8]) -> Vec<u8> {
        let mut buffer = Vec::new();
        let cbor = CborBuilder::with_scratch_space(&mut buffer)
            .write_canonical(payload.as_ref())
            .unwrap();

        let arg0 = cbor.index(index_str("0"));
        let arg1 = cbor.index(index_str("1"));
        let arg2 = cbor.index(index_str("2"));

        let mut direction = None;
        let mut step_size = None;
        let mut transition = None;

        if let Some(direction_x) = arg0 {
            direction = Some(direction_x.decode().as_bool().unwrap());
        }
        
        if let Some(step_size_x) = arg1 {
            let step_x = step_size_x.decode().make_static();

            if let cbor_data::CborValue::Number(Number::IEEE754(x)) = step_x {
                step_size = Some(x as f32);
            }
        }

        if let Some(transition_time_x) = arg2 {
            let transition_x = transition_time_x.decode().make_static();

            if let cbor_data::CborValue::Number(Number::Int(x)) = transition_x {
                transition = Some(x as u16);
            }
        }

        self.light.step(
            direction.unwrap(), step_size.unwrap(), transition.unwrap()
        );
        return b"Ok".to_vec();
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
}