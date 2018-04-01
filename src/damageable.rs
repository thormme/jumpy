pub trait Damageable {
    fn get_health(&self) -> u32;
    fn set_health(&mut self, u32) -> u32;
}
