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
mod bullet;

struct Playfield
{
  aliens: Vec<aliens::Alien>,     /* array of aliens to shoot down */
  bomb: Option<bullet::Bullet>,   /* aliens drop bombs on the player */
  player: hero::Hero,             /* our player hero */
  bullet: Option<bullet::Bullet>  /* player fires one bullet at a time */
}

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

  let mut score = 0;
  let mut lives = 3;
  let mut player_x_pos = 0.0;

  /* main gameplay loop */
  loop
  {
    /* game over if we're out of lives */
    if lives < 1
    {
      println!("GAME OVER!");
      break;
    }

    let mut player_move_left = false;
    let mut player_move_right = false;

    /* group up the objects into a playfield struct */
    let mut playfield = Playfield
    {
      aliens: aliens::spawn_playfield(&mut window),
      bomb: None,
      player: hero::Hero::new(&mut window, player_x_pos),
      bullet: None
    };
    
    while window.render_with_camera(&mut camera)
    {
      /* update aliens, player and any bullets / bombs in play */
      aliens::animate_playfield(&mut playfield.aliens);
      playfield.player.animate();

      if playfield.bomb.is_some() == true
      {
        playfield.bomb.as_mut().unwrap().animate();
      }
      
      if playfield.bullet.is_some() == true
      {
        playfield.bullet.as_mut().unwrap().animate();
      }

      /* get the player's x, y coords */
      let (player_x_pos, player_y_pos, _) = playfield.player.get_coords();

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
              (glfw::Key::Enter, Action::Press) =>
              {
                if playfield.bullet.is_none()
                {
                  playfield.bullet
                    = Some(bullet::Bullet::new(&mut window, player_x_pos, hero::BULLET_Y_START,
                                               hero::BULLET_RADIUS, hero::BULLET_COLOR_R, hero::BULLET_COLOR_G,
                                               hero::BULLET_COLOR_B, hero::BULLET_ASCENT));
                }
              },
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
        (true, false) => playfield.player.move_left(),
        (false, true) => playfield.player.move_right(),
        _ => {}
      }

      /* did the player's bullet hit an alien? */
      if playfield.bullet.is_some() == true
      {
        let (x, y, _) = playfield.bullet.as_mut().unwrap().get_coords();
        match aliens::detect_bullet_collision(&mut playfield.aliens, x, y)
        {
          Some(aliens::Collision::OutOfBounds) =>
          {
            playfield.bullet.as_mut().unwrap().destroy();
            playfield.bullet = None;
          },
          Some(aliens::Collision::HitAlien) =>
          {
            /* detect_bullet_collision() takes care of tidying up the alien */
            score = score + aliens::ALIEN_POINTS;
            playfield.bullet.as_mut().unwrap().destroy();
            playfield.bullet = None;
          },
          _ => {}
        }
      }
      
      /* did an alien fly into the player? */
      if aliens::detect_ship_collision(&mut playfield.aliens, player_x_pos, player_y_pos) == true
      {
        playfield.player.destroy(hero::Destruction::Explode);
        lives = lives - 1;
        break;
      }

      /* did the player beat the level? */
      if aliens::all_dead(&mut playfield.aliens) == true
      {
        println!("well done! level complete");
        playfield.player.destroy(hero::Destruction::NoExplode);
        break;
      }
    }
  }
}


