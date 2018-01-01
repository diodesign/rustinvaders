/* Space invaders in Rust
 *
 * Game concept by Tomohiro Nishikado / Taito
 * Rust code By Chris Williams <diodesign@gmail.com>
 *
 * Written for fun. See LICENSE.
 *
 */

extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;

use std::time::Instant;

use na::{Vector3, Translation3, UnitQuaternion, Point3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::camera::ArcBall;
use rand::Rng;

/* Aliens have 2 animation frames: normal and an alternative pose */
enum Frame
{
  Normal,
  Alternate
}

/* Aliens have 3 game states: alive, exploding, or dead */
enum State
{
  Alive,
  Dying,
  Dead
}

struct Pixel
{
  width: f32, height: f32, depth: f32,
  x:  f32, y:  f32, z:  f32, /* normal x,y,z frame positions */
  tx: f32, ty: f32, tz: f32, /* alternate x,y,z frame translations */
  r:  f32, g:  f32, b:  f32, /* color of the pixel */
  explode_x: f32, explode_y: f32, explode_z: f32, /* vector describing the pixel's explosion trajectory */
  node: Option<SceneNode> /* this pixel's scene node */
}

struct Alien
{
  pixels: Vec<Pixel>,
  model: SceneNode,
  animation_frame: Frame,
  state: State,
  time_of_death: Option<Instant>
}

impl Alien
{
  fn new(window: &mut Window) -> Alien
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
      animation_frame: Frame::Normal,

      state: State::Alive,
      time_of_death: None,
    }
  }

  fn spawn(&mut self, center_x: f32, center_y: f32, center_z: f32)
  {
    /* spin through the array of pixels to create this monster */
    for pixel in self.pixels.iter_mut()
    {
      /* create a cube pixel aka a scene node */
      let mut p = self.model.add_cube(pixel.width, pixel.height, pixel.depth);

      /* move it into position */
      p.append_translation(&Translation3::new(pixel.x + center_x, pixel.y + center_y, pixel.z + center_z));

      /* color it */
      p.set_color(pixel.r, pixel.g, pixel.b);

      /* keep a record of the pixel's scene node */
      (*pixel).node = Some(p);
    }
  }

  /* call this to switch pixels between their normal state and their
   * alternative state. this allows the alien to have two frames of animation:
   * normal and alternative. */
  fn animate(&mut self)
  {
    match self.animation_frame
    {
      Frame::Normal =>
      {
        /* move pixels into alternate positions, and update alien frame state */
        for pixel in self.pixels.iter_mut()
        {
          pixel.node.as_mut().unwrap().append_translation(&Translation3::new(pixel.tx, pixel.ty, pixel.tz));
        }
        self.animation_frame = Frame::Alternate;
      },

      Frame::Alternate =>
      {
        /* move pixels back to normal positions, and update alien frame state */
        for pixel in self.pixels.iter_mut()
        {
          pixel.node.as_mut().unwrap().append_translation(&Translation3::new(pixel.tx * -1.0, pixel.ty * -1.0, pixel.tz * -1.0));
        }
        self.animation_frame = Frame::Normal;
      }
    };
  }

  /* the aliens oscillate, rotating clockwise slightly then anticlockwise the other way, repeating this movement
   * as they slide along the screen. use this code to step the alien one way or the other
   * => direction = 1 to move clockwise, -1 to move anticlockwise */
  fn spin(&mut self, direction: f32)
  {
    let rotate = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.016 * direction);
    self.model.prepend_to_local_rotation(&rotate);
  }

  /* kill off this alien by marking it as dying and calculate how it's going to explode into pieces */
  fn die(&mut self, rng: &mut rand::ThreadRng)
  {
    self.state = State::Dying;

    /* generate random x,y,z vector for explosion trajectory for this pixel */
    for pixel in self.pixels.iter_mut()
    {
      pixel.explode_x = random_explosion_vector(rng);
      pixel.explode_y = random_explosion_vector(rng);
      pixel.explode_z = random_explosion_vector(rng);
    }

    self.time_of_death = Some(Instant::now());
  }

  /* animate blowing up the alien: scatter its compoents, spinning them, and then delete them */
  fn explode(&mut self)
  {
    let rotate = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.05);
    
    for pixel in self.pixels.iter_mut()
    {
      pixel.node.as_mut().unwrap().append_translation(&Translation3::new(pixel.explode_x, pixel.explode_y, pixel.explode_z));
      pixel.node.as_mut().unwrap().prepend_to_local_rotation(&rotate);
    }

    /* delete model, pixel by pixel, after a few seconds and mark alien as dead */
    if self.time_of_death.unwrap().elapsed().as_secs() > 3
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


fn main() {
  let mut window = Window::new("Rust invaders");
  window.set_framerate_limit(Some(60));
  window.set_background_color(0.2, 0.2, 0.8);
  window.set_light(Light::StickToCamera);

  /* set up the camera */
  let eye = Point3::new(0.0, 0.0, -120.0);
  let at = Point3::origin();
  let mut camera = ArcBall::new(eye, at);

  /* metadata */
  let start_time = Instant::now();
  let mut rng = rand::thread_rng();

  /* array of baddies to track */ 
  let mut baddies = Vec::<Alien>::with_capacity(55);

  /* create and spawn baddies. each alien is 11 x 8 pixel cubes, so space them out
   * accordingly - no pun intended. */
  for y in -2..3
  {
    for x in -6..5
    {
      let mut baddie = Alien::new(&mut window);
      baddie.spawn(x as f32 * 13.0, y as f32 * 10.0, 0.0);
      baddies.push(baddie);
    }
  }

  /* keep track of rotating the aliens back and forth slightly */
  let mut rotate_dir = 1.0;
  let mut rotate_pos = 0.0;

  while window.render_with_camera(&mut camera)
  {
    let mut animate_baddies = false;

    /* oscillate the aliens and flip between their two animation states */
    rotate_pos = rotate_pos + 0.004 * rotate_dir;
    if rotate_pos > 0.1
    {
      rotate_dir = -1.0;
      animate_baddies = true;
    }
    if rotate_pos < -0.1
    {
      rotate_dir = 1.0;
      animate_baddies = true;
    }

    /* update the alien positions */
    for baddie in baddies.iter_mut()
    {
      match baddie.state
      {
        State::Alive =>
        {
          baddie.spin(rotate_dir);
          if animate_baddies == true
          {
            baddie.animate();
          }
          
          /* self-destruct after 5 seconds to test animation */
          if start_time.elapsed().as_secs() > 5
          {
            baddie.die(&mut rng);
          }
        },

        State::Dying =>
        {
          baddie.explode();
        },

        State::Dead => continue
      };
    }
  }
}


