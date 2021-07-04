use std::ffi::CString;
use std::{
  cmp::Ordering,
  iter::Sum,
  ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};
use yare;
use yare::*;

#[derive(Copy, Clone)]
pub struct Vec2 {
  pub x: f32,
  pub y: f32,
}

impl Vec2 {
  pub fn dot(self, other: Vec2) -> f32 {
    self.x * other.x + self.y * other.y
  }

  pub fn norm_squared(self) -> f32 {
    self.x * self.x + self.y * self.y
  }

  pub fn norm(self) -> f32 {
    self.norm_squared().sqrt()
  }

  pub fn normalize(self) -> Vec2 {
    self / self.norm()
  }

  pub fn midpoint(self, other: Vec2) -> Vec2 {
    (self + other) / 2.
  }

  pub fn lerp(self, other: Vec2, ratio: f32) -> Vec2 {
    self * ratio + other * (1. - ratio)
  }

  pub fn dist_squared(self, other: Vec2) -> f32 {
    (self - other).norm_squared()
  }

  pub fn in_range(self, other: Vec2, range: f32) -> bool {
    (self - other) < range
  }

  pub fn towards(self, other: Vec2, length: f32) -> Vec2 {
    self + (other - self).normalize() * length
  }
}

impl Default for Vec2 {
  fn default() -> Self {
    Vec2 { x: 0., y: 0. }
  }
}

impl Add for Vec2 {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl Sub for Vec2 {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

impl Mul<f32> for Vec2 {
  type Output = Self;

  fn mul(self, other: f32) -> Self {
    Self {
      x: self.x * other,
      y: self.y * other,
    }
  }
}

impl Div<f32> for Vec2 {
  type Output = Self;

  fn div(self, other: f32) -> Self {
    Self {
      x: self.x / other,
      y: self.y / other,
    }
  }
}

impl AddAssign for Vec2 {
  fn add_assign(&mut self, other: Self) {
    self.x += other.x;
    self.y += other.y;
  }
}

impl SubAssign for Vec2 {
  fn sub_assign(&mut self, other: Self) {
    self.x -= other.x;
    self.y -= other.y;
  }
}

impl MulAssign<f32> for Vec2 {
  fn mul_assign(&mut self, other: f32) {
    self.x *= other;
    self.y *= other;
  }
}

impl DivAssign<f32> for Vec2 {
  fn div_assign(&mut self, other: f32) {
    self.x /= other;
    self.y /= other;
  }
}

impl PartialEq<f32> for Vec2 {
  fn eq(&self, other: &f32) -> bool {
    self.norm_squared().eq(&(other * other))
  }
}

impl PartialOrd<f32> for Vec2 {
  fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
    self.norm_squared().partial_cmp(&(other * other))
  }
}

impl Sum for Vec2 {
  fn sum<I: Iterator<Item = Vec2>>(iter: I) -> Self {
    let mut res = Vec2::default();
    for vec in iter {
      res += vec;
    }
    res
  }
}

#[derive(Copy, Clone)]
pub enum Shape {
  Circle,
  Square,
}

#[derive(Copy, Clone)]
pub struct Spirit {
  pub index: usize,
  pub alive: bool,
  pub energy_cap: u32,
  pub energy: u32,
  pub friendly: bool,
  pub pos: Vec2,
  pub size: u32,
  pub shape: Shape,
}

impl Spirit {
  pub fn goto(&self, pos: &Vec2) {
    unsafe { spirit::goto(self.index, pos.x, pos.y) }
  }

  /// Using a generic allows passing both an index or a reference to a spirit.
  pub fn energize<I: Into<usize>>(&self, target: &Spirit) {
    unsafe { spirit::energize(self.index, target.index) }
  }

  pub fn energize_base(&self, target: &Base) {
    unsafe { spirit::energize_base(self.index, target.index) }
  }

  pub fn drain_star(&self) {
    unsafe { spirit::energize(self.index, self.index) }
  }

  pub fn shout(&self, string: &str) {
    let c_string = CString::new(string).unwrap();
    unsafe { spirit::shout(self.index, c_string.as_ptr()) }
  }
}

pub fn get_spirits() -> Vec<Spirit> {
  unsafe {
    let count = spirit::count();
    let mut new_spirits = Vec::with_capacity(count);
    for index in 0..count {
      let alive = spirit::hp(index) > 0;
      let energy_cap = spirit::energy_capacity(index);
      let energy = spirit::energy(index);
      let friendly = player::me() == spirit::player_id(index);
      // let Position { x, y } = spirit::position(index);
      let x = spirit::position_x(index);
      let y = spirit::position_y(index);
      let size = spirit::size(index);
      let shape = if spirit::shape(index) == 0 {
        Shape::Circle
      } else {
        Shape::Square
      };

      new_spirits.push(Spirit {
        index,
        alive,
        energy_cap,
        energy,
        friendly,
        pos: Vec2 { x, y },
        size,
        shape,
      });
    }
    new_spirits
  }
}

pub fn get_my_spirits() -> Vec<Spirit> {
  let spirits = get_spirits();
  let mut my_spirits: Vec<Spirit> = Vec::new();
  for index in 0..spirits.len() {
    let spirit: &Spirit = &spirits[index];
    if (spirit.friendly) {
      my_spirits.push(spirits[index])
    }
  }
  return my_spirits;
}

pub fn get_alive_spirits() -> Vec<Spirit> {
  let spirits = get_spirits();
  let mut alive_spirits: Vec<Spirit> = Vec::new();
  for index in 0..spirits.len() {
    let spirit: &Spirit = &spirits[index];
    if (spirit.alive) {
      alive_spirits.push(spirits[index])
    }
  }
  return alive_spirits;
}

pub fn get_my_alive_spirits() -> Vec<Spirit> {
  let spirits = get_spirits();
  let mut my_alive_spirits: Vec<Spirit> = Vec::new();
  for index in 0..spirits.len() {
    let spirit: &Spirit = &spirits[index];
    if (spirit.alive) {
      my_alive_spirits.push(spirits[index])
    }
  }
  return my_alive_spirits;
}

// pub const spirits: std::vec::Vec<Spirit> = get_spirits();

pub struct Base {
  pub index: usize,
  pub pos: Vec2,
  pub energy_cap: u32,
  pub energy: u32,
  pub current_spirit_cost: u32,
  pub alive: bool,
  pub player_id: usize,
}

pub fn get_bases() -> Vec<Base> {
  unsafe {
    let count = base::count();
    let mut new_bases = Vec::with_capacity(count);
    for index in 0..count {
      let x = base::position_x(index);
      let y = base::position_y(index);
      let energy_cap = base::energy_capacity(index);
      let energy = base::energy(index);
      let current_spirit_cost = base::current_spirit_cost(index);
      let alive = base::hp(index) == 1;
      let player_id = base::player_id(index);

      new_bases.push(Base {
        index,
        pos: Vec2 { x, y },
        energy_cap,
        energy,
        current_spirit_cost,
        alive,
        player_id,
      });
    }
    new_bases
  }
}

// pub const bases: std::vec::Vec<Base> = get_bases();

pub struct Star {
  pub index: usize,
  pub pos: Vec2,
  pub size: u32,
  pub energy: u32,
  pub range: u32,
}

pub fn get_stars() -> Vec<Star> {
  unsafe {
    let count = star::count();
    let mut new_stars = Vec::with_capacity(count);
    for index in 0..count {
      let x = star::position_x(index);
      let y = star::position_y(index);
      let size = 220; // 80 for smaller stars, Will needs to fix
      let energy = 0; // Will needs to fix
      let range = 200;

      new_stars.push(Star {
        index,
        pos: Vec2 { x, y },
        size,
        energy,
        range,
      });
    }
    new_stars
  }
}

// pub const stars: std::vec::Vec<Star> = get_stars();

pub struct Outpost {
  pub index: usize,
  pub pos: Vec2,
  pub energy_cap: u32,
  pub energy: u32,
  pub range: f32,
  pub player_id: usize,
}

pub fn get_outposts() -> Vec<Outpost> {
  unsafe {
    let count = outpost::count();
    let mut new_outposts = Vec::with_capacity(count);
    for index in 0..count {
      let x = outpost::position_x(index);
      let y = outpost::position_y(index);
      let energy = outpost::energy(index);
      let energy_cap = outpost::energy_capacity(index);
      let range = outpost::range(index);
      let player_id = outpost::player_id(index);

      new_outposts.push(Outpost {
        index,
        pos: Vec2 { x, y },
        energy,
        energy_cap,
        range,
        player_id,
      });
    }
    new_outposts
  }
}

pub mod graphics {
  use crate::wrappers::Vec2;

  #[derive(Copy, Clone)]
  pub struct Colour {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
  }
  fn setcolour(colour: Colour) {
    unsafe {
      yare::graphics::colour(colour.red, colour.green, colour.blue, colour.alpha);
    }
  }
  pub fn circle(pos: Vec2, radius: f32, colour: Colour) {
    setcolour(colour);
    unsafe {
      yare::graphics::circle(pos.x, pos.y, radius);
    }
  }
  pub fn line(pos1: Vec2, pos2: Vec2, colour: Colour) {
    setcolour(colour);
    unsafe {
      yare::graphics::line(pos1.x, pos1.y, pos2.x, pos2.y);
    }
  }
}

pub fn get_player_id() -> usize {
  return unsafe { player::me() };
}

pub fn log(s: &str) {
  let c_string = CString::new(s).unwrap();
  unsafe { console::log(c_string.as_ptr()) }
}
