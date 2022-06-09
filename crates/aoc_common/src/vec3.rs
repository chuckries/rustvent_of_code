use num_traits::PrimInt;

pub type Vec3us = Vec3<usize>;
pub type Vec3i32 = Vec3<i32>;
pub type Vec3i64 = Vec3<i64>;
pub type Vec3u32 = Vec3<u32>;
pub type Vec3u64 = Vec3<u64>;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Default, Debug)]
pub struct Vec3<T: PrimInt> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: PrimInt> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z}
    }

    pub const fn x(&self) -> T {
        self.x
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.x
    }

    pub const fn y(&self) -> T {
        self.y
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.y
    }

    pub const fn z(&self) -> T {
        self.z
    }

    pub fn z_mut(&mut self) -> &mut T {
        &mut self.z
    }

    pub fn zero() -> Self {
        Self { x: T::zero(), y: T::zero(), z: T::zero() }
    }

    pub fn unit_x() -> Self {
        Self { x: T::one(), y: T::zero(), z: T::zero() }
    }

    pub fn unit_y() -> Self {
        Self { x: T::zero(), y: T::one(), z: T::zero() }
    }

    pub fn unit_z() -> Self {
        Self { x: T::zero(), y: T::zero(), z: T::one() }
    }
}

impl <T: PrimInt + std::fmt::Display> std::fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T: PrimInt + std::ops::Neg<Output = T>> std::ops::Neg for Vec3<T> {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<T: PrimInt> std::ops::Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self. z + rhs.z)
    }
}

impl<T: PrimInt> std::ops::AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}