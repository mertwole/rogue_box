use std::ops;

pub struct Math {}

impl Math {
    pub fn small_enought(x: f32) -> bool {
        x < std::f32::EPSILON * 10.0 && x > -std::f32::EPSILON * 10.0
    }

    pub fn min<T>(a: T, b: T) -> T
    where
        T: PartialOrd,
    {
        if a < b {
            a
        } else {
            b
        }
    }

    pub fn max<T>(a: T, b: T) -> T
    where
        T: PartialOrd,
    {
        if a > b {
            a
        } else {
            b
        }
    }
}

// region IVec2
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct IVec2 {
    pub x: isize,
    pub y: isize,
}

impl std::fmt::Display for IVec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl IVec2 {
    pub fn new(x: isize, y: isize) -> IVec2 {
        IVec2 { x, y }
    }

    pub fn zero() -> IVec2 {
        IVec2 { x: 0, y: 0 }
    }

    pub fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

impl ops::Add<IVec2> for IVec2 {
    type Output = IVec2;
    fn add(self, rhs: IVec2) -> IVec2 {
        IVec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub<IVec2> for IVec2 {
    type Output = IVec2;
    fn sub(self, rhs: IVec2) -> IVec2 {
        IVec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Mul<IVec2> for IVec2 {
    type Output = IVec2;
    fn mul(self, rhs: IVec2) -> IVec2 {
        IVec2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl ops::Mul<isize> for IVec2 {
    type Output = IVec2;
    fn mul(self, rhs: isize) -> IVec2 {
        IVec2::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::Mul<IVec2> for isize {
    type Output = IVec2;
    fn mul(self, rhs: IVec2) -> IVec2 {
        rhs * self
    }
}

impl ops::Div<IVec2> for IVec2 {
    type Output = IVec2;
    fn div(self, rhs: IVec2) -> IVec2 {
        IVec2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl ops::Div<isize> for IVec2 {
    type Output = IVec2;
    fn div(self, rhs: isize) -> IVec2 {
        IVec2::new(self.x / rhs, self.y / rhs)
    }
}

// endregion

// region Vec2
#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn new_xy(xy: f32) -> Vec2 {
        Vec2 { x: xy, y: xy }
    }

    pub fn zero() -> Vec2 {
        Vec2 { x: 0.0, y: 0.0 }
    }

    pub fn to_ivec2(self) -> IVec2 {
        IVec2::new(self.x as isize, self.y as isize)
    }

    pub fn sqr_length(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        self.sqr_length().sqrt()
    }

    pub fn normalized(self) -> Vec2 {
        self / self.length()
    }

    pub fn dot(self, rhs: Vec2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn min(a: Vec2, b: Vec2) -> Vec2 {
        Vec2::new(Math::min(a.x, b.x), Math::min(a.y, b.y))
    }

    pub fn max(a: Vec2, b: Vec2) -> Vec2 {
        Vec2::new(Math::max(a.x, b.x), Math::max(a.y, b.y))
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Mul<Vec2> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::Mul<Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        rhs * self
    }
}

impl ops::Div<Vec2> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}
// endregion

// region Rect

#[derive(Copy, Clone)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    pub fn new(min: Vec2, max: Vec2) -> Rect {
        Rect { min, max }
    }

    pub fn zero() -> Rect {
        Rect {
            min: Vec2::zero(),
            max: Vec2::zero(),
        }
    }

    pub fn is_overlap(&self, other: &Rect) -> bool {
        if self.min.x > other.max.x {
            return false;
        }
        if self.max.x < other.min.x {
            return false;
        }

        if self.min.y > other.max.y {
            return false;
        }
        if self.max.y < other.min.y {
            return false;
        }

        true
    }
}

// endregion

// region Vec3
#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn new_xyz(xyz: f32) -> Vec3 {
        Vec3 {
            x: xyz,
            y: xyz,
            z: xyz,
        }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn sqr_length(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        self.sqr_length().sqrt()
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn random_on_unit_sphere(rand_0: f32, rand_1: f32) -> Vec3 {
        let theta = rand_0 * std::f32::consts::PI * 2.0;
        let phi = f32::acos((2.0 * rand_1) - 1.0);
        let x = f32::sin(phi) * f32::cos(theta);
        let y = f32::sin(phi) * f32::sin(theta);
        let z = f32::cos(phi);

        Vec3::new(x, y, z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3 {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

// endregion
