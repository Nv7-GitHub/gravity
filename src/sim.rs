#[derive(Copy, Clone)]
pub struct Planet {
  pub mass: f64,
  
  pub velx: f64,
  pub vely: f64,

  pub x: f64,
  pub y: f64,
  pub id: usize,
}

const G: f64 = 6.67430e-11f64;
const SOFTEN: f64 = 5.0;

impl Planet {
  pub fn new(mass: f64, x: f64, y: f64, id: usize) -> Self {
    Self {
      mass, x, y, id, velx: 0.0, vely: 0.0
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
      let ang = ydiff.atan2(xdiff);

      // Calculate force
      let dist_squared = ydiff * ydiff + xdiff * xdiff;
      //let force = (G * self.mass * p.mass) / dist_squared;
      let force = (G * self.mass * p.mass) / (dist_squared + SOFTEN).powf(1.5);

      // Apply force
      let force_x = force * ang.cos();
      let force_y = force * ang.sin();
      self.velx += force_x;
      self.vely += force_y;
    }

    // Integrate
    self.x += self.velx;
    self.y += self.vely;
  }

  pub fn collide(&mut self, planets: &mut Vec<Planet>) {
    for p in planets {
      if p.id == self.id {
        continue;
      }

      // Calculate dist
      let ydiff = p.y - self.y;
      let xdiff = p.x - self.x;
      let dist = (ydiff*ydiff + xdiff*xdiff).sqrt();

      if dist > self.radius() + p.radius() {
        // Not colliding
        continue
      }

      // Calculate angle
      let ang = ydiff.atan2(xdiff);

      // Calculate force
      let force = (dist - (self.radius() + p.radius())) / 2.0;

      // Apply
      let force_x = force * ang.cos();
      let force_y = force * ang.sin();
      self.x += force_x;
      self.y += force_y;
    }
  }
}