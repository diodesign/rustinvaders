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

use bullet;
use collision;

const HERO_HEIGHT:     f32 = 13.0;
const HERO_RADIUS:     f32 = 5.0;
const HERO_GRAY:       f32 = 0.8;
const HERO_MOVE_STEP:  f32 = 1.0;
const HERO_Y_BASE: f32 = -90.0;

pub const HERO_Y_FLOOR: f32 = HERO_Y_BASE - (HERO_HEIGHT / 2.0);

const BULLET_Y_START: f32 = HERO_Y_BASE + (HERO_HEIGHT / 2.0);
const BULLET_RADIUS:  f32 = 2.0;
const BULLET_COLOR_R: f32 = 1.0;
const BULLET_COLOR_G: f32 = 0.0;
const BULLET_COLOR_B: f32 = 0.0;
const BULLET_ASCENT:  f32 = 2.0;

/* Player has 3 game states: alive, exploding, or dead */
#[derive(PartialEq)]
pub enum State
{
  Alive,
  Dying,
  Dead
}

pub struct Hero
{
  x: f32, y: f32, z: f32,   /* game world coords of the hero's ship */
  ship: SceneNode,          /* the ship in the graphics context */
  pub state: State,
  pub bullet: Option<bullet::Bullet> /* bullet fired by the ship */
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
      ship: window.add_cone(HERO_RADIUS, HERO_HEIGHT),
      bullet: None
    };
    
    hero.ship.append_translation(&Translation3::new(hero.x, hero.y, hero.z));
    hero.ship.set_color(HERO_GRAY, HERO_GRAY, HERO_GRAY);

    return hero;
  }

  /* make sure everything is removed from the game world */
  pub fn delete(&mut self)
  {
    self.ship.unlink();
    self.destroy_bullet();
  }

  /* start blowing up the ship */
  pub fn destroy(&mut self)
  {
    self.state = State::Dying;
  }

  /* animate the ship exploding or its bullet */
  pub fn animate(&mut self)
  {
    /* if the ship is blowing up then keep it hidden, otherwise visible */
    match self.state
    {
      State::Alive => self.ship.set_visible(true),
      _ => self.ship.set_visible(false)
    }

    if self.bullet.is_some() == true
    {
      /* animate the bullet */
      self.bullet.as_mut().unwrap().animate();
    }
  }

  /* fire a new bullet if one isn't already in play */
  pub fn fire(&mut self, mut window: &mut Window)
  {
    if self.bullet.is_some() == false
    {
      self.bullet = Some(bullet::Bullet::new(&mut window, self.x, BULLET_Y_START,
                                             BULLET_RADIUS, BULLET_COLOR_R, BULLET_COLOR_G,
                                             BULLET_COLOR_B, BULLET_ASCENT));
    }
  }

  /* remove bullet from game */
  pub fn destroy_bullet(&mut self)
  {
    if self.bullet.as_mut().is_some() == true
    {
      self.bullet.as_mut().unwrap().destroy();
      self.bullet = None;
    }
  }

  /* check to see if the ship has collidded with a thing at x,y. if so, then
   * blow up the ship */
  pub fn collision(&mut self, x: f32, y: f32) -> collision::CollisionOutcome
  {
    let scenario = collision::Collision
    {
      a: collision::CollisionObject{ x: x, y: y },
      b: collision::CollisionObject{ x: self.x, y: self.y }
    };

    match collision::check(scenario)
    {
      collision::CollisionOutcome::Hit =>
      {
        self.destroy();
        collision::CollisionOutcome::Hit
      },

      _ => collision::CollisionOutcome::Miss
    }
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
    self.ship.append_translation(&Translation3::new(distance, 0.0, 0.0));
  }
}

