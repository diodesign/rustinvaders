/* Space invaders in Rust
 *
 * 2D collision checking
 *
 * Game concept by Tomohiro Nishikado / Taito
 * Rust code By Chris Williams <diodesign@gmail.com>
 *
 * Written for fun. See LICENSE.
 *
 */

const COLLISION_TOLERANCE: f32 = 5.0; /* objects closer than this are considered collided */

/* either objects hit, miss, or leave their y-bounds
 * (no need to check for x or z bounds in this game)
 */
#[derive(PartialEq)]
pub enum CollisionOutcome
{
  Hit,
  Miss
}

#[derive(Clone, Copy)]
pub struct CollisionObject
{
  pub x: f32, pub y: f32 /* x,y coords of object */
}

/* describe a collision scenario */
#[derive(Clone, Copy)]
pub struct Collision
{
  /* two objects to check */
  pub a: CollisionObject,
  pub b: CollisionObject,
}

pub fn check(scenario: Collision) -> CollisionOutcome
{
  if scenario.a.x < scenario.b.x + COLLISION_TOLERANCE &&
     scenario.a.x > scenario.b.x - COLLISION_TOLERANCE &&
     scenario.a.y < scenario.b.y + COLLISION_TOLERANCE &&
     scenario.a.y > scenario.b.y - COLLISION_TOLERANCE
  {
    return CollisionOutcome::Hit;
  }

  return CollisionOutcome::Miss;
}

