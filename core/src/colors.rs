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
///
/// ## Note
/// when comparing two ColorVecs a, b, the order is defined as:
/// - a < b iff \forall i, a[i] <= b[i] and \exists j, a[j] < b[j]
/// - a <= b iff \forall i, a[i] <= b[i]
/// - a > b iff \forall i, a[i] >= b[i] and \exists j, a[j] > b[j]
/// - a >= b iff \forall i, a[i] >= b[i]
/// - otherwise a and b are not comparable
///
/// Also the equality is trivially defined as the equality of the vectors.
///
/// Hence, the color vector cannot impl `PartialOrd` and `Ord`, following stmts are false:
///   - `a <= b` iff `a < b || a == b`
///   - `a >= b` iff `a > b || a == b`
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

    /// Check if the color vector is less than the other.
    ///
    /// The order is defined as: \forall i, a[i] <= b[i] and \exists j, a[j] < b[j]
    pub fn lt(&self, rhs: &Self) -> bool {
        self.0.iter().zip(rhs.0.iter()).all(|(a, b)| *a <= *b)
            && self.0.iter().zip(rhs.0.iter()).any(|(a, b)| *a < *b)
    }

    /// Check if the color vector is less than or equal to the other.
    ///
    /// The order is defined as: \forall i, a[i] <= b[i]
    pub fn le(&self, rhs: &Self) -> bool {
        self.0.iter().zip(rhs.0.iter()).all(|(a, b)| *a <= *b)
    }

    /// Check if the color vector is greater than the other.
    ///
    /// The order is defined as: \forall i, a[i] >= b[i] and \exists j, a[j] > b[j]
    pub fn gt(&self, rhs: &Self) -> bool {
        self.0.iter().zip(rhs.0.iter()).all(|(a, b)| *a >= *b)
            && self.0.iter().zip(rhs.0.iter()).any(|(a, b)| *a > *b)
    }

    /// Check if the color vector is greater than or equal to the other.
    pub fn ge(&self, rhs: &Self) -> bool {
        self.0.iter().zip(rhs.0.iter()).all(|(a, b)| *a >= *b)
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
