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
mod hero;

fn main()
{
  let mut window = Window::new("Rust invaders");
  window.set_framerate_limit(Some(60));
  window.set_background_color(0.0, 0.0, 0.0); 
  window.set_light(Light::StickToCamera);

  /* set up the camera */
  let eye = Point3::new(0.0, 0.0, -250.0);
  let at = Point3::origin();
  let mut camera = ArcBall::new(eye, at);

  /* main gameplay loop */
  loop
  {
    let mut player_x_pos = 0.0;

    /* create an array of baddies to track */ 
    let mut baddies = aliens::spawn_playfield(&mut window);

    /* create player */
    let mut player = hero::Hero::new();
    player.spawn(&mut window, player_x_pos);

    while window.render_with_camera(&mut camera)
    {
      aliens::animate_playfield(&mut baddies);
    
      for mut event in window.events().iter()
      {
        match event.value
        {
          /* handle this keypress */
          WindowEvent::Key(code, _, Action::Press, _) | WindowEvent::Key(code, _, Action::Repeat, _) =>
          {
            match code
            {
              glfw::Key::Z => player.move_left(),
              glfw::Key::X => player.move_right(),
              _ => {}
            }
            event.inhibited = true;
          },

          _ => {}
        }
      }
    }
  }
}

