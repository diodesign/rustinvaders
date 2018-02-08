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
    let mut player_move_left = false;
    let mut player_move_right = false;

    /* create an array of baddies to track */ 
    let mut baddies = aliens::spawn_playfield(&mut window);

    /* create player */
    let mut player = hero::Hero::new();
    player.spawn(&mut window, player_x_pos);

    while window.render_with_camera(&mut camera)
    {
      aliens::animate_playfield(&mut baddies);
      player.animate();

      /* do collision detection */
      match player.get_bullet_coords()
      {
        Some((x, y, z)) =>
        {
          match aliens::detect_bullet_collision(&mut baddies, x, y, z)
          {
            Some(aliens::Collision::OutOfBounds) => player.destroy_bullet(),
            _ => {}
          }
        }
        None => {} /* no bullet, nothing to detect */
      };
      
      /* do player collision */

      /* check events for things like keypresses */
      for mut event in window.events().iter()
      {
        match event.value
        {
          /* handle a keypress */
          WindowEvent::Key(code, _, action, _) =>
          {
            match (code, action)
            {
              (glfw::Key::Z, Action::Press)   => player_move_left  = true,
              (glfw::Key::Z, Action::Release) => player_move_left  = false,
              (glfw::Key::X, Action::Press)   => player_move_right = true,
              (glfw::Key::X, Action::Release) => player_move_right = false,
              (glfw::Key::Enter, Action::Press) => player.fire(&mut window),
              (_, _) => {}
            }

            /* stop other keypresses going through to the default handler */
            event.inhibited = true;
          },

          _ => {} /* pass on other events to the default handlers */
        }
      }

      /* process results of events */
      match (player_move_left, player_move_right)
      {
        (true, false) => player.move_left(),
        (false, true) => player.move_right(),
        _ => {}
      }
    }
  }
}

