/* Space invaders in Rust
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

use glfw::{Action, WindowEvent};
use na::Point3;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::ArcBall;

mod aliens;

fn main() {
  let mut window = Window::new("Rust invaders");
  window.set_framerate_limit(Some(60));
  window.set_background_color(0.0, 0.0, 0.0); 
  window.set_light(Light::StickToCamera);

  /* set up the camera */
  let eye = Point3::new(0.0, 0.0, -200.0);
  let at = Point3::origin();
  let mut camera = ArcBall::new(eye, at);

  /* array of baddies to track */ 
  let mut baddies = Vec::<aliens::Alien>::with_capacity(55);

  /* create and spawn baddies. each alien is 11 x 8 pixel cubes, so space them out
   * accordingly - no pun intended. */
  for y in -2..3
  {
    for x in -6..5
    {
      let mut baddie = aliens::Alien::new(&mut window);
      baddie.spawn(x as f32 * 13.0, y as f32 * 10.0, 0.0);
      baddies.push(baddie);
    }
  }

  while window.render_with_camera(&mut camera)
  {
    /* update the alien positions */
    for baddie in baddies.iter_mut()
    {
      baddie.animate();
    }

    /* process pending events */
    for event in window.events().iter()
    {
      match event.value
      {
        /* press a mouse key to kill them all */
        WindowEvent::MouseButton(_, Action::Press, _) =>
        {
          for baddie in baddies.iter_mut()
          {
            baddie.die();
          }
        },
        _ => { }
      }
    }
  }
}


