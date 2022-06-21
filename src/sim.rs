#[derive(Copy, Clone)]
pub struct Planet {
  pub mass: f64,
  pub x: f64,
  pub y: f64,

  id: usize,
}

const G: f64 = 6.67430e-11f64;

impl Planet {
  pub fn new(mass: f64, x: f64, y: f64, id: usize) -> Self {
    Self {
      mass, x, y, id
    }
  }

  pub fn radius(&self) -> f64 {
    return self.mass / 100000.0;
  }

  pub fn sim(&mut self, planets: &Vec<Planet>) {
    for p in planets {
      if p.id == self.id {
        continue;
      }

      // Calculate angle
      let ydiff = p.y - self.y;
      let xdiff = p.x - self.x;
      let ang = (ydiff/xdiff).atan();

      // Calculate force
      let dist_squared = ydiff * ydiff + xdiff * xdiff;
      let mut force = (G * self.mass * p.mass) / dist_squared;

      // Check for collision
      let dist = dist_squared.sqrt();
      if (dist - force) < (self.radius() + p.radius())  { // Force minus distance between 2 circles is greater than the radii's added
        force = dist - (self.radius() + p.radius());
      }

      // Apply force
      let force_x = force * ang.cos();
      let force_y = force * ang.sin();
      self.x += force_x;
      self.y += force_y;
    }
  }
}