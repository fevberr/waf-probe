use rand::Rng;

pub fn rand0() -> String {
    format!("{:016x}", rand::thread_rng().gen::<u64>())
}