use crate::geometry::Rotation;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x, y
        }
    }

    pub fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    pub fn l1_norm(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
    
    pub fn rotate(self, rot: Rotation) -> Self {
        match rot {
            Rotation::Clockwise => Self {
                x: self.y,
                y: -self.x,
            },
            Rotation::CounterClockwise => Self {
                x: -self.y,
                y: self.x,
            },
        }
    }
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = format!("({}, {})", self.x, self.y);
        f.pad(&s)
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, other: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl std::ops::Mul<u32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x * (rhs as i32),
            y: self.y * (rhs as i32),
        }
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from(tup: (i32, i32)) -> Self {
        Self {
            x: tup.0,
            y: tup.1,
        }
    }
}

impl From<(u32, u32)> for Vec2 {
    fn from(tup: (u32, u32)) -> Self {
        Self {
            x: tup.0 as i32,
            y: tup.1 as i32,
        }
    }
}