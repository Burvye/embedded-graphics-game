use embedded_graphics::pixelcolor::{PixelColor, Rgb888};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

const SPEED: i32 = 50;

pub struct Player {
    pub position: Point,
    pub velocity: (i32, i32),
    pub color: PrimitiveStyle<Rgb888>,
    pub input: (Hinput, Vinput),
}
pub fn draw(player: &Player, display: &mut SimulatorDisplay<Rgb888>) {
    Circle::new(Point::new(player.position.x, player.position.y), 10)
        .into_styled(player.color)
        .draw(display);
}
// TODO: Finish this velocites function
pub fn update_velocites(player: &mut Player) {
    player.position.x += player.velocity.1;
    player.velocity.0 = (player.velocity.0 as f32 * 0.5) as i32;
    player.position.y += player.velocity.0;
    player.velocity.1 = (player.velocity.1 as f32 * 0.5) as i32;
}
pub fn move_player(player: &mut Player) {
    match player.input.1 {
        Vinput::UP => {
            player.velocity.0 += -2 * SPEED;
            player.input.1 = Vinput::NONE
        }
        Vinput::DOWN => {
            player.velocity.0 += 2 * SPEED;
            player.input.1 = Vinput::NONE
        }
        _ => {}
    }
    match player.input.0 {
        Hinput::LEFT => {
            player.velocity.1 += -SPEED;
            player.input.0 = Hinput::NONE;
        }
        Hinput::RIGHT => {
            player.velocity.1 += SPEED;
            player.input.0 = Hinput::NONE;
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
