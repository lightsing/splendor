use abi_stable::StableAbi;
use num_enum::TryFromPrimitive;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use strum::EnumIter;

/// An enum to represent the colors.
#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIter, TryFromPrimitive, StableAbi,
)]
pub enum Color {
    Black = 0,
    Blue,
    Green,
    Red,
    White,
    Yellow,
}

/// A struct to represent the color combinations.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, StableAbi)]
pub struct ColorVec([u8; 6]);

impl ColorVec {
    #[inline(always)]
    pub const fn empty() -> Self {
        ColorVec([0; 6])
    }

    #[inline(always)]
    pub const fn new(black: u8, blue: u8, green: u8, red: u8, white: u8, yellow: u8) -> Self {
        ColorVec([black, blue, green, red, white, yellow])
    }

    #[inline(always)]
    pub fn get(&self, color: Color) -> u8 {
        self.0[color as usize]
    }

    #[inline(always)]
    pub fn set(&mut self, color: Color, value: u8) {
        self.0[color as usize] = value;
    }

    #[inline(always)]
    pub fn add(&mut self, color: Color, value: u8) {
        self.0[color as usize] += value;
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        self.0.iter().copied()
    }

    #[inline(always)]
    pub fn total(&self) -> u8 {
        self.0.iter().sum()
    }
}

impl PartialOrd for ColorVec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ColorVec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(std::cmp::Ordering::Equal, |acc, (a, b)| acc.then(a.cmp(b)))
    }
}

impl Add for ColorVec {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl Add<&ColorVec> for ColorVec {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self {
        self += rhs;
        self
    }
}

impl AddAssign<&ColorVec> for ColorVec {
    fn add_assign(&mut self, rhs: &Self) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(a, b)| *a += b);
    }
}

impl AddAssign for ColorVec {
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

impl Sub for ColorVec {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

impl Sub<&ColorVec> for ColorVec {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self {
        self -= rhs;
        self
    }
}

impl SubAssign for ColorVec {
    fn sub_assign(&mut self, rhs: Self) {
        *self -= &rhs;
    }
}

impl SubAssign<&ColorVec> for ColorVec {
    fn sub_assign(&mut self, rhs: &Self) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(a, b)| *a -= b);
    }
}
