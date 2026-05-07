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
pub fn draw(player: &Player, display: &mut SimulatorDisplay<Rgb888>) {
    Circle::new(Point::new(player.position.x, player.position.y), 10)
        .into_styled(player.color)
        .draw(display);
}
// TODO: Finish this velocites function
pub fn update_velocites(player: &mut Player) {
    if player.velocity.0 > 0.0 {}
}
pub fn move_player(player: &mut Player) {
    match player.input.0 {
        Vinput::UP => {
            player.velocity.0 = 5.0;
            player.input.0 = Vinput::NONE
        }
        Vinput::DOWN => {
            player.velocity.0 = 5.0;
            player.input.0 = Vinput::NONE
        }
        _ => {}
    }
    match player.input.1 {
        Hinput::LEFT => {
            player.velocity.1 = 5.0;
            player.input.1 = Hinput::NONE;
        }
        Hinput::RIGHT => {
            player.velocity.1 = 5.0;
            player.input.1 = Hinput::NONE;
        }
        _ => {}
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
