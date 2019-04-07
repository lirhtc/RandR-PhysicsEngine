class Shape {
    constructor(x, y) {
        this.x = x || 1;
        this.y = y || 1;
        this.mass = 0;
        this.velocity_x = 1;
        this.velocity_y = 1;
    }

    get get_mass() {
        return this.mass;
    }

    get get_coordinate_x() {
        return this.x;
    }

    get get_coordinate_y() {
        return this.y;
    }

    get get_velocity_x() {
        return this.velocity_x;
    }

    get get_velocity_y() {
        return this.velocity_y;
    }

    set_mass(value) {
        this.mass = value;
    }

    set_coordinate_x(value) {
        this.x = value;
    }

    set_coordinate_y(value) {
        this.y = value;
    }

    set_velocity_x(value) {
        this.velocity_x = value;
    }

    set_velocity_y(value) {
        this.velocity_y = value;
    }

    set_renderable(obj) {
        this.renderable = obj;
    }
}

class Circle extends Shape {
    constructor(radius) {
        super(0, 0);
        this.radius = radius;
        this.mass = 1;
        this.velocity_x = 0;
        this.velocity_y = 0;
        this.renderable = this._get_renderable(radius);
    }

    get get_coordinate_x() {
        return this.renderable.x;
    }

    get get_coordinate_y() {
        return this.renderable.y;
    }

    set_coordinate_x(value) {
        this.renderable.x = value;
    }

    set_coordinate_y(value) {
        this.renderable.y = value;
    }

    _get_renderable(radius) {
        let circle = new PIXI.Graphics();
        circle.beginFill(0xFFCC00, 1);
        circle.drawCircle(radius, radius, radius);
        let sprite = new PIXI.Sprite();
        sprite.addChild(circle);
        return sprite;
    }

    add_to_stage(stage) {
        stage.addChild(this.renderable);
    }
}