#[derive(Copy)]
pub struct Circle {
    pub radius: f64,
    pub positionX: f64,
    pub positionY: f64,
}
impl Circle {
    pub fn set_radius(&mut self, r: f64) {
        self.radius = r;
    }
    
    pub fn get_radius(&self) -> f64 {
        return self.radius;
    }
}