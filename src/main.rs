use embedded_graphics::pixelcolor::{PixelColor, Rgb888};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use std::thread;
use std::time::Duration;
pub mod player;
use crate::player::player::{draw, move_player, update_velocites};
use embedded_graphics_simulator::sdl2::Keycode;

fn main() -> Result<(), core::convert::Infallible> {
    println!("Hello, world!");

    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(1000, 600));
    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("embedded graphics window", &output_settings);
    let red_style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(Rgb888::CSS_DARK_ORANGE)
        .fill_color(Rgb888::CSS_ORANGE)
        .build();
    let mut positions: Vec<Point> = Vec::new();
    let mut player = player::Player {
        color: red_style,
        input: (player::Hinput::NONE, player::Vinput::NONE),
        position: Point { x: 500, y: 300 },
        velocity: (0, 0),
    };

    positions.push(player.position);
    screen_wrap(&mut positions, display.size());
    window.update(&mut display);
    'running: loop {
        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown {
                    keycode: Keycode::W,
                    ..
                } => {
                    player.input.1 = player::Vinput::UP;
                }
                SimulatorEvent::KeyDown {
                    keycode: Keycode::S,
                    ..
                } => {
                    player.input.1 = player::Vinput::DOWN;
                }
                SimulatorEvent::KeyDown {
                    keycode: Keycode::A,
                    ..
                } => {
                    player.input.0 = player::Hinput::LEFT;
                }
                SimulatorEvent::KeyDown {
                    keycode: Keycode::D,
                    ..
                } => {
                    player.input.0 = player::Hinput::RIGHT;
                }
                _ => {}
            }
        }

        display.clear(Rgb888::BLACK).unwrap();
        player_methods(&mut player, &mut display);
        // window update must be the last thing
        window.update(&mut display);
        thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}

fn player_methods(player: &mut player::Player, display: &mut SimulatorDisplay<Rgb888>) {
    move_player(player);
    update_velocites(player);
    draw(player, display);
}

fn screen_wrap(positions: &mut Vec<Point>, border: Size) {
    for position in positions {
        if position.x > border.width as i32 {
            position.x = 0
        }
        if position.y > border.height as i32 {
            position.y = 0
        }
        if position.x < 0 {
            position.x = border.width as i32
        }
        if position.y < 0 {
            position.y = border.height as i32
        }
    }
}
