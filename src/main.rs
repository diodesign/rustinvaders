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

use na::{Vector3, Translation3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::post_processing::SobelEdgeHighlight;

enum Frame
{
  Normal,
  Alternate
}

struct Pixel
{
  width: f32, height: f32, depth: f32,
  x:  f32, y:  f32, z:  f32, /* normal x,y,z frame positions */
  tx: f32, ty: f32, tz: f32, /* alternate x,y,z frame translations */
  r:  f32, g:  f32, b:  f32,
  node: Option<SceneNode>    /* this pixel's scene node */
}

struct Alien
{
  pixels: Vec<Pixel>,
  model: SceneNode,
  animation_frame: Frame
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
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x: -3.0, y:  4.0, z: 0.0, tx:  1.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None },
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x:  3.0, y:  4.0, z: 0.0, tx: -1.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None },
        
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x: -2.0, y:  3.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None },
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x:  2.0, y:  3.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None },

        Pixel { width:  7.0, height: 1.0, depth: 1.0, x:  0.0, y:  2.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None },

        Pixel { width:  2.0, height: 1.0, depth: 1.0, x: -4.0, y:  1.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None },
        Pixel { width:  3.0, height: 1.0, depth: 1.0, x:  0.0, y:  1.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None },
        Pixel { width:  2.0, height: 1.0, depth: 1.0, x:  4.0, y:  1.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None },

        Pixel { width: 11.0, height: 1.0, depth: 1.0, x:  0.0, y:  0.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None },

        Pixel { width:  1.0, height: 1.0, depth: 1.0, x:  5.0, y: -1.0, z: 0.0, tx:  0.0, ty: 3.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None },
        Pixel { width:  7.0, height: 1.0, depth: 1.0, x:  0.0, y: -1.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None },
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x: -5.0, y: -1.0, z: 0.0, tx:  0.0, ty: 3.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None },

        Pixel { width:  1.0, height: 1.0, depth: 1.0, x: -5.0, y: -2.0, z: 0.0, tx:  0.0, ty: 3.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None },
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x: -3.0, y: -2.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None },
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x:  3.0, y: -2.0, z: 0.0, tx:  0.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None },
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x:  5.0, y: -2.0, z: 0.0, tx:  0.0, ty: 3.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None },

        Pixel { width:  2.0, height: 1.0, depth: 1.0, x: -1.5, y: -3.0, z: 0.0, tx: -2.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None },
        Pixel { width:  2.0, height: 1.0, depth: 1.0, x:  1.5, y: -3.0, z: 0.0, tx:  2.0, ty: 0.0, tz: 0.0, r: 0.2, g: 1.0, b: 0.2, node: None }
      ],

      /* attach all the pixels together as a group */
      model: window.add_group(),

      /* start off in normal animation frame */
      animation_frame: Frame::Normal
    }
  }

  fn spawn(&mut self)
  {
    /* spin through the array of pixels to create this monster */
    for pixel in self.pixels.iter_mut()
    {
      /* create a cube pixel aka a scene node */
      let mut p = self.model.add_cube(pixel.width, pixel.height, pixel.depth);

      /* move it into position */
      p.append_translation(&Translation3::new(pixel.x, pixel.y, pixel.z));

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

}

fn main() {
  let mut window = Window::new("Rust invaders");
  // window.set_framerate_limit(Some(60));
  window.set_background_color(0.6, 0.6, 0.9);
  window.set_light(Light::StickToCamera);

  /* create our first baddie! */
  let mut baddie = Alien::new(&mut window);
  baddie.spawn();

  let mut sobel = SobelEdgeHighlight::new(2.0);
  let mut rotate_dir = 1.0;
  let mut rotate_pos = 0.0;

  while window.render_with_effect(&mut sobel)
  {
    /* oscillate the alien and flip its animation frames */
    rotate_pos = rotate_pos + 0.004 * rotate_dir;
    let rotate = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.016 * rotate_dir);
    baddie.model.prepend_to_local_rotation(&rotate);
    if rotate_pos > 0.1
    {
      rotate_dir = -1.0;
      baddie.animate();
    }

    if rotate_pos < -0.1
    {
      rotate_dir = 1.0;
      baddie.animate();
    }
  }
}


