use std::thread;
use fltk::app::App;
use tokio::runtime::Runtime;
use coap::Server;
use std::sync::{Arc, Mutex};
use std::env::var;


mod device;
mod service;
mod switch;


fn main() {
    let COAP_ADDRESS = var("COAP_ADDRESS").unwrap_or_else(|_| {
        panic!("Variable COAP_ADDRESS is not set");
    });
    let CONTROL_ADDRESS = var("CONTROL_ADDRESS").unwrap_or_else(|_| {
        panic!("Variable CONTROL_ADDRESS is not set");
    });
    let app = App::default();
    
    // Initialize HW
    let switch = Arc::new(Mutex::new(
        device::PhysicalSwitch::new(400, 400)
    ));

    let sensor = switch::PushButtonSensor::new(
        Arc::clone(&switch), CONTROL_ADDRESS
    );
    let node = Arc::new(Mutex::new(
        service::NetworkService::new(1, sensor)
    ));

    // Handle event handling of button press
    let switch_copy = Arc::clone(&switch);
    let node_copy = Arc::clone(&node);

    switch.lock().unwrap().handle_press(move |_| {
        if let Some(choice) = switch_copy.lock().unwrap().get_event() {
            let cmd = String::from(choice.as_str());
            node_copy.lock().unwrap().report_status(&cmd);
        }
    });

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


