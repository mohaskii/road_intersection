use lazy_static::lazy_static;
use rand::Rng;
pub use sdl2::pixels::Color;
#[allow(non_snake_case)]
pub use sdl2::rect::Rect;
pub use sdl2::render::Canvas;
pub use sdl2::video::Window;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref Cars: Mutex<Vec<car>> = Mutex::new(Vec::new());
    static ref DIRECTION_COLOR_MAP: HashMap<dir, Color> = {
        let mut m = HashMap::new();

        m.insert(dir::left, Color::RED);
        m.insert(dir::right, Color::GREEN);
        m.insert(dir::top, Color::BLUE);
        m.insert(dir::bottom, Color::YELLOW);
        m
    };
    static ref velocity: i32 = 5;
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum dir {
    left,
    right,
    top,
    bottom,
}
#[derive(Debug)]
pub struct car {
    origin: dir,
    direction: dir,
    color: Color,
    // changeDir: bool,
    coords: (i32, i32),
}
pub fn moveAllcars(canvas: &mut Canvas<Window>) {
    {
        for car in Cars.lock().unwrap().iter_mut() {
            match car.origin {
                dir::left => car.coords.0 += velocity.clone(),
                dir::bottom => car.coords.1 -= velocity.clone(),
                dir::right => car.coords.0 -= velocity.clone(),
                dir::top => car.coords.1 += velocity.clone(),
                _ => (),
            };
            car.draw(canvas);
        }
    }
}
impl car {
    pub fn newCar(origin: dir, canvas: &mut Canvas<Window>) {
        let (width, height) = canvas.output_size().unwrap();
        let mut _2 = [dir::left, dir::right];
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut _2);
        let direction = _2[0];
        let color = *DIRECTION_COLOR_MAP.get(&direction).unwrap();
        let midHeight = (height / 2) as i32;
        let midwidth = (width / 2) as i32;

        let coord = {
            match origin {
                dir::right => ((width - 80) as i32, midHeight),
                dir::left => (0, midHeight),
                dir::top => (midwidth - 80, 0),
                dir::bottom => (midwidth, (height as i32) - 68),
                _ => (0, 0),
            }
        };

        {
            let mut cars = Cars.lock().unwrap();
            cars.push(car {
                origin,
                direction,
                color,
                // changeDir: false,
                coords: coord,
            });

            println!(
                "{:?}",
                car {
                    origin,
                    direction,
                    color,
                    coords: coord,
                }
            )
        }
    }

    pub fn move_vehicle(&mut self, canvas: &mut Canvas<Window>) {
        // Logique de déplacement du véhicule
        // Vous pouvez ajuster la position en fonction de la direction
    }
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color); // Rouge pour la voiture
        let rect = Rect::new(self.coords.0, self.coords.1, 80, 68);
        canvas.fill_rect(rect).expect("Failed to draw car");
    }
}
