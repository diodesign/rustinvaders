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

  /* create an array of baddies to track */ 
  let mut baddies = aliens::spawn_playfield(&mut window);

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


