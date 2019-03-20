use crate::Vec2;

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
}

pub trait Physics {
    fn physics(&mut self, time: &f32, a: &f32);
    fn moving(&self) -> bool;
}

impl Physics for Particle {
    fn physics(&mut self, time: &f32, a: &f32)
    {
        if self.pos.y <= 0.0f32 && self.vel.y <= 0.0f32 {
            return;
        }

        // determine new position
        self.pos.y = self.pos.y + (self.vel.y * time) + 0.5f32 * a * (time * time);
        self.pos.x = self.pos.x + (self.vel.x * time);

        // determine new velocity
        self.vel.y = (a * time) + self.vel.y;

        if self.pos.y < 0.0f32 {
            self.pos.y = 0.0f32;
            self.vel.y = 0.0f32;
            self.vel.x = 0.0f32; //we landed in sand :)
        }
    }

    fn moving(&self) -> bool
    {
        return (self.vel.x != 0.0f32) || (self.vel.y != 0.0f32 && self.pos.y != 0.0f32)
    }
}

impl std::fmt::Display for Particle {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
//        write!(fmt, "Pos: [{}, {}],\tVel: [{}, {}]", self.pos.x, self.pos.y, self.vel.x, self.vel.y)
        write!(fmt, "Pos: {},\tVel: {}", self.pos, self.vel)
    }
}



//{
//println!("\n\nParticle Tests: ");
//let mut particle = Particle {
//pos: Vec2 {x: 0.0f32, y: 0.0f32},
//vel: Vec2 {x: 0.0f32, y: 0.0f32},
//};
//println!("{}", particle);
//}