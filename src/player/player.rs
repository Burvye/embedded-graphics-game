use embedded_graphics::pixelcolor::{PixelColor, Rgb888};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

pub struct Player {
    pub position: Point,
    pub velocity: (f32, f32),
    pub color: PrimitiveStyle<Rgb888>,
    pub input: (Vinput, Hinput),
    pub timer: i32,
}
impl Player {
    pub fn draw(&self, display: &mut SimulatorDisplay<Rgb888>) {
        Circle::new(Point::new(self.position.x, self.position.y), 10)
            .into_styled(self.color)
            .draw(display);
    }
    // TODO: Finish this velocites function
    pub fn update_velocites(&mut self) {
        if self.velocity.0 > 0.0 {}
    }
    pub fn move_player(&mut self) {
        match self.input.0 {
            Vinput::UP => {
                self.velocity.0 = 5.0;
                self.input.0 = Vinput::NONE
            }
            Vinput::DOWN => {
                self.velocity.0 = 5.0;
                self.input.0 = Vinput::NONE
            }
            _ => {}
        }
        match self.input.1 {
            Hinput::LEFT => {
                self.velocity.1 = 5.0;
                self.input.1 = Hinput::NONE;
            }
            Hinput::RIGHT => {
                self.velocity.1 = 5.0;
                self.input.1 = Hinput::NONE;
            }
            _ => {}
        }
    }
}

pub enum Vinput {
    UP,
    DOWN,
    NONE,
}
pub enum Hinput {
    LEFT,
    RIGHT,
    NONE,
}

pub enum Color {
    RED,
}
