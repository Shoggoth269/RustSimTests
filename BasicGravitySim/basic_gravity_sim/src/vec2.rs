pub struct Vec2 {
    pub x :f32,
    pub y :f32,
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Vec2 {
        Vec2 {x: self.x + _rhs.x, y: self.y + _rhs.y }
    }
}

impl std::ops::Add<f32> for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: f32) -> Vec2 {
        Vec2 {x: self.x + _rhs, y: self.y + _rhs}
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, _rhs: Vec2) {
        *self = Vec2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        };
    }
}

impl std::ops::AddAssign<f32> for Vec2 {
    fn add_assign(&mut self, _rhs: f32) {
        *self = Vec2 {
            x: self.x + _rhs,
            y: self.y + _rhs,
        }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, _rhs: f32) -> Vec2 {
        Vec2 {x: self.x * _rhs, y: self.y * _rhs}
    }
}

impl std::ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, _rhs: f32) {
        *self = Vec2 {
            x: self.x * _rhs,
            y: self.y * _rhs,
        }
    }
}

impl std::ops::Mul for &Vec2 {
    type Output = f32;

    fn mul(self, _rhs: &Vec2) -> f32 {
        (self.x * _rhs.x) + (self.y * _rhs.y)
    }
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "[{:.5}, {:.5}]", self.x, self.y)
    }
}





//
//{
//println!("\n\nAddition Tests: ");
//let mut add_test = Vec2 {
//x: 25.0f32,
//y: 12.332f32
//};
//
//println!("{}", add_test);
//add_test = add_test + Vec2{x: 10.0f32, y: 10.0f32};
//println!("{}", add_test);
//add_test += Vec2{x: 10.0f32, y: 10.0f32};
//println!("{}", add_test);
//}
//
//{
//println!("\n\nDot Product Tests: ");
//let a = Vec2 {
//x: 5f32,
//y: 10f32,
//};
//let b = Vec2 {
//x: 2f32,
//y: 2f32,
//};
//println!("{} * {} = {}", a, b, &a * &b);
//}
//
//
//{
//println!("\n\nScalar Multiplication Tests: ");
//let mut mul_test = Vec2 {
//x: 2.5f32,
//y: 5.0f32,
//};
//
//println!("{}", mul_test);
//mul_test = mul_test * 2.0f32;
//println!("{}", mul_test);
//mul_test *= 2.0f32;
//println!("{}", mul_test);
//}