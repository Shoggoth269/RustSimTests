struct Vec2 {
    x :f32,
    y :f32,
}

struct Particle {
    pos: Vec2,
    vel: Vec2,
}

trait Physics {
    fn physics(&mut self, time: &f32, a: &f32);
}

impl Physics for Particle {
    fn physics(&mut self, time: &f32, a: &f32)
    {
        if self.pos.y <= 0.0f32 && self.vel.y <= 0.0f32 {
            return;
        }

        // determine new position
        self.pos.y = self.pos.y + (self.vel.y * time) + 0.5f32 * a * (time * time);

        // determine new velocity
        self.vel.y = (a * time) + self.vel.y;

        if self.pos.y < 0.0f32 {
            self.pos.y = 0.0f32;
            self.vel.y = 0.0f32;
        }
    }
}

trait Display {
    fn display(&self);
}

impl Display for Vec2 {
    fn display(&self) {
        println!("{}, {}", self.x, self.y);
    }
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "[{}, {}]", self.x, self.y)
    }
}

impl std::fmt::Display for Particle {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "Pos: [{}, {}],\tVel: [{}, {}]", self.pos.x, self.pos.y, self.vel.x, self.vel.y)
    }
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

fn main() {

    {
        println!("\n\nAddition Tests: ");
        let mut add_test = Vec2 {
            x: 25.0f32,
            y: 12.332f32
        };

        add_test.display();
        add_test = add_test + Vec2{x: 10.0f32, y: 10.0f32};
        add_test.display();
        add_test += Vec2{x: 10.0f32, y: 10.0f32};
        add_test.display();
    }

    {
        println!("\n\nDot Product Tests: ");
        let a = Vec2 {
            x: 5f32,
            y: 10f32,
        };
        let b = Vec2 {
            x: 2f32,
            y: 2f32,
        };
        println!("{} * {} = {}", a, b, &a * &b);
    }


    {
        println!("\n\nScalar Multiplication Tests: ");
        let mut mul_test = Vec2 {
            x: 2.5f32,
            y: 5.0f32,
        };

        mul_test.display();
        mul_test = mul_test * 2.0f32;
        mul_test.display();
        mul_test *= 2.0f32;
        mul_test.display();
    }


    {
        println!("\n\nParticle Tests: ");
        let mut particle = Particle {
            pos: Vec2 {x: 0.0f32, y: 0.0f32},
            vel: Vec2 {x: 0.0f32, y: 0.0f32},
        };
        println!("{}", particle);
    }


    {
        println!("\n\nStarting main loop...");

        let mut particle = Particle {
            pos: Vec2 {x: 0.0f32, y: 0.0f32},
            vel: Vec2 {x: 0.0f32, y: 100.0f32},
        };
        let a :f32 = 9.81f32;

        let mut now = std::time::SystemTime::now(); // used for timing
        let mut last_frame = std::time::SystemTime::now();

        let mut total_dt :f32 = 0.0f32;

        loop {
            let duration = std::time::SystemTime::now().duration_since(now).unwrap();
            let dt: f32 = (duration.as_secs() as f64
                + duration.subsec_nanos() as f64 * 1e-9) as f32;
            total_dt += dt as f32;
            now = std::time::SystemTime::now();

            // do some physics
            particle.physics(&dt, &-a);

            let duration_frame = std::time::SystemTime::now().duration_since(last_frame).unwrap();
            if  (particle.vel.y != 0.0f32 && particle.pos.y != 0.0f32) && (duration_frame.as_secs() as f64 + duration_frame.subsec_nanos() as f64 * 1e-9) as f32
                >= 0.030f32
            {

                println!("{},\t{}", total_dt, particle);
                last_frame = std::time::SystemTime::now();
            }
        }
    }
}
