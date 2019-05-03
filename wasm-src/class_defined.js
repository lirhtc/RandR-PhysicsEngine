export class Shape {
    constructor() {
        this.mass = 1
        this.x = 1;
        this.y = 1;
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

    set set_mass(value) {
        this.mass = value;
    }

    set set_coordinate_x(value) {
        this.x = value;
    }

    set set_coordinate_y(value) {
        this.y = value;
    }
}