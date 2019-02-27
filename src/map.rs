use std::collections::HashMap;

use quicksilver::{
    Result,
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{Event, Settings, State, Window, run},
    input,
};

use rand::prelude::*;

pub struct Map {
    data: Vec<usize>,
    color_mapping: HashMap<usize, quicksilver::graphics::Color>,
    width: u32,
    height: u32,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let mut color_mapping: HashMap<usize, Color> = HashMap::new();
        color_mapping.insert(0, Color::WHITE);
        Map { data: vec![0; (width * height) as usize], width: width, height: height, color_mapping: color_mapping }
    }

    pub fn new_with_walls(width: u32, height: u32) -> Map {
        let mut data = vec![0; (width*height) as usize];
        // Top walls
        for (i, c) in data.iter_mut().enumerate().take(width as usize) {
            if (i as u32) < width {
                *c = 1;
            }
        }
        // Bottom walss
        for (i, c) in data.iter_mut().rev().enumerate().take(width as usize) {
            if (i as u32) < width {
                *c = 2;
            }
        }

        // Left walls
        for i in (0..data.len()).step_by(width as usize) {
            data[i] = 3;
        }

        // Right walls
        for i in ((width-1)..data.len() as u32).step_by(width as usize) {
            data[i as usize] = 4;
        }
        let mut color_mapping:HashMap<usize, Color> = HashMap::new();
        let mut rng = rand::thread_rng();
        color_mapping.insert(0, Color::WHITE);
        color_mapping.insert(1, Color { r: rng.gen(), g: rng.gen(), b: rng.gen(), a: 1.0 } );
        color_mapping.insert(2, Color { r: rng.gen(), g: rng.gen(), b: rng.gen(), a: 1.0 } );
        color_mapping.insert(3, Color { r: rng.gen(), g: rng.gen(), b: rng.gen(), a: 1.0 } );
        color_mapping.insert(4, Color { r: rng.gen(), g: rng.gen(), b: rng.gen(), a: 1.0 } );
        let mut map = Map { data: data, width: width, height: height, color_mapping: color_mapping };
        map.recursive_maze(0, 0, width, height, 0);
        map
    }

    pub fn recursive_maze(&mut self, x: u32, y: u32, width: u32, height:u32, depth: u32) {
        println!("Building maze with x: {}, y: {}, width: {}, height: {}", x, y, width, height);
        if width <= 3 || height <= 3 || depth == 4 {
            return;
        }

        let mut rng = thread_rng();


        // Determine color for this split and save it.
        let color_n = self.color_mapping.len();
        self.color_mapping.insert(color_n, Color { r: rng.gen(), g: rng.gen(), b: rng.gen(), a: 1.0 } );

        if width > height {
            println!("Splitting vertical.");
            // Split in vertical
            let x_split = rng.gen_range(x+2, x + width-2);
            // Draw wall
            for i in y..(y+height) {
                self.data[(x_split + i * self.width) as usize] = color_n;
            }
            let new_width = width - x_split;

            for (i, _) in self.data.iter().enumerate().step_by(16 as usize) {
                let slice = &self.data[i..(i+self.width as usize)];
                println!("{:?}", slice);
            }
            // Recurse left
            self.recursive_maze(x_split, y, new_width, height, depth+1);
            // Recurse right
            self.recursive_maze(x, y, width - new_width, height, depth+1);
        } else {
            println!("Splitting horizontal.");
            let y_split = rng.gen_range(y+2, y + height-2);
            for i in x..(x+width) {
                self.data[(i + y_split * self.height) as usize] = color_n;
            }
            let new_height = height - y_split;

            for (i, _) in self.data.iter().enumerate().step_by(16 as usize) {
                let slice = &self.data[i..(i+self.width as usize)];
                println!("{:?}", slice);
            }

            //Recurse top
            self.recursive_maze(x, y, width, height - new_height, depth+1);
            // Recurse bottom
            self.recursive_maze(x, y_split, width, new_height, depth+1);
        }

    }

    pub fn render_map(&self, window: &mut Window) {
        // Draw top view map
        for (i, v) in self.data.iter().enumerate() {
            let y = ((i as u32) / self.width) * (512 / self.height);
            let x = ((i as u32) % self.width) * (512 / self.width);

            window.draw(&Rectangle::new(Vector::new(x, y), Vector::new(512/self.width, 512 / self.height)),
                        Col(*self.color_mapping.get(v).unwrap()));
        }

    }

    pub fn render_world(&self, player_position:(f32, f32), player_angle: f32, window: &mut Window) {
        // Render world.
        for i in 0..512 {
            let mut draw_color: Color = Color::WHITE;
            let mut c = 0.0;
            let mut x = 0.0;
            let mut y = 0.0;
            let fov = std::f32::consts::PI/3.0;
            let angle = player_angle - fov / 2.0 + fov * (i as f32) / 512.0;
            while c < 20.0 {
                x = player_position.0 + c * angle.cos();
                y = player_position.1 + c * angle.sin();
                let index = ((x as u32) + (y as u32) * self.width) as usize;
                if self.data[index] != 0 {
                    draw_color = *self.color_mapping.get(&self.data[index]).unwrap();
                    break;
                }
                c+=0.01;
            }

            
            // Draw the rendered world line
            let mut draw_height = 512.0/c;
            if draw_height < 0.0 { draw_height = 0.0;}
            let start = Vector::new(512 + i, 256.0 - draw_height );
            let end = Vector::new(512 + i, 256.0 + draw_height);
            window.draw(&Line::new(start, end).with_thickness(1.0),
                        Col(draw_color));

            // Drawy viewcone.
           window.draw(&Line::new(Vector::new(x * (512/self.width) as f32, y * (512/self.height) as f32), Vector::new(player_position.0 * (512/self.width) as f32, player_position.1 * (512/self.height) as f32)).with_thickness(1.0),
                        Col(Color::BLACK));
        }
    }
}