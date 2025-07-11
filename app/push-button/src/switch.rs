use crate::device::PhysicalSwitch;
use std::sync::{Arc, Mutex};


#[derive(Copy, Clone, Debug)]
pub enum PushButtonEvent {
    Release = 0,
    Click = 1,
    Hold = 2,
    DoubleClick = 3,
    Stuck = 255
}


type OnlyPhysicalSwitch = Arc<Mutex<PhysicalSwitch>>;


pub struct PushButtonSensor {
    group_id: u16,
    switch: OnlyPhysicalSwitch,
    description: String,
    status: PushButtonEvent,
    single_click_time: u16,
    ip_addresses: Vec<String>
}


impl PushButtonSensor {

    pub fn new(switch: OnlyPhysicalSwitch, ip_controller: String) -> Self {
        Self {
            group_id: 0,
            switch: switch,
            description: String::from("Push-Button Sensor"),
            status: PushButtonEvent::Release,
            single_click_time: 300,
            ip_addresses: vec![ip_controller]
        }
    }
    
    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn app_group(&self) -> u16 {
        self.group_id
    }

    pub fn ip_destinations(&self) -> &Vec<String> {
        &self.ip_addresses
    }

    pub fn status(&self) -> PushButtonEvent {
        self.status
    }

    pub fn push(&mut self, action: PushButtonEvent) {
        self.status = action;
    }
}