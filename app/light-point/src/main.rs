use std::thread;
use fltk::app::App;
use tokio::runtime::Runtime;
use coap::Server;
use std::sync::{Arc, Mutex};
use std::env::var;


mod device;
mod service;
mod light;


fn main() {
    let COAP_ADDRESS = var("COAP_ADDRESS").unwrap_or_else(|_| {
        panic!("Variable COAP_ADDRESS is not set");
    });
    
    let app = App::default();
    
    // Initialize HW
    let lamp = Arc::new(Mutex::new(
        device::PhysicalLight::new(500, 300, 50)
    ));
    let actuator = light::LightPointActuator::new(Arc::clone(&lamp));
    let node = Arc::new(Mutex::new(
        service::NetworkService::new(1, actuator)
    ));

    // Run CoAP server
    thread::spawn(move || {
        let runtime = Runtime::new().unwrap();

        runtime.block_on(async move {
            let mut server = Server::new(COAP_ADDRESS).unwrap();
            
            server.run(|request| async {
                let answer = node.lock().unwrap().handle_event(&request);

                return match request.response {
                    Some(mut message) => {
                        message.message.payload = answer.to_vec();
                        Some(message)
                    },
                    _ => None
                };
            }).await.unwrap();
        });
    });

    // Run GUI - simulate light hardware
    app.run().unwrap();
}
