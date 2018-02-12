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

pub const BULLET_Y_START: f32 = HERO_Y_BASE + (HERO_HEIGHT / 2.0);
pub const BULLET_RADIUS:  f32 = 2.0;
pub const BULLET_COLOR_R: f32 = 1.0;
pub const BULLET_COLOR_G: f32 = 0.0;
pub const BULLET_COLOR_B: f32 = 0.0;
pub const BULLET_ASCENT:  f32 = 2.0;

/* Player has 3 game states: alive, exploding, or dead */
pub enum State
{
  Alive,
  Dying,
  Dead
}

/* define how a player's ship should be removed from the playfield */
pub enum Destruction
{
  Explode,  /* make it blow up */
  NoExplode /* just remove it immediately */
}

pub struct Hero
{
  x: f32, y: f32, z: f32,   /* world coords of the hero's ship */
  ship: Option<SceneNode>,  /* the ship in the graphics context */
  state: State
}

impl Hero
{
  /* create a new ship model at the given point on the x axis */
  pub fn new(window: &mut Window, x: f32) -> Hero
  {
    let mut hero = Hero
    {
      state: State::Alive,
      x: x, y: HERO_Y_BASE, z: 0.0,
      ship: Some(window.add_cone(HERO_RADIUS, HERO_HEIGHT))
    };
    
    hero.ship.as_mut().unwrap().append_translation(&Translation3::new(hero.x, hero.y, hero.z));
    hero.ship.as_mut().unwrap().set_color(HERO_GRAY, HERO_GRAY, HERO_GRAY);

    return hero;
  }

  /* blow up the ship */
  pub fn destroy(&mut self, mode: Destruction)
  {
    self.ship.as_mut().unwrap().unlink();
    match mode
    {
      Destruction::Explode => self.state = State::Dying,
      Destruction::NoExplode => self.state = State::Dead
    }
  }

  /* animate the ship explode */
  pub fn animate(&self)
  {
  }

  /* returns Some(x, y, z) coords of the ship */
  pub fn get_coords(&self) -> (f32, f32, f32)
  {
    (self.x, self.y, self.z)
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

