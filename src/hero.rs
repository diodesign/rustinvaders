/* Space invaders in Rust
 *
 * Player graphics
 *
 * Game concept by Tomohiro Nishikado / Taito
 * Rust code By Chris Williams <diodesign@tuta.io>
 *
 * Written for fun. See LICENSE.
 *
 */

extern crate glfw;
extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;

use std::time::Instant;
use na::{ Translation3, UnitQuaternion, Vector3 };
use kiss3d::window::Window;
use kiss3d::scene::SceneNode;

use super::bullet;
use super::collision;
use super::aliens::random_explosion_vector;

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

/* when the ship explodes, we need to animate its debris particles */
struct Debris
{
  node: SceneNode, /* the object in the game world */
  x: f32, y: f32, z: f32 /* movement vector */
}

/* Player has 3 game states: alive, exploding, or dead */
#[derive(PartialEq)]
pub enum State
{
  Alive,   /* playing normally */
  Dying,   /* exploding in death */
  Dead     /* finished exploding, reseting to alive */
}

pub struct Hero
{
  x: f32, y: f32, z: f32,            /* game world coords of the hero's ship */
  ship: SceneNode,                   /* the ship in the graphics context */
  time_of_death: Option<Instant>,    /* when the hero started dying */
  debris: Vec<Debris>,               /* vector array of debris particles when dying */
  pub state: State,                  /* whether the hero is alive, exploding or dead */
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
      bullet: None,
      time_of_death: None,
      debris: Vec::new()
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

    for particle in self.debris.iter_mut()
    {
      particle.node.unlink();
    }
  }

  /* start blowing up the ship */
  pub fn destroy(&mut self, mut window: &mut Window)
  {
    self.time_of_death = Some(Instant::now());
    self.state = State::Dying;

    /* create particles of exploding debris */
    let mut rnd = rand::thread_rng();
    for _ in 0..20
    {
      let mut particle = Debris
      {
        node: window.add_cube(2.0, 2.0, 2.0),
        x: random_explosion_vector(&mut rnd),
        y: random_explosion_vector(&mut rnd).abs(), /* only explode upwards */
        z: random_explosion_vector(&mut rnd),
      };

      /* color the debris a firey red and move it into position of the player's ship */ 
      particle.node.set_color(1.0, 0.2, 0.2);
      particle.node.append_translation(&Translation3::new(self.x, self.y, self.z));

      self.debris.push(particle);
    }
  }

  /* animate the ship exploding or its bullet */
  pub fn animate(&mut self)
  {
    /* if the ship is blowing up then keep it hidden, otherwise visible */
    match self.state
    {
      State::Alive => self.ship.set_visible(true),
      State::Dying =>
      {
        /* continue blowing up the ship */
        self.ship.set_visible(false);
        self.explode();

        /* after 5 seconds, prepare to ressurrect the hero and also
         * delete all the flying debris */
        if self.time_of_death.unwrap().elapsed().as_secs() > 4
        {
          self.state = State::Dead;
          while self.debris.len() > 0
          {
            let mut particle = self.debris.pop();
            particle.unwrap().node.unlink();
          }
        }
      },
      State::Dead =>
      {
        /* prepare to ressurrect the ship. if we're out of lives, let
         * the main game loop catch that */
        self.time_of_death = None;
        self.state = State::Alive;
      }
    }

    if self.bullet.is_some() == true
    {
      /* animate the bullet */
      self.bullet.as_mut().unwrap().animate();
    }
  }

  /* create debris and animate them when the ship explodes */
  fn explode(&mut self)
  {
    let rotate = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.10);
    for particle in self.debris.iter_mut()
    {
      particle.node.append_translation(&Translation3::new(particle.x, particle.y, particle.z));
      particle.node.prepend_to_local_rotation(&rotate);
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

  /* check to see if the ship has collided with a thing at x,y.
   * note: this check does *NOT* affect the ship */
  pub fn collision(&mut self, x: f32, y: f32) -> collision::CollisionOutcome
  {
    let scenario = collision::Collision
    {
      a: collision::CollisionObject{ x: x, y: y },
      b: collision::CollisionObject{ x: self.x, y: self.y }
    };

    return collision::check(scenario);
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

