pub struct Circle {
    mass: f64;
    positionX: f64;
    positionY: f64;
    velocityX: f64;
    velocityY: f64;
    radius: f64;
}

impl Circle {
    getMass(&self) -> f64{
        return self.mass;
    }

    setMass(&mut self, value: f64) {
        self.mass = value;
    }

    getPositionX(&self) -> f64{
        return self.positionX;
    }

    setPositionX(&mut self, value: f64) {
        self.positionX = value;
    }

    getPositionY(&self) {
        return self.positionY;
    }

    setPositionY(&mut self, value: f64) {
        self.positionY = value;
    }

    getVolecityX(&self) -> f64 {
        return self.velocityX;
    }

    setVelocityX(&mut self, value: f64) {
        self.velocityX = value;
    }

    etVolecityY(&self) -> f64 {
        return self.velocityY;
    }

    setVelocityY(&mut self, value: f64) {
        self.velocityY = value;
    }

    getRadius(&self) -> f64 {
        return self.radius;
    }

    setRadius(&self, value: f64) -> {
        this.radius = value;
    }
}