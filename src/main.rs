use raylib::prelude::*;
mod sim;
use sim::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("N-Body")
        .build();
    rl.set_target_fps(60);

    // Make planets
    let mut planets = Vec::new();
    planets.push(Planet::new(1000000.0, 100.0, 100.0, 0));
    planets.push(Planet::new(1000000.0, 100.0, 200.0, 1));

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        for p in planets.iter() {
            d.draw_circle(p.x as i32, p.y as i32, p.mass as f32 / 100000.0, Color::BLUE)
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