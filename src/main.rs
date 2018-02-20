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

use glfw::{ Action, WindowEvent };
use na::Point3;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::ArcBall;

mod aliens;
mod hero;
mod bullet;
mod collision;

const MAX_SCORE: i32 = 9999999; /* seems like a cool number */
const MAX_LIVES: i32 = 9; /* also a cool number */

/* collect up the objects in the playfield */
struct Playfield
{
  aliens: aliens::Aliens,         /* squadron of enemy aliens to shoot down */
  player: hero::Hero,             /* our player hero */
}

/* maintain state from level to level */
struct Game
{
  score: i32, /* player's current points score */
  lives: i32, /* player's current number of lives */
  player_x_pos: f32, /* player's ship x-position (y and z are fixed) */
}

enum LevelOutcome
{
  Victory, /* player beat the level */
  Died /* player ran out of lives */
}

fn main()
{
  let mut window = Window::new("Rust Invaders");
  window.set_framerate_limit(Some(60));
  window.set_light(Light::StickToCamera);

  /* notes: each of config_game, play_game, and game_over must delete all
   * scene objects before exiting. each function must track its own objects,
   * there is no automatic clean-up */

  loop
  {
    /* render the opening screen + menu */
    config_game(&mut window);
    
    /* setup and play the game */
    play_game(&mut window);

    /* render game over screen */
    game_over(&mut window);
  }
}

/* show a menu or at least give the player a chance to start */
fn config_game(mut window: &mut Window)
{
  /* TODO */
}

/* show the bad news */
fn game_over(mut window: &mut Window)
{
  /* TODO */
}

/* show end of level congratualtions */
fn congrats(mut window: &mut Window)
{
  /* TODO */
}

/* a game is a loop of levels until the player runs out of lives */
fn play_game(mut window: &mut Window)
{
  /* set up the camera and scene for the whole game */
  window.set_background_color(0.0, 0.0, 0.0); 
  let eye = Point3::new(0.0, 0.0, -250.0);
  let at = Point3::origin();
  let mut camera = ArcBall::new(eye, at);
  
  /* these variables carry across from level to level */
  let mut state = Game
  {
    score: 0, lives: 3, player_x_pos: 0.0,
  };

  /* play level after level until player dies */
  loop
  {
    match play_level(&mut window, &mut camera, &mut state)
    {
      LevelOutcome::Died => break, /* exit to game over screen */
      LevelOutcome::Victory => congrats(&mut window)
    }
  }
}

/* play a level of the game
 * => window = graphics context
 *    camera = viewing camera context
 *    state = game state variables
 * <= LevelOutcome::PlayerDead if hero ran out of lives
 */
fn play_level(mut window: &mut Window, camera: &mut ArcBall, state: &mut Game) -> LevelOutcome
{
  /* create the baddies and hero for this level */
  let mut playfield = Playfield
  {
    aliens: aliens::Aliens::new(&mut window),
    player: hero::Hero::new(&mut window, state.player_x_pos),
  };

  let mut player_move_left = false;
  let mut player_move_right = false;
  let mut player_fire = false;

  /* rendering loop */
  while window.render_with_camera(camera)
  {
    /* update aliens, player and any of their bullets / bombs in play */
    playfield.aliens.animate();
    playfield.player.animate();

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
            (glfw::Key::Enter, Action::Press) => player_fire = true,
            (glfw::Key::Enter, Action::Release) => player_fire = false,
            (_, _) => {}
          }

          /* stop other keypresses going through to the default handler */
          event.inhibited = true;
        },

        /* ignore mouse events */
        WindowEvent::MouseButton(_, _, _) => event.inhibited = true,
        WindowEvent::Scroll(_, _) => event.inhibited = true,

        _ => {} /* pass on other events to the default handlers */
      }
    }
    
    /* stop playing the level if the player is alive and the aliens are all dead, or if we're
     * out of lives. this check means we keep animating enemy and ship explosions when
     * the player has shot all the aliens or has run out of lives, rather than bailing out
     * immediately */
    if (playfield.player.state == hero::State::Alive && playfield.aliens.all_dead() == true) ||
       (playfield.player.state != hero::State::Dying && state.lives < 1)
    {
      break;
    }

    /* only update the player if it's still alive, otherwise all sorts
     * of inconsistencies will occur (ship hit by a bomb or alien while dying etc) */
    if playfield.player.state != hero::State::Alive
    {
      continue; /* skip movement, collision detection, etc while player is dead/dying */
    }

    /* process results of events: if a movement key is held down then
     * continue moving in that direction */
    match (player_move_left, player_move_right)
    {
      (true, false) => playfield.player.move_left(),
      (false, true) => playfield.player.move_right(),
      _ => {}
    }

    /* player can keep fire button held down, but we only allow one
     * hero bullet per playfield as per the original game */
    if player_fire == true
    {
      playfield.player.fire(&mut window); /* needs window to create its bullet */
    }

    playfield.aliens.fire(&mut window); /* aliens drop bombs as soon as they are able */

    /* did the player's bullet hit an alien? */
    if playfield.player.bullet.is_some() == true
    {
      let (x, y, _) = playfield.player.bullet.as_mut().unwrap().get_coords();
      if playfield.aliens.collision(x, y) == collision::CollisionOutcome::Hit
      {
        /* the call to collision() removes the alien if there is a hit, but 
         * we have to tell the ship's bullet to blow up too */
        playfield.player.destroy_bullet();
        state.score = state.score + aliens::ALIEN_POINTS;
        if state.score > MAX_SCORE
        {
          state.score = MAX_SCORE;
        }
      }

      /* remove bullet if it's gone out of bounds */
      if y > aliens::ALIEN_Y_CEILING
      {
        playfield.player.destroy_bullet();
      }
    }
    
    /* did an alien bomb hit the player? */
    if playfield.aliens.bomb.is_some() == true
    {
      let (x, y, _) = playfield.aliens.bomb.as_mut().unwrap().get_coords();
      if playfield.player.collision(x, y) == collision::CollisionOutcome::Hit
      {
        /* tell aliens to blow up their bomb, and the player its ship, if there is a hit */
        playfield.aliens.destroy_bomb();
        playfield.player.destroy(&mut window); /* window needed to add explosion debris to game world */
        state.lives = state.lives - 1;
      }

      /* remove the bomb if it goes out of bounds */
      if y < hero::HERO_Y_FLOOR
      {
        playfield.aliens.destroy_bomb();
      }
    }

    /* get the player's x, y coords */
    let (player_x_pos, player_y_pos, _) = playfield.player.get_coords();

    /* did an alien fly into the player? */
    if playfield.aliens.collision(player_x_pos, player_y_pos) == collision::CollisionOutcome::Hit
    {
      playfield.player.destroy(&mut window); /* window needed to add explosion debris to game world */
      state.lives = state.lives - 1
    }

    /* did the aliens manage to get below the player? if so, that's an instant 
     * game over, I'm afraid */
    if playfield.aliens.lowest_y() <= player_y_pos
    {
      playfield.player.destroy(&mut window); /* window needed to add explosion debris to game world */
      state.lives = 0;
    }
  }

  /* we've exited the level loop. remove all objects from the playfield */
  playfield.aliens.delete();
  playfield.player.delete();

  /* if we're still alive then we beat the level, otherwise we died */
  if state.lives > 0
  {
    return LevelOutcome::Victory
  }

  return LevelOutcome::Died
}

