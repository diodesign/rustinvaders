/* Space invaders in Rust
 *
 * Player graphics
 *
 * Game concept by Tomohiro Nishikado / Taito
 * Rust code By Chris Williams <diodesign@gmail.com>
 *
 * Written for fun. See LICENSE.
 *
 */

extern crate glfw;
extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;

use na::Translation3;
use kiss3d::window::Window;
use kiss3d::scene::SceneNode;

const HERO_HEIGHT:    f32 = 13.0;
const HERO_RADIUS:    f32 = 5.0;
const HERO_Y_BASE:    f32 = -90.0;
const HERO_GRAY:      f32 = 0.8;
const HERO_MOVE_STEP: f32 = 1.0;

/* Player has 3 game states: alive, exploding, or dead */
pub enum State
{
  Alive,
  Dying,
  Dead
}

pub struct Hero
{
  x: f32, y: f32, z: f32,
  ship: Option<SceneNode>,
  state: State,
}

impl Hero
{
  /* allocate and initialize a new alien onject */
  pub fn new() -> Hero
  {
    Hero
    {
      x: 0.0, y: 0.0, z: 0.0,    /* world coords of the hero's ship */
      ship: None,                /* the 'ship' */
      state: State::Alive,
    }
  }

  /* create a new ship model at the given point on the x axis */
  pub fn spawn(&mut self, window: &mut Window, x: f32)
  {
    self.x = x;
    self.y = HERO_Y_BASE;
    self.z = 0.0;
    
    let mut ship = window.add_cone(HERO_RADIUS, HERO_HEIGHT);
    ship.append_translation(&Translation3::new(self.x, self.y, self.z));
    ship.set_color(HERO_GRAY, HERO_GRAY, HERO_GRAY);
    
    self.ship = Some(ship);
  }

  pub fn move_left(&mut self)
  {
    self.move_ship(HERO_MOVE_STEP);
  }

  pub fn move_right(&mut self)
  {
    self.move_ship(0.0 - HERO_MOVE_STEP);
  }

  fn move_ship(&mut self, distance: f32)
  {
    self.x = self.x + distance;
    self.ship.as_mut().unwrap().append_translation(&Translation3::new(distance, 0.0, 0.0));
  }
}

