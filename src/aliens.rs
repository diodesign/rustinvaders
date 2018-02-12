/* Space invaders in Rust
 *
 * Alien designs
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

use std::time::Instant;
use rand::Rng;
use na::{Vector3, Translation3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::scene::SceneNode;

/* define collision outcomes */
pub enum Collision
{
  OutOfBounds, /* bullet left playfield */
  HitAlien /* bullet hit an alien */
}

const ALIEN_HEIGHT: f32     = 10.0; /* in 3d units */
const ALIEN_WIDTH: f32      = 13.0; /* in 3d units */
const ALIENS_PER_ROW: i32   = 11;
const ALIEN_ROWS: i32       = 5;
const ALIEN_TOP_Y: i32      = 7;    /* in whole number of aliens from game world center */
const ALIEN_SIDE_SPACE: i32 = 3;    /* space either side (in nr of aliens) of alien pattern */
pub const ALIEN_POINTS: i32 = 100;  /* number of points per alien */

const COLLISION_TOLERANCE: f32 = 5.0; /* objects closer than this are considered collided */

/* aliens are made up of a number of pixels */
struct Pixel
{
  /* dimensions of this pixel */
  width: f32, height: f32, depth: f32,

  /* the pixel's x, y, z base coords and a translation to move the pixel into
     another position. this allows the pixel to be animated by switching between
     its base and translated positions. yes, this could be point structs... soon */
  x:  f32, y:  f32, z:  f32,
  tx: f32, ty: f32, tz: f32,

  r:  f32, g:  f32, b:  f32, /* color of the pixel */
  explode_x: f32, explode_y: f32, explode_z: f32, /* vector describing the pixel's explosion trajectory */
  node: Option<SceneNode> /* this pixel's scene node */
}

/* aliens have 3 game states: alive, exploding, or dead */
#[derive(PartialEq)]
enum State
{
  Alive,
  Dying,
  Dead
}

/* aliens are either shuffling left, right, or down and then right, or down then left */
enum Movement
{
  Left,         /* moving left */
  Right,        /* moving right */
  DownRight,    /* moving down, will go right */
  DownLeft      /* moving down, will go left */
}

/* aliens have 2 animation states: the base design and a slightly modified one */ 
enum Frame
{
  Base,
  Translated
}

pub struct Alien
{
  x: f32, y: f32, z: f32,         /* center of the model on the playfield */
  pixels: Vec<Pixel>,             /* the pixels making up this alien */
  model: SceneNode,               /* the scene node holding all the pixels */
  frame: Frame,                   /* the type of animation frame being displayed */
  state: State,                   /* whether the alien is alive, dead, etc */
  last_time: Instant,             /* last time we animated this alien */
  time_of_death: Option<Instant>, /* when the alien was declared dead */
  rng: rand::ThreadRng,           /* access to the thread's RNG */
  drop_steps: f32,                /* number of units we've moved alien down at end of row */
  movement: Movement              /* the direction the alien is traveling */
}

impl Alien
{
  /* allocate and initialize a new alien onject */
  pub fn new(window: &mut Window) -> Alien
  {
    Alien
    {
      /* describe the alien in blocks of pixels. could load this in as a model but
       * I want to animate this programmatically and I just want to draw something
       * to the screen. i've included the pixels going from left to right, top to
       * bottom, grouping horizontal lines into bars, and leaving individual pixels
       * as is. the overall design is:

           *     *
            *   *   
           *******  
          ** *** ** 
         ***********
         * ******* *
         * *     * *
            ** **       */
           

      pixels: vec!
      [
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x: -3.0, y:  4.0, z: 0.0, tx:  1.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None, explode_x: 0.0, explode_y: 0.0, explode_z: 0.0 },
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x:  3.0, y:  4.0, z: 0.0, tx: -1.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None, explode_x: 0.0, explode_y: 0.0, explode_z: 0.0 },
        
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x: -2.0, y:  3.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None, explode_x: 0.0, explode_y: 0.0, explode_z: 0.0 },
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x:  2.0, y:  3.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None, explode_x: 0.0, explode_y: 0.0, explode_z: 0.0 },

        Pixel { width:  7.0, height: 1.0, depth: 1.0, x:  0.0, y:  2.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None, explode_x: 0.0, explode_y: 0.0, explode_z: 0.0 },

        Pixel { width:  2.0, height: 1.0, depth: 1.0, x: -4.0, y:  1.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None, explode_x: 0.0, explode_y: 0.0, explode_z: 0.0 },
        Pixel { width:  3.0, height: 1.0, depth: 1.0, x:  0.0, y:  1.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None, explode_x: 0.0, explode_y: 0.0, explode_z: 0.0 },
        Pixel { width:  2.0, height: 1.0, depth: 1.0, x:  4.0, y:  1.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None, explode_x: 0.0, explode_y: 0.0, explode_z: 0.0 },

        Pixel { width: 11.0, height: 1.0, depth: 1.0, x:  0.0, y:  0.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None, explode_x: 0.0, explode_y: 0.0, explode_z: 0.0 },

        Pixel { width:  1.0, height: 1.0, depth: 1.0, x:  5.0, y: -1.0, z: 0.0, tx:  0.0, ty: 3.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None, explode_x: 0.0, explode_y: 0.0, explode_z: 0.0 },
        Pixel { width:  7.0, height: 1.0, depth: 1.0, x:  0.0, y: -1.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None, explode_x: 0.0, explode_y: 0.0, explode_z: 0.0 },
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x: -5.0, y: -1.0, z: 0.0, tx:  0.0, ty: 3.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None, explode_x: 0.0, explode_y: 0.0, explode_z: 0.0 },

        Pixel { width:  1.0, height: 1.0, depth: 1.0, x: -5.0, y: -2.0, z: 0.0, tx:  0.0, ty: 3.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None, explode_x: 0.0, explode_y: 0.0, explode_z: 0.0 },
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x: -3.0, y: -2.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None, explode_x: 0.0, explode_y: 0.0, explode_z: 0.0 },
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x:  3.0, y: -2.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None, explode_x: 0.0, explode_y: 0.0, explode_z: 0.0 },
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x:  5.0, y: -2.0, z: 0.0, tx:  0.0, ty: 3.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None, explode_x: 0.0, explode_y: 0.0, explode_z: 0.0 },

        Pixel { width:  2.0, height: 1.0, depth: 1.0, x: -1.5, y: -3.0, z: 0.0, tx: -2.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None, explode_x: 0.0, explode_y: 0.0, explode_z: 0.0 },
        Pixel { width:  2.0, height: 1.0, depth: 1.0, x:  1.5, y: -3.0, z: 0.0, tx:  2.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None, explode_x: 0.0, explode_y: 0.0, explode_z: 0.0 }
      ],

      x: 0.0, y: 0.0, z: 0.0, /* default position of alien model's center */

      /* attach all the pixels together as a group */
      model: window.add_group(),

      /* start off in normal animation frame */
      frame: Frame::Base,

      state: State::Alive,
      last_time: Instant::now(), 
      time_of_death: None,
      rng: rand::thread_rng(),
      drop_steps: 0.0,
      movement: Movement::Right
    }
  }

  /* calling new() just initializes the alien. call spawn() to actually create it on screen
   * => center_x, center_y, center_z = coords for the center of the alien model
   *    angle = y-axis rotation angle to apply to the alien */
  pub fn spawn(&mut self, center_x: f32, center_y: f32, center_z: f32, angle: f32)
  {
    self.x = center_x;
    self.y = center_y;
    self.z = center_z;

    /* spin through the array of pixels to create this monster */
    for pixel in self.pixels.iter_mut()
    {
      /* create a cube pixel aka a scene node */
      let mut p = self.model.add_cube(pixel.width, pixel.height, pixel.depth);

      /* move pixel into position within the alien */
      p.append_translation(&Translation3::new(pixel.x, pixel.y, pixel.z));

      /* color it */
      p.set_color(pixel.r, pixel.g, pixel.b);

      /* keep a record of the pixel's scene node */
      (*pixel).node = Some(p);
    }

    /* move the whole model into position and rotate it as required */
    self.model.append_translation(&Translation3::new(center_x, center_y, center_z));
    self.rotate(angle);
  }

  /* kill off this alien by marking it as dying and calculate how it's going to explode into pieces */
  pub fn die(&mut self)
  {
    /* only aliens still alive can die */
    match self.state
    {
      State::Alive => {},
      _ => return
    };

    self.state = State::Dying;

    /* generate random x,y,z vector for explosion trajectory for this pixel */
    for pixel in self.pixels.iter_mut()
    {
      pixel.explode_x = random_explosion_vector(&mut self.rng);
      pixel.explode_y = random_explosion_vector(&mut self.rng);
      pixel.explode_z = random_explosion_vector(&mut self.rng);
    }

    self.time_of_death = Some(Instant::now());
  }
  
  /* call for each video frame to animate the alien */
  pub fn animate(&mut self)
  {
    /* are we supposed to be exploding or be alive doing stuff? */
    match self.state
    {
      State::Alive =>
      {
        /* rotate the alien slightly */
        self.rotate(0.018);

        /* switch between animation frames every second */
        if self.last_time.elapsed().as_secs() > 1
        {
          self.switch();
          self.last_time = Instant::now();
        }

        let mut tx = 0.0;
        let mut ty = 0.0;

        /* step alien to the left or right or down */
        match self.movement
        {
          Movement::Left => tx = 0.1,
          Movement::Right => tx = -0.1,
          Movement::DownRight | Movement::DownLeft => ty = -0.5
        }

        /* update position of the alien */
        self.x = self.x + tx;
        self.y = self.y + ty;
        self.drop_steps = self.drop_steps + ty;
        self.model.append_translation(&Translation3::new(tx, ty, 0.0));
      },

      State::Dying =>
      {
        self.explode();
      },

      _ => {}
    }
  }

  /* rotate the whole alien model by given angle along y-axis */
  fn rotate(&mut self, angle: f32)
  {
    let rotate = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), angle);
    self.model.prepend_to_local_rotation(&rotate);
  }

  /* call this to switch pixels between their base and translated positions.
   * this allows the alien to have two frames of animation */
  fn switch(&mut self)
  {
    match self.frame
    {
      Frame::Base =>
      {
        /* move pixels into alternate positions, and update alien frame state */
        for pixel in self.pixels.iter_mut()
        {
          pixel.node.as_mut().unwrap().append_translation(&Translation3::new(pixel.tx, pixel.ty, pixel.tz));
        }
        self.frame = Frame::Translated;
      },

      Frame::Translated =>
      {
        /* move pixels back to normal positions, and update alien frame state */
        for pixel in self.pixels.iter_mut()
        {
          pixel.node.as_mut().unwrap().append_translation(&Translation3::new(pixel.tx * -1.0, pixel.ty * -1.0, pixel.tz * -1.0));
        }
        self.frame = Frame::Base;
      }
    };
  }
  
  /* animate blowing up the alien: scatter its compoents, spinning them, and then delete them */
  fn explode(&mut self)
  {
    let rotate = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.05);
    let secs_since_death = self.time_of_death.unwrap().elapsed().as_secs();
    
    for pixel in self.pixels.iter_mut()
    {
      pixel.node.as_mut().unwrap().append_translation(&Translation3::new(pixel.explode_x, pixel.explode_y, pixel.explode_z));
      pixel.node.as_mut().unwrap().prepend_to_local_rotation(&rotate);
   
      /* change color of the pixel based on seconds passed */ 
      match secs_since_death
      {
        0 | 1 => pixel.node.as_mut().unwrap().set_color(1.0, 0.4, 0.0),
            2 => pixel.node.as_mut().unwrap().set_color(1.0, 0.6, 0.0),
            3 => pixel.node.as_mut().unwrap().set_color(1.0, 0.8, 0.0),
            4 => pixel.node.as_mut().unwrap().set_color(1.0, 1.0, 0.0),
            5 => pixel.node.as_mut().unwrap().set_color(0.8, 0.8, 0.0),
            6 => pixel.node.as_mut().unwrap().set_color(0.6, 0.6, 0.0),
            7 => pixel.node.as_mut().unwrap().set_color(0.4, 0.4, 0.0),
            _ => pixel.node.as_mut().unwrap().set_color(0.2, 0.2, 0.0),
      };
    }

    /* after a period of seconds, wipe away the remains: mark all components of the alien invisible,
     * unlink them from the scene, and mark the alien as dead. */
    if secs_since_death > 20
    {
      for pixel in self.pixels.iter_mut()
      {
        pixel.node.as_mut().unwrap().set_visible(false);
        pixel.node.as_mut().unwrap().unlink();
      }
      self.state = State::Dead;
    }
  }
}
  
/* generate a random value suitable for exploding a pixel */
fn random_explosion_vector(rng: &mut rand::ThreadRng) -> f32
{
  if rng.gen()
  {
    return rng.gen_range(-0.5f32, -0.1f32);
  }
  
  return rng.gen_range(0.1f32, 0.5f32);
}

/* construct a playfield of aliens, pass it back as a vector of aliens */
pub fn spawn_playfield(mut window: &mut Window) -> Vec<Alien>
{
  let mut baddies = Vec::<Alien>::with_capacity(55);

  /* generate a formation ALIENS_PER_ROW number of aliens wide, centered
   * on the x-axis, and ALIEN_ROWS number of aliens tall, from ALIEN_TOP_Y downwards.
   * ALIEN_TOP_Y is in whole number of aliens from the center of the game world  */ 

  for y in (ALIEN_TOP_Y - ALIEN_ROWS)..ALIEN_TOP_Y
  {
    for x in 0 - (ALIENS_PER_ROW / 2)..(ALIENS_PER_ROW / 2) + 1
    {
      let mut baddie = Alien::new(&mut window);
      let rotation = 0.4 * ((x + y) as f32);
      baddie.spawn(x as f32 * ALIEN_WIDTH, y as f32 * ALIEN_HEIGHT, 0.0, rotation);
      baddies.push(baddie);
    }
  }

  return baddies;
}

/* update the positions of the aliens and check to see if any collided with the invisible walls
 * which causes them to move down a row and reverse movement */
pub fn animate_playfield(aliens: &mut Vec<Alien>)
{
  let mut hit_wall_right = false;
  let mut hit_wall_left = false;

  /* move the aliens and check for collision with side walls */
  for baddie in aliens.iter_mut().filter(|f| f.state != State::Dead)
  {
    /* animate and move the alien */
    baddie.animate();

    /* if we're moving left or right, check to see if we hit a wall */
    match baddie.movement
    {
      Movement::Left | Movement::Right =>
      {
        /* did the baddie just collide with a wall om the left? */
        if baddie.x > ((ALIENS_PER_ROW / 2) + 1 + ALIEN_SIDE_SPACE) as f32 * ALIEN_WIDTH
        {
          hit_wall_left = true;
        }
        
        /* did the baddie just collide with a wall om the right? */
        if baddie.x < ((ALIENS_PER_ROW / 2) + ALIEN_SIDE_SPACE) as f32 * (0.0 - ALIEN_WIDTH)
        {
          hit_wall_right = true;
        }
      },

      /* if we're going down then make sure we don't go down too far - just one row */
      Movement::DownLeft =>
      {
        if baddie.drop_steps < 0.0 - ALIEN_HEIGHT
        {
          baddie.movement = Movement::Left
        }
      },
      
      Movement::DownRight =>
      {
        if baddie.drop_steps < 0.0 - ALIEN_HEIGHT
        {
          baddie.movement = Movement::Right
        }
      }
    }
  }

  /* if one or more of the aliens hit a side wall, then change their directions so they're
   * all moving downwards */
  if hit_wall_right == true
  {
    for faller in aliens.iter_mut()
    {
      faller.drop_steps = 0.0;
      faller.movement = Movement::DownLeft; /* go down then left */
    }
  }
  if hit_wall_left == true
  {
    for faller in aliens.iter_mut()
    {
      faller.drop_steps = 0.0;
      faller.movement = Movement::DownRight; /* go down then left */
    }
  }
}

/* check to see if a bullet has left the playfield or hit an alien.
 * if an alien is hit then blow it up and remove it */
pub fn detect_bullet_collision(aliens: &mut Vec<Alien>, bullet_x: f32, bullet_y: f32) -> Option<Collision>
{
  /* bullet is dead as soon as it's out of area */
  if bullet_y > (ALIEN_TOP_Y as f32 * ALIEN_HEIGHT)
  {
    return Some(Collision::OutOfBounds);
  }

  /* check through all the aliens */
  for baddie in aliens.iter_mut().filter(|f| f.state == State::Alive)
  {
    if detect_collision(bullet_x, bullet_y, baddie.x, baddie.y) == true
    {
      baddie.die();
      return Some(Collision::HitAlien);
    }
  }

  return None; /* nothing happened */
}

/* check to see if an alien (in the array aliens) has hit the player (x, y) craft */
pub fn detect_ship_collision(aliens: &mut Vec<Alien>, x: f32, y: f32) -> bool
{
  for baddie in aliens.iter_mut().filter(|f| f.state == State::Alive)
  {
    /* if an alien sneaks under the player then it's game over */
    if baddie.y < y || detect_collision(baddie.x, baddie.y, x, y) == true
    {
      return true;
    }
  }

  return false;
}
 
/* returns true if object with coords ax,ay is very close to or in direct collision with
 * an object at coords bx,by. no z coord comparison, of course */
fn detect_collision(ax: f32, ay: f32, bx: f32, by: f32) -> bool
{
  if ax < bx + COLLISION_TOLERANCE && ax > bx - COLLISION_TOLERANCE &&
     ay < by + COLLISION_TOLERANCE && ay > by - COLLISION_TOLERANCE
  {
    return true;
  }

  return false;
}

/* return number of aliens still alive in the array of aliens */
pub fn still_alive(aliens: &mut Vec<Alien>) -> i32
{
  let mut count = 0;
  for _ in aliens.iter().filter(|f| f.state == State::Alive)
  {
    count = count + 1;
  }
  return count;
}


