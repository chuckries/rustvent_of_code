use num_traits::PrimInt;

use crate::Vec3;

pub struct Aabb<T: PrimInt> {
    p0: Vec3<T>,
    p1: Vec3<T>,
}

impl<T: PrimInt> Aabb<T> {
    pub fn new(p0: Vec3<T>, p1: Vec3<T>) -> Self {
        Self {
            p0,
            p1
        }
    }

    pub fn p0(&self) -> Vec3<T> {
        self.p0
    }

    pub fn p1(&self) -> Vec3<T> {
        self.p1
    }

    pub fn bounding<I>(it: I) -> Self 
        where I: Iterator<Item = Vec3<T>>
    {
        let mut min = Vec3::new(T::max_value(), T::max_value(), T::max_value());
        let mut max = Vec3::new(T::min_value(), T::min_value(), T::min_value());
    
        for p in it {
            if p.x < min.x {
                min.x = p.x;
            }
            if p.x > max.x {
                max.x = p.x;
            }
            if p.y < min.y {
                min.y = p.y;
            }
            if p.y > max.y {
                max.y = p.y;
            }
            if p.z < min.z {
                min.z = p.z;
            }
            if p.z > max.z {
                max.z = p.z;
            }
        }

        Self::new(min, max)
    }

    pub fn contains(&self, p: Vec3<T>) -> bool {
        p.x >= self.p0.x && p.x <= self.p1.x &&
        p.y >= self.p0.y && p.y <= self.p1.y &&
        p.z >= self.p0.z && p.z <= self.p1.z
    }
}