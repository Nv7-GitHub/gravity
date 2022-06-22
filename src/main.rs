use raylib::prelude::*;
mod sim;
use sim::*;
use rand::Rng;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;

const START_WIDTH: i32 = 700;
const START_HEIGHT: i32 = 700;

const SPEED: i32 = 5;
const SCALESPEED: f32 = 1.05;
const TRAIL_LEN: usize = 25;
const TRAIL_WIDTH: f32 = 0.8;
const TRAIL_MIN_DIST: f64 = 10.0;

const MOUSE_MASS: f64 = 2500000.0;
const PLANETS: usize = 100;

fn apply_scale(val: f64, off: i32, scale: f32) -> f32 {
    (((val as i32) + off) as f32) * scale
}

fn calc_speed(scale: f32) -> i32 {
    ((SPEED as f32) / scale) as i32
}

#[derive(Copy, Clone, Debug)]
pub struct TrailPoint(f64, f64);

impl TrailPoint {
    pub fn vector(&self, offx: i32, offy: i32, scale: f32) -> Vector2 {
        Vector2::new(apply_scale(self.0, offx, scale), apply_scale(self.1, offy, scale))
    }

    pub fn largedist(&self, other: &TrailPoint) -> bool {
        let xdiff = self.0 - other.0;
        let ydiff = self.1 - other.1;
        xdiff * xdiff + ydiff * ydiff > TRAIL_MIN_DIST * TRAIL_MIN_DIST
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(START_WIDTH, START_HEIGHT)
        .title("N-Body")
        .resizable()
        .build();
    rl.set_target_fps(60);

    // Make planets
    let mut planets = Vec::new();
    let mut colors: Vec<Color> = Vec::new();
    let mut trails: Vec<Vec<TrailPoint>> = Vec::new();
    let mut rng = rand::thread_rng();

    // Make mouse planet
    planets.push(Planet::new(MOUSE_MASS, (START_WIDTH/2) as f64, (START_HEIGHT/2) as f64, 0));
    colors.push(Color::WHITE);
    trails.push(Vec::new());
    for i in 0..PLANETS {
        planets.push(Planet::new(rng.gen_range(100000.0..2500000.0), rng.gen_range(0.0..START_WIDTH as f64), rng.gen_range(0.0..START_HEIGHT as f64), i + 1));
        colors.push(Color::new(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255), 255));
        trails.push(Vec::new())
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
        
        if rl.is_mouse_button_down(MOUSE_LEFT_BUTTON) {
            let x = rl.get_mouse_x() as f32 / scale - offx as f32;
            let y = rl.get_mouse_y() as i32 as f32 / scale - offy as f32;
            planets[0].x = x as f64;
            planets[0].y = y as f64;
            planets[0].velx = 0.0;
            planets[0].vely = 0.0;
            trails[0] = Vec::new();
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

        // Save trail
        for p in planets.iter() {
            if trails[p.id].len() > 2 {
                let pt = TrailPoint(p.x, p.y);
                if pt.largedist(trails[p.id].last().unwrap()) {
                    trails[p.id].push(pt);
                }

                if trails[p.id].len() > TRAIL_LEN {
                    trails[p.id].remove(0);
                }
            } else {
                trails[p.id].push(TrailPoint(p.x, p.y));
            }
        }

        let mut d = rl.begin_drawing(&thread);

        // Draw planets
        d.clear_background(Color::BLACK);
        for p in planets.iter() {
            // Draw trail
            if trails[p.id].len() > 2 {
                let mut prev: TrailPoint = trails[p.id][0];
                for el in trails[p.id].iter().skip(1) {
                    d.draw_line_ex(prev.vector(offx, offy, scale), el.vector(offx, offy, scale), p.radius() as f32 * 2.0 * TRAIL_WIDTH * scale, colors[p.id]);
                    d.draw_circle_v(el.vector(offx, offy, scale), p.radius() as f32 * TRAIL_WIDTH * scale, colors[p.id]);
                    prev = *el;
                }
                // Draw line from start of trail to planet
                let el = TrailPoint(p.x, p.y);
                d.draw_line_ex(prev.vector(offx, offy, scale), el.vector(offx, offy, scale), p.radius() as f32 * 2.0 * TRAIL_WIDTH * scale, colors[p.id]);
            }
            // Draw planet
            d.draw_circle(apply_scale(p.x, offx, scale) as i32, apply_scale(p.y, offy, scale) as i32, p.radius() as f32 * scale, colors[p.id]);
        }

        // Draw FPS
        d.draw_fps(10, 10);
    }
}