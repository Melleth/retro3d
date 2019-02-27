extern crate quicksilver;
extern crate image;
extern crate rand;

mod map;
use map::Map;

use quicksilver::{
    Result,
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{Event, Settings, State, Window, run},
    input::*,
};

use image::{RGB, ImageBuffer,GenericImage, GenericImageView};

use rand::prelude::*;

const ROTATE_SPEED: f32 = 0.1;
const MOVEMENT_SPEED: f32 = 0.1;


fn random_color() -> [u8; 3] {
    [rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()]
}

fn convert_color(in_color: [u8; 3]) -> Color {
    let out_color = Color{  r: in_color[0] as f32 / 255.0,
                        g: in_color[1] as f32 / 255.0,
                        b: in_color[2] as f32 / 255.0,
                        a: 1.0
    };
    out_color
}

struct DrawGame {
    player_angle: f32,
    player_position: (f32, f32),
    map: Map,
    
}

impl State for DrawGame {
    fn new() -> Result<DrawGame> {
        let map = Map::new_with_walls(16, 16);
        Ok(DrawGame{map: map, player_position: (8.0,8.0), player_angle: 0.0})
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        self.map.render_map(window);
        self.map.render_world(self.player_position, self.player_angle, window);


        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::Key(Key::Left, ButtonState::Pressed) => {
                self.player_angle -= ROTATE_SPEED;
            },
            Event::Key(Key::Right, ButtonState::Pressed) => {
                self.player_angle += ROTATE_SPEED;
            },
            Event::Key(Key::W, ButtonState::Pressed)=> {
                let dx = self.player_angle.cos()*MOVEMENT_SPEED;
                let dy = self.player_angle.sin()*MOVEMENT_SPEED;
                self.player_position.0 += dx;
                self.player_position.1 += dy;
            },
            Event::Key(Key::S, ButtonState::Pressed) => {
                let dx = self.player_angle.cos()*MOVEMENT_SPEED;
                let dy = self.player_angle.sin()*MOVEMENT_SPEED;
                self.player_position.0 -= dx;
                self.player_position.1 -= dy;
            },
            _ => ()
        };
        Ok(())
    }
}

fn main() {
    run::<DrawGame>("retro3d", Vector::new(1024, 512), Settings::default());
}