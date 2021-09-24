mod wrappers;

use crate::wrappers::*;

#[no_mangle]
pub extern "C" fn tick(_tick: u32) {
  // let spirits: Vec<Spirit> = get_spirits();
  // let my_spirits: Vec<Spirit> = get_my_spirits();
  // let alive_spirits: Vec<Spirit> = get_alive_spirits();
  let my_alive_spirits: Vec<Spirit> = get_my_alive_spirits();
  // let bases: Vec<Base> = get_bases();
  let stars: Vec<Star> = get_stars();
  // let outposts: Vec<Outpost> = get_outposts();
  // let player_id: usize = get_player_id();

  let pos = &stars[0].pos;
  for spirit in &my_alive_spirits {
    spirit.goto(pos);
    graphics::line(
      spirit.pos,
      *pos,
      graphics::Colour {
        red: 255,
        green: 0,
        blue: 255,
        alpha: 1.0,
      },
    )
  }
}
