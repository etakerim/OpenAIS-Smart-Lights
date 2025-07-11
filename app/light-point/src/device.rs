use fltk::{
    prelude::*,
    window::Window,
    enums::{Color, FrameType},
    frame::Frame
};
use palette::{Hsv, Srgb, IntoColor};


pub struct PhysicalLight {
    fixture: Frame
}


impl PhysicalLight {

    pub fn new(width: i32, height: i32, frame: i32) -> Self {
        let mut device = Window::default()
            .with_size(width, height)
            .with_label("Light Fixture");

        let mut fixture = Frame::default()
            .with_size(width - frame, height - frame)
            .center_of(&device);

        device.set_color(Color::White);
        fixture.set_frame(FrameType::FlatBox);
        device.end();
        device.show();

        Self {
            fixture: fixture
        }
    }

    pub fn glow(&mut self, brightness: f32) {
        if brightness < 0.0 && brightness > 1.0 { return };
        let rgb: Srgb = Hsv::new(60.0, 1.0, brightness).into_color();
        let color = rgb.into_format::<u8>();

        self.fixture.set_color(
            Color::from_rgb(color.red, color.green, color.blue)
        );
        fltk::app::awake();
        self.fixture.redraw();
    }
}