/* Space invaders in Rust
 *
 * Bullet time
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

pub struct Bullet
{
  x: f32, y: f32, z: f32,
  bullet: SceneNode,
  speed: f32
}

impl Bullet
{
  /* create a new bullet
   * => window  = graphics context in which to create the bullet 
   *    x, y    = coords on where to start the bullet (z = 0.0)
   *    rad     = radius of the bullet's sphere
   *    r, g, b = color of the bullet,
   *    speed   = rate at which bullet will move in y direction
   */
  pub fn new(window: &mut Window, x: f32, y: f32, rad: f32, r: f32, g: f32, b: f32, speed: f32) -> Bullet
  {
    let mut shell = Bullet
    {
      x: x, y: y, z: 0.0,
      bullet: window.add_sphere(rad),
      speed: speed
    };

    shell.bullet.append_translation(&Translation3::new(shell.x, shell.y, shell.z));
    shell.bullet.set_color(r, g, b); 

    return shell;
  }

  /* remove the bullet's sphere from the screen */
  pub fn destroy(&mut self)
  {
    self.bullet.unlink();
  }

  /* if a bullet is in play then move it. if the ship is blowing up, then blow it up some more */
  pub fn animate(&mut self)
  {
    self.y = self.y + self.speed;
    self.bullet.append_translation(&Translation3::new(0.0, self.speed, 0.0));
  }

  /* returns Some(x, y, z) coords of active bullet, or None if no bullet */ 
  pub fn get_coords(&self) -> (f32, f32, f32)
  {
    return (self.x, self.y, self.z);
  }
}

