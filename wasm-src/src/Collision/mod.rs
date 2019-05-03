mod CollisionDetection;
use super::World::Shape;
pub fn getSomething (shape: Shape) -> f64{
    return CollisionDetection::getAABB(shape);
}