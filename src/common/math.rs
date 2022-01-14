use std::ops;

pub const EPSILON : f32 = 0.0001;
pub const PI : f32 = 3.14159265359;
pub const INV_PI : f32 = 0.31830988618;

pub struct Math { }

impl Math{
    pub fn small_enought(x : f32) -> bool {
        x < std::f32::EPSILON * 10.0 && x > -std::f32::EPSILON * 10.0
    }
    
    pub fn min(a : f32, b : f32) -> f32 {
        if a < b { a } else { b } 
    }

    pub fn max(a : f32, b : f32) -> f32 {
        if a > b { a } else { b } 
    }

    pub fn min_triple(a : f32, b : f32, c : f32) -> f32{
        Math::min(a, Math::min(b, c))
    }

    pub fn max_triple(a : f32, b : f32, c : f32) -> f32{
        Math::max(a, Math::max(b, c))
    }
}

// region IVec2 
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct IVec2{
    pub x : isize,
    pub y : isize
}

impl IVec2{
    pub fn new(x : isize, y : isize) -> IVec2{
        IVec2 { x, y }
    }

    pub fn zero() -> IVec2{
        IVec2 { x : 0, y : 0 }
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

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn new_xy(xy : f32) -> Vec2 {
        Vec2 { x : xy, y : xy }
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

    pub fn new_xyz(xyz : f32) -> Vec3{
        Vec3 { x : xyz, y : xyz, z : xyz }
    }

    pub fn zero() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
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

    pub fn random_on_unit_sphere(rand_0 : f32, rand_1 : f32) -> Vec3 {
        let theta = rand_0 * PI * 2.0;
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