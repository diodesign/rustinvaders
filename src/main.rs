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

use na::Translation3;
use kiss3d::window::Window;
use kiss3d::light::Light;

struct Pixel
{
  width: f32, height: f32, depth: f32,
  x: f32, y: f32, z: f32,
  r: f32, g: f32, b: f32
}

struct Alien
{
  pixels: Vec<Pixel>
}

impl Alien
{
  fn new() -> Alien
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
            ** **         */

      pixels: vec!
      [
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x: -3.0, y:  4.0, z: 0.0, r: 0.2, g: 1.0, b: 0.2 },
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x:  3.0, y:  4.0, z: 0.0, r: 0.2, g: 1.0, b: 0.2 },
        
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x: -2.0, y:  3.0, z: 0.0, r: 0.2, g: 1.0, b: 0.2 },
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x:  2.0, y:  3.0, z: 0.0, r: 0.2, g: 1.0, b: 0.2 },

        Pixel { width:  7.0, height: 1.0, depth: 1.0, x:  0.0, y:  2.0, z: 0.0, r: 0.2, g: 1.0, b: 0.2 },

        Pixel { width:  2.0, height: 1.0, depth: 1.0, x: -4.0, y:  1.0, z: 0.0, r: 0.2, g: 1.0, b: 0.2 },
        Pixel { width:  3.0, height: 1.0, depth: 1.0, x:  0.0, y:  1.0, z: 0.0, r: 0.2, g: 1.0, b: 0.2 },
        Pixel { width:  2.0, height: 1.0, depth: 1.0, x:  4.0, y:  1.0, z: 0.0, r: 0.2, g: 1.0, b: 0.2 },

        Pixel { width: 11.0, height: 1.0, depth: 1.0, x:  0.0, y:  0.0, z: 0.0, r: 0.2, g: 1.0, b: 0.2 },

        Pixel { width:  1.0, height: 1.0, depth: 1.0, x:  5.0, y: -1.0, z: 0.0, r: 0.2, g: 1.0, b: 0.2 },
        Pixel { width:  7.0, height: 1.0, depth: 1.0, x:  0.0, y: -1.0, z: 0.0, r: 0.2, g: 1.0, b: 0.2 },
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x: -5.0, y: -1.0, z: 0.0, r: 0.2, g: 1.0, b: 0.2 },

        Pixel { width:  1.0, height: 1.0, depth: 1.0, x: -5.0, y: -2.0, z: 0.0, r: 0.2, g: 1.0, b: 0.2 },
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x: -3.0, y: -2.0, z: 0.0, r: 0.2, g: 1.0, b: 0.2 },
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x:  3.0, y: -2.0, z: 0.0, r: 0.2, g: 1.0, b: 0.2 },
        Pixel { width:  1.0, height: 1.0, depth: 1.0, x:  5.0, y: -2.0, z: 0.0, r: 0.2, g: 1.0, b: 0.2 },

        Pixel { width:  2.0, height: 1.0, depth: 1.0, x: -1.5, y: -3.0, z: 0.0, r: 0.2, g: 1.0, b: 0.2 },
        Pixel { width:  2.0, height: 1.0, depth: 1.0, x:  1.5, y: -3.0, z: 0.0, r: 0.2, g: 1.0, b: 0.2 }
      ]
    }
  }

  fn spawn(&self, window: &mut Window)
  {
    /* spin through the array of pixels to create this monster */
    for pixel in self.pixels.iter()
    {
      /* create a cube pixel */
      let mut p = window.add_cube(pixel.width, pixel.height, pixel.depth);

      /* move it into position */
      p.append_translation(&Translation3::new(pixel.x, pixel.y, pixel.z));

      /* and color it */
      p.set_color(pixel.r, pixel.g, pixel.b);
    }
  }
}

fn main() {
    let mut window = Window::new("Rust invaders");

    /* create our first baddie! */
    let baddie = Alien::new();
    baddie.spawn(&mut window);

    window.set_background_color(0.0, 0.0, 0.0);
    window.set_light(Light::StickToCamera);

    while !window.should_close() {
        window.render();
    }
}
