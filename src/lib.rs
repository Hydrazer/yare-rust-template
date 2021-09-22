mod wrappers;

use crate::wrappers::*;

#[no_mangle]
#[allow(dead_code)] // You probably want to remove this
pub extern "C" fn tick(_tick: u32) {
  //   let spirits: Vec<Spirit> = get_spirits();
  //   let my_spirits: Vec<Spirit> = get_my_spirits();
  // let alive_spirits: Vec<Spirit> = get_alive_spirits();
  let my_spirit_vec: Vec<Spirit> = get_my_alive_spirits();
  //   let bases: Vec<Base> = get_bases();
  let stars: Vec<Star> = get_stars();

  for i in &my_spirit_vec {
    spirit.goto([100, 100]);
  }


  //   let outposts: Vec<Outpost> = get_outposts();
  //   let player_id: usize = get_player_id();

  // let pos = &stars[0].pos;
  // for spirit in &my_alive_spirits {
  //   spirit.goto(pos);
  //   graphics::line(
  //     spirit.pos,
  //     *pos,
  //     graphics::Colour {
  //       red: 255,
  //       green: 0,
  //       blue: 255,
  //       alpha: 1.0,
  //     },
  //   )
  // }

  // this is a mess, but it works lol (yes ima change it)
  // log(&*format!(
  //   "=== STATS ===\n
  //       FRIENDLY SPIRITS: {}\n
  //       OTHER SPIRITS: {}\n
  //       ",
  //   my_alive_spirits.len(),
  //   alive_spirits.len() - my_alive_spirits.len()
  // ));
}
