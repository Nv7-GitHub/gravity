use raylib::prelude::*;
mod sim;
use sim::*;
use rand::Rng;

const WIDTH: i32 = 700;
const HEIGHT: i32 = 700;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("N-Body")
        .build();
    rl.set_target_fps(60);

    // Make planets
    let mut planets = Vec::new();
    let mut rng = rand::thread_rng();
    for i in 0..10 {
        planets.push(Planet::new(rng.gen_range(1000000.0..10000000.0), rng.gen_range(0.0..WIDTH as f64), rng.gen_range(0.0..HEIGHT as f64), i));
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        for p in planets.iter() {
            d.draw_circle(p.x as i32, p.y as i32, p.radius() as f32, Color::BLUE)
        }

        // Simulate
        let mut new_planets = vec![Planet::new(0.0, 0.0, 0.0, 0); planets.len()];
        for (i, p) in planets.iter().enumerate() {
            let mut p = *p;
            p.sim(&planets);
            new_planets[i] = p;
        }
        planets = new_planets;

        // Draw FPS
        d.draw_fps(10, 10);
    }
}