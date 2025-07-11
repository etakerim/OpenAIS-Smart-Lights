use crate::device::PhysicalLight;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


type OnlyPhysicaLight = Arc<Mutex<PhysicalLight>>;


pub struct LightPointActuator {
    lamp: OnlyPhysicaLight,
    group_id: u16,

    description: String,
    status: bool,
    intensity: f32,      // 0..1

    priority: u8,        // 0..5
    step_size: f32,      // 0..1
    dimming_time: u16,   // x100 ms

    status_resend: u16,  // 0 .. 600s
    status_report: u8    // ID accoding 4012 "oA Status-Report Structure"
}


impl LightPointActuator {

    pub fn new(lamp: OnlyPhysicaLight) -> Self {
        lamp.lock().unwrap().glow(1.0);

        Self {
            lamp: lamp,
            group_id: 0,
            description: String::from("Dimmable Light-Point"),
            status: true,
            intensity: 1.0,
            priority: 1,
            step_size: 0.2,
            dimming_time: 10,
            status_resend: 10,
            status_report: 1
        }
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> bool {
        self.status
    }

    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    pub fn app_group(&self) -> u16 {
        self.group_id
    }

    pub fn status_resend(&self) -> u16 {
        self.status_resend
    }

    pub fn switch(&mut self, action: Option<bool>, intensity: Option<f32>, time: Option<u16>) {
        // let mut remain_transition = time * 100;     // miliseconds
        // let time_step = 500;
        // let mut level = self.intensity;
        // let level_step = (self.intensity - intensity) / ((remain_transition / time_step) as f32);
        match action {
            Some(x) => {
                self.status = x;
                self.intensity = if x { 1.0 } else { 0.0 };
            },
            _ => (),
        }

        match intensity {
            Some(x) => {
                self.status = if x > 0.0 { true } else { false };
                self.intensity = x;
            }
            _ => (),
        }
        println!("Light intensity is {}", self.intensity);

        self.lamp.lock().unwrap().glow(self.intensity);
    }


    pub fn dim(&mut self, direction: bool) {
         // cap maximum intensity
         if direction {
            if self.intensity >= 1.0 {
                self.intensity = 1.0;
                self.status = true;
            }
            self.intensity += self.step_size;
        } else {
            if self.intensity <= 0.0 {
                self.intensity = 0.0;
                self.status = false;
            } else {
                self.intensity -= self.step_size;
            }
        }

        // let remain_transition = self.dimming_time;
        self.lamp.lock().unwrap().glow(self.intensity);
    }

    pub fn step(&mut self, direction: bool, step: f32, transition_time: u16) {
        // let relative_step = self.intensity * step;

        // cap maximum intensity
        if direction {
            if self.intensity >= 1.0 {
                self.intensity = 1.0;
                self.status = true;
            }
            self.intensity += step;
        } else {
            if self.intensity <= 0.0 {
                self.intensity = 0.0;
                self.status = false;
            } else {
                self.intensity -= step;
            }
        }
        // let remain_transition = transition_time;
        println!("Light intensity is {}", self.intensity);
        self.lamp.lock().unwrap().glow(self.intensity);
    }
}
