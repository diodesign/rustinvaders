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

/* Aliens have 3 game states: alive, exploding, or dead */
pub enum State
{
  Alive,
  Dying,
  Dead
}

enum Frame
{
  Base,
  Translated
}

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

pub struct Alien
{
  pixels: Vec<Pixel>,
  model: SceneNode,
  frame: Frame,
  state: State,
  last_time: Instant,
  time_of_death: Option<Instant>,
  rng: rand::ThreadRng
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

      /* attach all the pixels together as a group */
      model: window.add_group(),

      /* start off in normal animation frame */
      frame: Frame::Base,

      state: State::Alive,
      last_time: Instant::now(), 
      time_of_death: None,
      rng: rand::thread_rng()
    }
  }

  /* calling new() just initializes the alien. call spawn() to actually create it on screen
   * => center_x, center_y, center_z = coords for the center of the alien model
   *    angle = y-axis rotation angle to apply to the alien */
  pub fn spawn(&mut self, center_x: f32, center_y: f32, center_z: f32, angle: f32)
  {
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

  for y in -2..3
  {
    for x in -6..5
    {
      let mut baddie = Alien::new(&mut window);
      let rotation = 0.4 * ((x + y) as f32);
      baddie.spawn(x as f32 * 13.0, y as f32 * 10.0, 0.0, rotation);
      baddies.push(baddie);
    }
  }

  return baddies;
}

