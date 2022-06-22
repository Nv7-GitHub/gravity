use raylib::prelude::*;
mod sim;
use sim::*;
use rand::Rng;
use raylib::consts::KeyboardKey::*;

const WIDTH: i32 = 700;
const HEIGHT: i32 = 700;
const SPEED: i32 = 5;
const SCALESPEED: f32 = 1.05;

fn apply_scale(val: i32, scale: f32) -> i32 {
    ((val as f32) * scale) as i32
}

fn calc_speed(scale: f32) -> i32 {
    ((SPEED as f32) / scale) as i32
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("N-Body")
        .build();
    rl.set_target_fps(60);

    // Make planets
    let mut planets = Vec::new();
    let mut colors: Vec<Color> = Vec::new();
    let mut rng = rand::thread_rng();
    for i in 0..25 {
        planets.push(Planet::new(rng.gen_range(100000.0..2500000.0), rng.gen_range(0.0..WIDTH as f64), rng.gen_range(0.0..HEIGHT as f64), i));
        colors.push(Color::new(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255), 255));
    }

    // Transform
    let mut offx = 0;
    let mut offy = 0;
    let mut scale: f32 = 1.0;

    while !rl.window_should_close() {
        // Key presses
        if rl.is_key_down(KEY_A) {
            offx += calc_speed(scale);
        }
        if rl.is_key_down(KEY_D) {
            offx -= calc_speed(scale);
        }
        if rl.is_key_down(KEY_W) {
            offy += calc_speed(scale);
        }
        if rl.is_key_down(KEY_S) {
            offy -= calc_speed(scale);
        }
        if rl.is_key_down(KEY_UP) {
            scale *= SCALESPEED;
        }
        if rl.is_key_down(KEY_DOWN) {
            scale /= SCALESPEED;
        }
        

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        for p in planets.iter() {
            d.draw_circle(apply_scale(p.x as i32 + offx, scale), apply_scale(p.y as i32 + offy, scale), p.radius() as f32 * scale, colors[p.id])
        }

        // Simulate
        let mut new_planets = vec![Planet::new(0.0, 0.0, 0.0, 0); planets.len()];
        for (i, p) in planets.iter().enumerate() {
            let mut p = *p;
            p.sim(&planets);
            new_planets[i] = p;
        }
        planets = new_planets;

        // Collide
        let mut new_planets = planets.clone();
        for (i, p) in planets.iter().enumerate() {
            let mut p = *p;
            p.collide(&mut new_planets);
            new_planets[i] = p;
        }
        planets = new_planets;

        // Draw FPS
        d.draw_fps(10, 10);
    }
}