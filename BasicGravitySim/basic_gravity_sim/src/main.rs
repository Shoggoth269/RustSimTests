mod vec2;
pub use crate::vec2::Vec2;

mod particle;
pub use particle::Particle;
use particle::Physics;


fn main() {

    {
        println!("\n\nStarting main loop...");

        let mut particle = Particle {
            pos: Vec2 {x: 0.0f32, y: 0.0f32},
            vel: Vec2 {x: 2.5f32, y: 200.0f32},
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
            if  (particle.moving()) && (duration_frame.as_secs() as f64 + duration_frame.subsec_nanos() as f64 * 1e-9) as f32
                >= 0.030f32
            {

                println!("{:.5},\t{}", total_dt, particle);
                last_frame = std::time::SystemTime::now();
            }
        }
    }
}
