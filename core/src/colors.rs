use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use strum::EnumIter;

/// An enum to represent the colors.
#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumIter,
    TryFromPrimitive,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    /// Black color, Coal.
    Black = 0,
    /// Blue color, Sapphire.
    Blue,
    /// Green color, Emerald.
    Green,
    /// Red color, Ruby.
    Red,
    /// White color, Diamond.
    White,
    /// Yellow color, Gold.
    Yellow,
}

impl Color {
    /// Get the emoji representation of the color.
    #[inline(always)]
    pub fn emoji(&self) -> &'static str {
        match self {
            Color::Black => "⚫",
            Color::Blue => "🔵",
            Color::Green => "🟢",
            Color::Red => "🔴",
            Color::White => "⚪",
            Color::Yellow => "🟡",
        }
    }
}

/// A struct to represent the color combinations.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ColorVec([u8; 6]);

impl ColorVec {
    /// Create a new empty color vector.
    #[inline(always)]
    pub const fn empty() -> Self {
        ColorVec([0; 6])
    }

    /// Create a new color vector with the given values.
    #[inline(always)]
    pub const fn new(black: u8, blue: u8, green: u8, red: u8, white: u8, yellow: u8) -> Self {
        ColorVec([black, blue, green, red, white, yellow])
    }

    /// Get the value of a color.
    #[inline(always)]
    pub fn get(&self, color: Color) -> u8 {
        self.0[color as usize]
    }

    /// Set the value of a color.
    #[inline(always)]
    pub fn set(&mut self, color: Color, value: u8) {
        self.0[color as usize] = value;
    }

    /// Add a value to a color.
    #[inline(always)]
    pub fn add(&mut self, color: Color, value: u8) {
        self.0[color as usize] += value;
    }

    /// Sub a value to a color.
    #[inline(always)]
    pub fn sub(&mut self, color: Color, value: u8) {
        self.0[color as usize] -= value;
    }

    /// Get an iterator over the colors.
    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        self.0.iter().copied()
    }

    /// Get the total number of tokens.
    #[inline(always)]
    pub fn total(&self) -> u8 {
        self.0.iter().sum()
    }

    /// Saturation subtraction.
    #[inline(always)]
    pub fn saturating_sub(&self, rhs: &Self) -> Self {
        let mut res = ColorVec::empty();
        for (color, (a, b)) in self.0.iter().zip(rhs.0.iter()).enumerate() {
            res.set(Color::try_from(color).unwrap(), a.saturating_sub(*b));
        }
        res
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

impl TryFrom<usize> for Color {
    type Error = TryFromPrimitiveError<Color>;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Color::try_from(value as u8)
    }
}
