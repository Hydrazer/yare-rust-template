mod wrappers;

use crate::wrappers::*;
use yare;

#[no_mangle]
pub extern "C" fn tick(_tick: u32) {
    let spirits: Vec<Spirit> = get_spirits();
    let my_spirits: Vec<Spirit> = get_my_spirits();
    let alive_spirits: Vec<Spirit> = get_alive_spirits();
    let my_alive_spirits: Vec<Spirit> = get_my_alive_spirits();
    let bases: Vec<Base> = get_bases();
    let stars: Vec<Star> = get_stars();
    let outposts: Vec<Outpost> = get_outposts();
    let player_id: usize = get_player_id();

    let pos = &stars[0].pos;
    for index in 0..my_spirits.len() {
        let spirit: &Spirit = &my_spirits[index];
        if spirit.friendly {
            spirit.goto(pos);
            graphics::line(
                spirit.pos,
                *pos,
                graphics::Colour {
                    red: 255,
                    green: 0,
                    blue: 255,
                    alpha: 1,
                },
            )
        }
    }

    // this is a mess, but it works lol (yes ima change it)
    log(&*(String::from("There is ") + &*my_spirits.len().to_string() + " friendly spirits found."))
}
