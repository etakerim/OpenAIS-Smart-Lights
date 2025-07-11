use fltk::{
    prelude::*,
    window::Window,
    button::Button,
    menu::Choice
};


pub struct PhysicalSwitch {
    switch: Button,
    event: Choice
}


impl PhysicalSwitch {

    pub fn new(width: i32, height: i32) -> Self {
        let mut device = Window::default()
            .with_size(width, height)
            .with_label("Light switch");

        let switch = Button::default()
            .with_size(200, 200)
            .with_label("Press")
            .center_of_parent();

        let mut event = Choice::default()
            .with_size(80, 30)
            .with_label("Select item");
        

        event.add_choice("CLICK");
        event.add_choice("HOLD");
        event.set_value(0);
        
        device.end();
        device.show();


        Self {
            switch: switch,
            event: event
        }
    }

    pub fn get_event(&self) -> Option<String> {
        self.event.choice()
    }

    pub fn handle_press<F: FnMut(&mut Button) + 'static>(&mut self, command: F) {
        self.switch.set_callback(command);
    }
}
