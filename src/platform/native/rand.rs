use rand::{thread_rng, Rng};

pub fn rand() -> f32 {
    thread_rng().gen::<f32>()
}
